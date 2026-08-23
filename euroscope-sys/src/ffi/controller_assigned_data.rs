//! `CFlightPlanControllerAssignedData` FFI declarations.
//!
//! A sub-object of `CFlightPlan`; every entry point takes the owning
//! `CFlightPlan*` and routes through `GetControllerAssignedData()`.

use std::ffi::{c_char, c_int};

use crate::FlightPlanPtr;

unsafe extern "C" {
    /// `GetSquawk` — the assigned squawk (may be empty). Borrowed, NUL-terminated.
    pub fn es_assigneddata_squawk(h: FlightPlanPtr) -> *const c_char;

    /// `SetSquawk` — assign a new squawk. Returns success.
    pub fn es_assigneddata_set_squawk(h: FlightPlanPtr, squawk: *const c_char) -> bool;

    /// `GetFinalAltitude` — controller-overridden final altitude.
    pub fn es_assigneddata_final_altitude(h: FlightPlanPtr) -> c_int;

    /// `SetFinalAltitude` — set the final altitude. Returns success.
    pub fn es_assigneddata_set_final_altitude(h: FlightPlanPtr, final_altitude: c_int) -> bool;

    /// `GetClearedAltitude` — cleared altitude (0 = none, 1 = ILS, 2 = visual).
    pub fn es_assigneddata_cleared_altitude(h: FlightPlanPtr) -> c_int;

    /// `SetClearedAltitude` — set the cleared altitude. Returns success.
    pub fn es_assigneddata_set_cleared_altitude(h: FlightPlanPtr, cleared_altitude: c_int) -> bool;

    /// `GetCommunicationType` — comms type code (0 unassigned, V, R, T).
    pub fn es_assigneddata_communication_type(h: FlightPlanPtr) -> c_char;

    /// `SetCommunicationType` — set the comms type code. Returns success.
    pub fn es_assigneddata_set_communication_type(
        h: FlightPlanPtr,
        communication_type: c_char,
    ) -> bool;

    /// `GetScratchPadString` — scratch pad string value. Borrowed, NUL-terminated.
    pub fn es_assigneddata_scratch_pad_string(h: FlightPlanPtr) -> *const c_char;

    /// `SetScratchPadString` — set the scratch pad string. Returns success.
    pub fn es_assigneddata_set_scratch_pad_string(h: FlightPlanPtr, string: *const c_char) -> bool;

    /// `GetAssignedSpeed` — assigned speed (0 = none).
    pub fn es_assigneddata_assigned_speed(h: FlightPlanPtr) -> c_int;

    /// `SetAssignedSpeed` — set the assigned speed. Returns success.
    pub fn es_assigneddata_set_assigned_speed(h: FlightPlanPtr, assigned_speed: c_int) -> bool;

    /// `GetAssignedMach` — assigned Mach * 100 (0 = none).
    pub fn es_assigneddata_assigned_mach(h: FlightPlanPtr) -> c_int;

    /// `SetAssignedMach` — set the assigned Mach (* 100). Returns success.
    pub fn es_assigneddata_set_assigned_mach(h: FlightPlanPtr, assigned_mach: c_int) -> bool;

    /// `GetAssignedRate` — assigned climb/descend rate (0 = none).
    pub fn es_assigneddata_assigned_rate(h: FlightPlanPtr) -> c_int;

    /// `SetAssignedRate` — set the assigned rate. Returns success.
    pub fn es_assigneddata_set_assigned_rate(h: FlightPlanPtr, assigned_rate: c_int) -> bool;

    /// `GetAssignedHeading` — assigned heading (0 = none).
    pub fn es_assigneddata_assigned_heading(h: FlightPlanPtr) -> c_int;

    /// `SetAssignedHeading` — set the assigned heading. Returns success.
    pub fn es_assigneddata_set_assigned_heading(h: FlightPlanPtr, assigned_heading: c_int) -> bool;

    /// `GetDirectToPointName` — assigned direct-to point name. Borrowed, NUL-terminated.
    pub fn es_assigneddata_direct_to_point_name(h: FlightPlanPtr) -> *const c_char;

    /// `SetDirectToPointName` — set the direct-to point name. Returns success.
    pub fn es_assigneddata_set_direct_to_point_name(
        h: FlightPlanPtr,
        point_name: *const c_char,
    ) -> bool;

    /// `GetFlightStripAnnotation` — annotation at `index` (0-8). Borrowed, NUL-terminated.
    pub fn es_assigneddata_flight_strip_annotation(h: FlightPlanPtr, index: c_int)
    -> *const c_char;

    /// `SetFlightStripAnnotation` — set annotation at `index` (0-8). Returns success.
    pub fn es_assigneddata_set_flight_strip_annotation(
        h: FlightPlanPtr,
        index: c_int,
        annotation: *const c_char,
    ) -> bool;
}
