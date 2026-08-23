//! `tracing` integration: a [`Layer`] that routes log events to EuroScope's
//! message box.
//!
//! This module provides only the part that is specific to EuroScope — writing
//! to the message box under its context/threading constraints. File logging,
//! filtering, and formatting for other sinks are standard `tracing` concerns:
//! compose [`MboxLayer`] with your own layers (e.g. a [`tracing-appender`] file
//! writer located via [`get_plugin_path`](crate::get_plugin_path)) and install
//! the result yourself:
//!
//! ```ignore
//! use euroscope::tracing::{MboxLayer, is_mbox_target};
//! use tracing_subscriber::{Registry, filter::filter_fn, fmt, layer::SubscriberExt, util::SubscriberInitExt};
//!
//! let dir = euroscope::get_plugin_path().unwrap().parent().unwrap().join("logs");
//! let file = tracing_appender::rolling::daily(dir, format!("{}.log", MyPlugin::NAME));
//!
//! Registry::default()
//!     .with(level)                                                    // your level, from config
//!     .with(fmt::layer().json().with_writer(file))                    // your file, your format
//!     .with(MboxLayer::new().with_filter(filter_fn(is_mbox_target)))  // our message box
//!     .try_init()
//!     .ok();
//! ```
//!
//! Events with `target = "mbox"` (see [`MBOX_TARGET`] / [`is_mbox_target`]) then
//! reach the message box; route as much or as little there as you like — the
//! layer applies no filter of its own.
//!
//! ## How the message box is reached
//!
//! A [`Context`] (needed to write the box) only exists during a callback, on
//! EuroScope's main thread, and is not `Send`. So [`MboxLayer`] is a pure
//! producer: it renders each event and pushes the line into an `mpsc` channel.
//! The `register_plugin!` dispatcher drains that channel — via `drain` — right
//! after `Plugin::new` and after every `on_timer`, where it *does* hold the
//! context. Events from background threads therefore surface at the next timer
//! tick; events from the main thread surface at the end of the current
//! callback.
//!
//! The channel is a process-global sink, created once by the first
//! [`MboxWriter`] and shared by any others, so stacking layers all feed the
//! same channel (rather than replacing and orphaning it) and a reload keeps
//! working even though `tracing`'s global subscriber can only be installed
//! once.
//!
//! [`tracing-appender`]: https://docs.rs/tracing-appender

use std::{
    io::{self, Write},
    sync::mpsc::{self, Receiver, Sender},
};

use ::tracing::{
    Event, Metadata, Subscriber,
    field::{Field, Visit},
};
use parking_lot::Mutex;
use tracing_subscriber::{
    Layer,
    fmt::{
        self, FmtContext, MakeWriter,
        format::{FormatEvent, FormatFields, Writer},
    },
    registry::LookupSpan,
};

use crate::Context;

/// The `target` an event must carry to be routed to EuroScope's message box,
/// e.g. `tracing::info!(target: euroscope::tracing::MBOX_TARGET, "hi")`.
pub const MBOX_TARGET: &str = "mbox";

/// Whether an event's metadata targets the message box (`target == "mbox"`).
/// Handy as a `filter_fn` predicate when composing your subscriber.
#[must_use]
#[inline]
pub fn is_mbox_target(meta: &Metadata<'_>) -> bool {
    meta.target() == MBOX_TARGET
}

// The message-box channel, created lazily by the first `MboxWriter::new` and
// shared by every writer thereafter. The layer(s) (globally installed,
// `'static`) send into `SINK`; the main-thread dispatcher drains `DRAIN`.
// Creating more writers/layers reuses this channel rather than replacing it, so
// stacking layers cannot orphan a receiver, and a reload keeps the
// still-installed subscriber working (the statics are re-seeded if the DLL is
// actually unloaded, and reused as-is if it is not).
static SINK: Mutex<Option<Sender<String>>> = Mutex::new(None);
static DRAIN: Mutex<Option<Receiver<String>>> = Mutex::new(None);

/// Drain every pending message-box line and write it to EuroScope, using
/// `handler` (the plugin name) as the message's handler column.
///
/// Called by the `register_plugin!` dispatcher on the main thread; a no-op if
/// no [`MboxLayer`] is active. The channel is emptied into a local buffer
/// before writing, so the lock is not held across the FFI calls.
pub(crate) fn drain(ctx: &Context, handler: &str) {
    let lines: Vec<String> = {
        let guard = DRAIN.lock();
        match guard.as_ref() {
            Some(rx) => rx.try_iter().collect(),
            None => return,
        }
    };
    for line in lines {
        ctx.display_message(handler, "", &line);
    }
}

