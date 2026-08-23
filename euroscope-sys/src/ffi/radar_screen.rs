//! `CRadarScreen` method declarations (Rust -> EuroScope). The `screen` handle
//! is the `CRadarScreen*` delivered to a radar-screen callback.

use std::ffi::{c_char, c_int};

use crate::EsHandle;

unsafe extern "C" {
    /// The owning `CPlugIn*` (`CRadarScreen::GetPlugIn`).
    pub fn es_screen_get_plugin(screen: EsHandle) -> EsHandle;

    /// Radar drawing area, as `left/top/right/bottom` out-params.
    pub fn es_screen_radar_area(
        screen: EsHandle,
        l: *mut c_int,
        t: *mut c_int,
        r: *mut c_int,
        b: *mut c_int,
    );
    /// Toolbar area.
    pub fn es_screen_toolbar_area(
        screen: EsHandle,
        l: *mut c_int,
        t: *mut c_int,
        r: *mut c_int,
        b: *mut c_int,
    );
    /// Chat area.
    pub fn es_screen_chat_area(
        screen: EsHandle,
        l: *mut c_int,
        t: *mut c_int,
        r: *mut c_int,
        b: *mut c_int,
    );

    /// Convert a lat/lon to screen pixels.
    pub fn es_screen_position_to_pixel(
        screen: EsHandle,
        lat: f64,
        lon: f64,
        x: *mut c_int,
        y: *mut c_int,
    );
    /// Convert screen pixels to a lat/lon.
    pub fn es_screen_pixel_to_position(
        screen: EsHandle,
        x: c_int,
        y: c_int,
        lat: *mut f64,
        lon: *mut f64,
    );

    /// Register a clickable/hover screen object (`CRadarScreen::AddScreenObject`).
    pub fn es_screen_add_screen_object(
        screen: EsHandle,
        object_type: c_int,
        object_id: *const c_char,
        l: c_int,
        t: c_int,
        r: c_int,
        b: c_int,
        moveable: bool,
        message: *const c_char,
    );

    /// Request an asynchronous repaint.
    pub fn es_screen_request_refresh(screen: EsHandle);
    /// Rebuild cached map content.
    pub fn es_screen_refresh_map_content(screen: EsHandle);

    /// Read an ASR variable (borrowed NUL-terminated string).
    pub fn es_screen_get_data_from_asr(screen: EsHandle, name: *const c_char) -> *const c_char;
    /// Persist an ASR variable.
    pub fn es_screen_save_data_to_asr(
        screen: EsHandle,
        name: *const c_char,
        description: *const c_char,
        value: *const c_char,
    );

    /// Current display bounds as two corners' lat/lon (out-params).
    pub fn es_screen_get_display_area(
        screen: EsHandle,
        ld_lat: *mut f64,
        ld_lon: *mut f64,
        ru_lat: *mut f64,
        ru_lon: *mut f64,
    );
    /// Set the display bounds from two corners' lat/lon.
    pub fn es_screen_set_display_area(
        screen: EsHandle,
        ld_lat: f64,
        ld_lon: f64,
        ru_lat: f64,
        ru_lon: f64,
    );

    /// Show/hide a sector-file element on this screen. `element` is a
    /// `CSectorElement*`.
    pub fn es_screen_show_sector_element(
        screen: EsHandle,
        element: EsHandle,
        component: *const c_char,
        show: bool,
    );

    /// Trigger another plugin's tag function on a target.
    #[allow(clippy::too_many_arguments)]
    pub fn es_screen_start_tag_function(
        screen: EsHandle,
        callsign: *const c_char,
        item_plugin: *const c_char,
        item_code: c_int,
        item_string: *const c_char,
        function_plugin: *const c_char,
        function_id: c_int,
        x: c_int,
        y: c_int,
        l: c_int,
        t: c_int,
        r: c_int,
        b: c_int,
    );
}
