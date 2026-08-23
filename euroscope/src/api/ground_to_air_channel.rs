//! `CGrountToAirChannel` — a reference to a ground-to-air communication channel.

use std::marker::PhantomData;

use euroscope_sys::EsHandle;

use crate::{
    Context,
    api::handle::{iter, scoped},
    utils::cstr,
};

/// A reference to a ground-to-air communication channel (frequency).
///
/// A thin handle over EuroScope's `CGrountToAirChannel`, valid only for the
/// duration of the callback / query block that produced it (`'cb`).
#[derive(Clone, Copy)]
pub struct GroundToAirChannel<'cb> {
    raw: EsHandle,
    _marker: PhantomData<&'cb ()>,
}

impl GroundToAirChannel<'_> {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw `CGrountToAirChannel*`. Escape hatch for `euroscope-sys` calls
    /// not yet wrapped here.
    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    /// Whether this handle references a valid channel.
    pub fn is_valid(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_is_valid(self.raw) }
    }

    /// The name of the communication channel. Borrows EuroScope-owned memory;
    /// valid for this callback only.
    pub fn name(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_gtachannel_name(self.raw)) }
    }

    /// The frequency of the communication channel, in MHz.
    pub fn frequency(&self) -> f64 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_frequency(self.raw) }
    }

    /// The voice server name of the channel. Borrows EuroScope-owned memory;
    /// valid for this callback only.
    pub fn voice_server(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_gtachannel_voice_server(self.raw)) }
    }

    /// The voice server channel name. Borrows EuroScope-owned memory; valid for
    /// this callback only.
    pub fn voice_channel(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_gtachannel_voice_channel(self.raw)) }
    }

    /// Whether this channel is the primary channel.
    pub fn is_primary(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_is_primary(self.raw) }
    }

    /// Whether this channel is the ATIS channel.
    pub fn is_atis(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_is_atis(self.raw) }
    }

    /// Whether the channel is receiving text messages.
    pub fn is_text_receive_on(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_is_text_receive_on(self.raw) }
    }

    /// Whether the channel is transmitting text messages.
    pub fn is_text_transmit_on(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_is_text_transmit_on(self.raw) }
    }

    /// Whether the channel's voice-receive checkbox is set. Note this alone does
    /// not mean the voice server connection is on.
    pub fn is_voice_receive_on(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_is_voice_receive_on(self.raw) }
    }

    /// Whether the channel is transmitting voice.
    pub fn is_voice_transmit_on(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_is_voice_transmit_on(self.raw) }
    }

    /// Whether the channel is successfully connected to the voice server.
    pub fn is_voice_connected(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_is_voice_connected(self.raw) }
    }

    /// Toggle the primary setting of the channel (like clicking its check box).
    /// Mutates EuroScope state.
    pub fn toggle_primary(&self) {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_toggle_primary(self.raw) }
    }

    /// Toggle the ATIS setting of the channel (like clicking its check box).
    /// Mutates EuroScope state.
    pub fn toggle_atis(&self) {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_toggle_atis(self.raw) }
    }

    /// Toggle the text-receive setting of the channel (like clicking its check
    /// box). Mutates EuroScope state.
    pub fn toggle_text_receive(&self) {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_toggle_text_receive(self.raw) }
    }

    /// Toggle the text-transmit setting of the channel (like clicking its check
    /// box). Mutates EuroScope state.
    pub fn toggle_text_transmit(&self) {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_toggle_text_transmit(self.raw) }
    }

    /// Toggle the voice-receive setting of the channel (like clicking its check
    /// box). Mutates EuroScope state.
    pub fn toggle_voice_receive(&self) {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_toggle_voice_receive(self.raw) }
    }

    /// Toggle the voice-transmit setting of the channel (like clicking its check
    /// box). Mutates EuroScope state.
    pub fn toggle_voice_transmit(&self) {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_gtachannel_toggle_voice_transmit(self.raw) }
    }
}

scoped!(
    GroundToAirChannelHandle,
    GroundToAirChannel,
    es_gtachannel_free
);
iter!(
    GroundToAirChannels,
    GroundToAirChannelHandle,
    GroundToAirChannel,
    es_gtachannel_free,
    es_plugin_gtachannel_select_first,
    es_plugin_gtachannel_select_next
);

/// Ground-to-air channel iteration.
impl Context {
    /// Iterate over all voice ground-to-air channels.
    pub fn ground_to_air_channels(&self) -> GroundToAirChannels<'_> {
        // SAFETY: `Context` holds a live CPlugIn* for the plugin's lifetime.
        GroundToAirChannels::new(unsafe { self.as_ptr() })
    }
}
