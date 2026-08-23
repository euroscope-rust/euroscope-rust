//! `CFlightPlanPositionPredictions` FFI declarations.
//!
//! This is a sub-object of `CFlightPlan`; the handle argument is the owning
//! `CFlightPlan*`, from which the shim derives the prediction array.

use std::ffi::{c_char, c_int};

use crate::FlightPlanPtr;

unsafe extern "C" {
    /// `GetPointsNumber` — number of points in the prediction array.
    pub fn es_positionpredictions_points_number(h: FlightPlanPtr) -> c_int;

    /// `GetPosition(Index)` — predicted position `Index` minutes from now,
    /// returned via `lat`/`lon` out-params.
    pub fn es_positionpredictions_position(
        h: FlightPlanPtr,
        index: c_int,
        lat: *mut f64,
        lon: *mut f64,
    );

    /// `GetAltitude(Index)` — predicted altitude/level `Index` minutes from now
    /// (no ALT/FL conversion).
    pub fn es_positionpredictions_altitude(h: FlightPlanPtr, index: c_int) -> c_int;

    /// `GetControllerId(Index)` — position ID of the predicted controller
    /// `Index` minutes from now. Borrowed, NUL-terminated, ANSI.
    pub fn es_positionpredictions_controller_id(h: FlightPlanPtr, index: c_int) -> *const c_char;
}
