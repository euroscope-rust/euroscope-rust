//! Opinionated conveniences for [`euroscope`] plugins.
//!
//! Everything here is layered on the safe `euroscope` API and is optional —
//! each module sits behind its own feature, both on by default:
//!
//! - `command` — dot-command parsing and dispatch via `argh`.
//! - `settings` — plugin configuration.

#[cfg(feature = "command")]
pub mod command;

#[cfg(feature = "settings")]
pub mod settings;
