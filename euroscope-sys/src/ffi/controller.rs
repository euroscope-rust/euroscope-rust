//! `CController` FFI declarations.

use std::ffi::{c_char, c_int};

use crate::{EsHandle, PluginPtr};

unsafe extern "C" {
    /// Whether the controller reference is valid. Wraps `CController::IsValid`.
    pub fn es_controller_is_valid(h: EsHandle) -> bool;

    /// Callsign of the controller. Borrowed, NUL-terminated, ANSI.
    /// Wraps `CController::GetCallsign`.
    pub fn es_controller_callsign(h: EsHandle) -> *const c_char;

    /// Position ID of the selected controller. Borrowed, NUL-terminated, ANSI.
    /// Wraps `CController::GetPositionId`.
    pub fn es_controller_position_id(h: EsHandle) -> *const c_char;

    /// Whether the controller is identified in the position file.
    /// Wraps `CController::GetPositionIdentified`.
    pub fn es_controller_position_identified(h: EsHandle) -> bool;

    /// Primary frequency of the controller (MHz), `199.980` if none.
    /// Wraps `CController::GetPrimaryFrequency`.
    pub fn es_controller_primary_frequency(h: EsHandle) -> f64;

    /// Full name of the controller. Borrowed, NUL-terminated, ANSI.
    /// Wraps `CController::GetFullName`.
    pub fn es_controller_full_name(h: EsHandle) -> *const c_char;

    /// Network rating id of the controller. Wraps `CController::GetRating`.
    pub fn es_controller_rating(h: EsHandle) -> c_int;

    /// Facility id of the controller. Wraps `CController::GetFacility`.
    pub fn es_controller_facility(h: EsHandle) -> c_int;

    /// Name of the sector file used by the controller. Borrowed,
    /// NUL-terminated, ANSI. Wraps `CController::GetSectorFileName`.
    pub fn es_controller_sector_file_name(h: EsHandle) -> *const c_char;

    /// Whether the controller is accepted as a controller by the servers.
    /// Wraps `CController::IsController`.
    pub fn es_controller_is_controller(h: EsHandle) -> bool;

    /// Center position of the controller, written to the `lat`/`lon`
    /// out-params. Wraps `CController::GetPosition`.
    pub fn es_controller_position(h: EsHandle, lat: *mut f64, lon: *mut f64);

    /// Visibility range of the controller. Wraps `CController::GetRange`.
    pub fn es_controller_range(h: EsHandle) -> c_int;

    /// Breaking state of the controller. Wraps `CController::IsBreaking`.
    pub fn es_controller_is_breaking(h: EsHandle) -> bool;

    /// Whether the controller is ready for ongoing coordination.
    /// Wraps `CController::IsOngoingAble`.
    pub fn es_controller_is_ongoing_able(h: EsHandle) -> bool;

    /// Free a controller handle allocated by a `*controller_select*` call.
    pub fn es_controller_free(c: EsHandle);

    /// First controller in the session, or null.
    /// Wraps `CPlugIn::ControllerSelectFirst`.
    pub fn es_plugin_controller_select_first(plugin: PluginPtr) -> EsHandle;

    /// Controller after `current`, or null. Wraps `CPlugIn::ControllerSelectNext`.
    pub fn es_plugin_controller_select_next(plugin: PluginPtr, current: EsHandle) -> EsHandle;

    /// Select a controller by callsign, or null. Wraps `CPlugIn::ControllerSelect`.
    pub fn es_plugin_controller_select(plugin: PluginPtr, callsign: *const c_char) -> EsHandle;

    /// Select a controller by position ID, or null.
    /// Wraps `CPlugIn::ControllerSelectByPositionId`.
    pub fn es_plugin_controller_select_by_position_id(
        plugin: PluginPtr,
        position_id: *const c_char,
    ) -> EsHandle;

    /// The logged-in controller ("myself"), or null. Wraps `CPlugIn::ControllerMyself`.
    pub fn es_plugin_controller_myself(plugin: PluginPtr) -> EsHandle;
}