/// Tear down the message-box channel, releasing the sender and receiver.
///
/// Called by the `register_plugin!` dispatcher on the plugin's destroy path,
/// after a final `drain`, so a subsequent load starts from a clean channel.
/// Any event emitted afterwards (e.g. from a background thread) is dropped until
/// a new [`MboxWriter`] is created.
pub(crate) fn cleanup() {
    let mut sink = SINK.lock();
    *sink = None;
    *DRAIN.lock() = None;
    drop(sink);
}

/// The message-box event format: `<message> [<key>=<value> …]`.
///
/// No level, timestamp or target — the box is terse. Deliberately not
/// configurable; supply your own [`FormatEvent`] to a `fmt` layer over
/// [`MboxWriter`] if you want a different shape.
#[derive(Debug, Clone, Copy, Default)]
pub struct MboxFormat;

#[derive(Default)]
struct MboxVisitor {
    message: String,
    fields: String,
}

impl MboxVisitor {
    fn separator(&mut self) {
        if !self.fields.is_empty() {
            self.fields.push(' ');
        }
    }
}

#[expect(
    clippy::missing_trait_methods,
    reason = "the other record_* defaults forward to record_debug, which we implement"
)]
impl Visit for MboxVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        use std::fmt::Write as _;
        if field.name() == "message" {
            value.clone_into(&mut self.message);
        } else {
            self.separator();
            let _ = write!(self.fields, "{}={value}", field.name());
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        use std::fmt::Write as _;
        if field.name() == "message" {
            self.message.clear();
            let _ = write!(self.message, "{value:?}");
        } else {
            self.separator();
            let _ = write!(self.fields, "{}={value:?}", field.name());
        }
    }
}

impl<S, N> FormatEvent<S, N> for MboxFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let mut visitor = MboxVisitor::default();
        event.record(&mut visitor);
        write!(writer, "{}", visitor.message)?;
        if !visitor.fields.is_empty() {
            write!(writer, " [{}]", visitor.fields)?;
        }
        // No trailing newline: each event is one distinct message-box line.
        Ok(())
    }
}

/// A [`MakeWriter`] that renders each event into a `String` and pushes it onto
/// the message-box channel. Constructing one (re)installs the channel.
#[derive(Debug, Clone, Copy)]
pub struct MboxWriter;

impl MboxWriter {
    /// Create the writer, ensuring the shared message-box channel exists.
    ///
    /// The channel is created once and reused: constructing several writers (or
    /// several [`MboxLayer`]s) makes them all feed the same channel instead of
    /// replacing it. Call once per plugin load.
    #[must_use]
    pub fn new() -> Self {
        let mut sink = SINK.lock();
        if sink.is_none() {
            let (tx, rx) = mpsc::channel();
            *sink = Some(tx);
            *DRAIN.lock() = Some(rx);
            drop(sink);
        }
        Self
    }
}

impl Default for MboxWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[expect(
    clippy::missing_trait_methods,
    reason = "make_writer_for's default (delegating to make_writer) is correct here"
)]
impl<'a> MakeWriter<'a> for MboxWriter {
    type Writer = MboxLine;

    fn make_writer(&'a self) -> Self::Writer {
        MboxLine { buf: Vec::new() }
    }
}

/// One event's worth of bytes; sends the assembled line to the channel on drop
/// (a `fmt` layer makes one writer per event and drops it once written).
#[doc(hidden)]
pub struct MboxLine {
    buf: Vec<u8>,
}

#[expect(
    clippy::missing_trait_methods,
    reason = "the write_* defaults over our write/flush are correct"
)]
impl Write for MboxLine {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[expect(clippy::missing_trait_methods, reason = "we do not need pin_drop")]
impl Drop for MboxLine {
    fn drop(&mut self) {
        if self.buf.is_empty() {
            return;
        }
        let line = String::from_utf8_lossy(&self.buf).into_owned();
        if let Some(tx) = SINK.lock().as_ref() {
            let _ = tx.send(line);
        }
    }
}

/// The message-box [`Layer`]: renders with [`MboxFormat`] and feeds the channel
/// that `drain` empties into EuroScope.
///
/// It applies no filter of its own — filter it yourself (e.g.
/// `.with_filter(filter_fn(is_mbox_target))`) to control which events reach the
/// box.
#[derive(Debug, Clone, Copy)]
pub struct MboxLayer;

impl MboxLayer {
    /// Build the message-box layer, ensuring the shared channel exists. Call
    /// once per plugin load.
    #[must_use]
    #[expect(
        clippy::new_ret_no_self,
        reason = "MboxLayer namespaces the constructor; the layer is an opaque fmt::Layer"
    )]
    pub fn new<S>() -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        fmt::layer()
            .event_format(MboxFormat)
            .with_ansi(false)
            .with_writer(MboxWriter::new())
    }
}
