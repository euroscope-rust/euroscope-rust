//! `CRadarTarget` FFI declarations.

use std::ffi::{c_char, c_int};

use crate::{EsHandle, FlightPlanPtr, PluginPtr};

unsafe extern "C" {
    /// Whether the radar-target handle references a live aircraft.
    /// Wraps `CRadarTarget::IsValid`.
    pub fn es_radartarget_is_valid(h: EsHandle) -> bool;

    /// Aircraft callsign. Borrowed, NUL-terminated, ANSI.
    /// Wraps `CRadarTarget::GetCallsign`.
    pub fn es_radartarget_callsign(h: EsHandle) -> *const c_char;

    /// System-assigned target ID (stable across instances, derived from the
    /// callsign). Borrowed, NUL-terminated, ANSI.
    /// Wraps `CRadarTarget::GetSystemID`.
    pub fn es_radartarget_system_id(h: EsHandle) -> *const c_char;

    /// Calculated vertical speed in feet/minute (protocol-inaccurate).
    /// Wraps `CRadarTarget::GetVerticalSpeed`.
    pub fn es_radartarget_vertical_speed(h: EsHandle) -> c_int;

    /// Calculated track direction in degrees.
    /// Wraps `CRadarTarget::GetTrackHeading`.
    pub fn es_radartarget_track_heading(h: EsHandle) -> f64;

    /// Ground speed in knots (reported, falling back to calculated).
    /// Wraps `CRadarTarget::GetGS`.
    pub fn es_radartarget_gs(h: EsHandle) -> c_int;

    /// Correlate this radar target with a flight plan; returns whether it
    /// succeeded. Mutates EuroScope state.
    /// Wraps `CRadarTarget::CorrelateWithFlightPlan`.
    pub fn es_radartarget_correlate_with_flight_plan(h: EsHandle, fp: FlightPlanPtr) -> bool;

    /// Uncorrelate this radar target from its flight plan. Mutates EuroScope
    /// state.
    /// Wraps `CRadarTarget::Uncorrelate`.
    pub fn es_radartarget_uncorrelate(h: EsHandle);

    /// Free a radar-target handle allocated by a `*radartarget_select*` /
    /// `*_correlated_*` call.
    pub fn es_radartarget_free(rt: EsHandle);

    /// First radar target in the session, or null.
    /// Wraps `CPlugIn::RadarTargetSelectFirst`.
    pub fn es_plugin_radartarget_select_first(plugin: PluginPtr) -> EsHandle;

    /// Radar target after `current`, or null. Wraps `CPlugIn::RadarTargetSelectNext`.
    pub fn es_plugin_radartarget_select_next(plugin: PluginPtr, current: EsHandle) -> EsHandle;

    /// The ASEL (selected) radar target, or null. Wraps `CPlugIn::RadarTargetSelectASEL`.
    pub fn es_plugin_radartarget_select_asel(plugin: PluginPtr) -> EsHandle;

    /// Select a radar target by callsign, or null. Wraps `CPlugIn::RadarTargetSelect`.
    pub fn es_plugin_radartarget_select(plugin: PluginPtr, callsign: *const c_char) -> EsHandle;

    /// The flight plan correlated with `rt`, or null (owned; free with
    /// `es_flightplan_free`). Wraps `CRadarTarget::GetCorrelatedFlightPlan`.
    pub fn es_radartarget_correlated_flight_plan(rt: EsHandle) -> EsHandle;

    /// Owned latest position snapshot (free with `es_rtposdata_free`).
    /// Wraps `CRadarTarget::GetPosition`.
    pub fn es_radartarget_current_position(rt: EsHandle) -> EsHandle;

    /// Owned snapshot before `current` (free with `es_rtposdata_free`).
    /// Wraps `CRadarTarget::GetPreviousPosition`.
    pub fn es_radartarget_previous_position(rt: EsHandle, current: EsHandle) -> EsHandle;

    /// Set the ASEL (selected) aircraft to `rt`. Wraps `CPlugIn::SetASELAircraft`.
    pub fn es_plugin_set_asel_radartarget(plugin: PluginPtr, rt: EsHandle);
}
