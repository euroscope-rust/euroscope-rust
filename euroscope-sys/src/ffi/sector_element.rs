//! `CSectorElement` FFI declarations.

use std::ffi::{c_char, c_int};

use crate::{EsHandle, PluginPtr};

unsafe extern "C" {
    /// Whether the sector-element reference is valid (`IsValid`).
    pub fn es_sectorelement_is_valid(h: EsHandle) -> bool;

    /// The element type code (`GetElementType`).
    pub fn es_sectorelement_element_type(h: EsHandle) -> c_int;

    /// The element name (`GetName`). Borrowed, NUL-terminated, ANSI.
    pub fn es_sectorelement_name(h: EsHandle) -> *const c_char;

    /// The position at `index` (`GetPosition`). Writes lat/lon out-params and
    /// returns whether the index is valid.
    pub fn es_sectorelement_position(
        h: EsHandle,
        index: c_int,
        lat: *mut f64,
        lon: *mut f64,
    ) -> bool;

    /// Name of the switchable component at `index` (`GetComponentName`).
    /// Borrowed, NUL-terminated, ANSI.
    pub fn es_sectorelement_component_name(h: EsHandle, index: c_int) -> *const c_char;

    /// The frequency of VOR/NDB/AIRPORT elements, else 0.0 (`GetFrequency`).
    pub fn es_sectorelement_frequency(h: EsHandle) -> f64;

    /// Name of the runway at `index` (0 or 1) (`GetRunwayName`). Borrowed,
    /// NUL-terminated, ANSI.
    pub fn es_sectorelement_runway_name(h: EsHandle, index: c_int) -> *const c_char;

    /// Heading of the runway at `index` (0 or 1), or -1 (`GetRunwayHeading`).
    pub fn es_sectorelement_runway_heading(h: EsHandle, index: c_int) -> c_int;

    /// Name of the airport this element belongs to (`GetAirportName`).
    /// Borrowed, NUL-terminated, ANSI.
    pub fn es_sectorelement_airport_name(h: EsHandle) -> *const c_char;

    /// Whether the element is active for departure (true) or arrival (false)
    /// at runway `index` (`IsElementActive`).
    pub fn es_sectorelement_is_element_active(h: EsHandle, departure: bool, index: c_int) -> bool;

    /// Free a sector-element handle allocated by a `*sectorelement_select*` call.
    pub fn es_sectorelement_free(el: EsHandle);

    /// First sector element of `element_type`, or null.
    /// Wraps `CPlugIn::SectorFileElementSelectFirst`.
    pub fn es_plugin_sectorelement_select_first(plugin: PluginPtr, element_type: c_int)
    -> EsHandle;

    /// Sector element of `element_type` after `current`, or null.
    /// Wraps `CPlugIn::SectorFileElementSelectNext`.
    pub fn es_plugin_sectorelement_select_next(
        plugin: PluginPtr,
        current: EsHandle,
        element_type: c_int,
    ) -> EsHandle;
}
