//! `CGrountToAirChannel` FFI declarations.

use std::ffi::c_char;

use crate::{EsHandle, PluginPtr};

unsafe extern "C" {
    /// Whether the channel handle is valid (`CGrountToAirChannel::IsValid`).
    pub fn es_gtachannel_is_valid(h: EsHandle) -> bool;

    /// Name of the communication channel (`GetName`). Borrowed, NUL-terminated.
    pub fn es_gtachannel_name(h: EsHandle) -> *const c_char;

    /// Frequency of the communication channel (`GetFrequency`).
    pub fn es_gtachannel_frequency(h: EsHandle) -> f64;

    /// Voice server name of the channel (`GetVoiceServer`). Borrowed, NUL-terminated.
    pub fn es_gtachannel_voice_server(h: EsHandle) -> *const c_char;

    /// Voice server channel name (`GetVoiceChannel`). Borrowed, NUL-terminated.
    pub fn es_gtachannel_voice_channel(h: EsHandle) -> *const c_char;

    /// Whether this is the primary channel (`GetIsPrimary`).
    pub fn es_gtachannel_is_primary(h: EsHandle) -> bool;

    /// Whether this is the ATIS channel (`GetIsAtis`).
    pub fn es_gtachannel_is_atis(h: EsHandle) -> bool;

    /// Whether the channel is receiving text messages (`GetIsTextReceiveOn`).
    pub fn es_gtachannel_is_text_receive_on(h: EsHandle) -> bool;

    /// Whether the channel is transmitting text messages (`GetIsTextTransmitOn`).
    pub fn es_gtachannel_is_text_transmit_on(h: EsHandle) -> bool;

    /// Whether the channel's voice-receive checkbox is set (`GetIsVoiceReceiveOn`).
    pub fn es_gtachannel_is_voice_receive_on(h: EsHandle) -> bool;

    /// Whether the channel is transmitting voice (`GetIsVoiceTransmitOn`).
    pub fn es_gtachannel_is_voice_transmit_on(h: EsHandle) -> bool;

    /// Whether the channel is connected to the voice server (`GetIsVoiceConnected`).
    pub fn es_gtachannel_is_voice_connected(h: EsHandle) -> bool;

    /// Toggle the primary setting of the channel (`TogglePrimary`).
    pub fn es_gtachannel_toggle_primary(h: EsHandle);

    /// Toggle the ATIS setting of the channel (`ToggleAtis`).
    pub fn es_gtachannel_toggle_atis(h: EsHandle);

    /// Toggle the text-receive setting of the channel (`ToggleTextReceive`).
    pub fn es_gtachannel_toggle_text_receive(h: EsHandle);

    /// Toggle the text-transmit setting of the channel (`ToggleTextTransmit`).
    pub fn es_gtachannel_toggle_text_transmit(h: EsHandle);

    /// Toggle the voice-receive setting of the channel (`ToggleVoiceReceive`).
    pub fn es_gtachannel_toggle_voice_receive(h: EsHandle);

    /// Toggle the voice-transmit setting of the channel (`ToggleVoiceTransmit`).
    pub fn es_gtachannel_toggle_voice_transmit(h: EsHandle);

    /// Free a channel handle allocated by a `*gtachannel_select*` call.
    pub fn es_gtachannel_free(ch: EsHandle);

    /// First ground-to-air channel, or null.
    /// Wraps `CPlugIn::GroundToArChannelSelectFirst`.
    pub fn es_plugin_gtachannel_select_first(plugin: PluginPtr) -> EsHandle;

    /// Channel after `current`, or null. Wraps `CPlugIn::GroundToArChannelSelectNext`.
    pub fn es_plugin_gtachannel_select_next(plugin: PluginPtr, current: EsHandle) -> EsHandle;
}
