//! `CFlightPlanExtractedRoute` FFI declarations.
//!
//! A sub-object of `CFlightPlan`; every entry point takes the owning
//! `CFlightPlan` handle and re-derives the extracted route inside the shim.

use std::ffi::{c_char, c_int};

use crate::FlightPlanPtr;

unsafe extern "C" {
    /// `GetPointsNumber` — number of points in the extracted route array.
    pub fn es_extractedroute_points_number(h: FlightPlanPtr) -> c_int;

    /// `GetPointsCalculatedIndex` — index of the route edge closest to the
    /// aircraft's current position (`0..=points-2`), or `-1` if invalid.
    pub fn es_extractedroute_points_calculated_index(h: FlightPlanPtr) -> c_int;

    /// `GetPointsAssignedIndex` — index of the point assigned by a controller as
    /// next (direct to) (`0..=points-1`), or `-1` if no direct was given.
    pub fn es_extractedroute_points_assigned_index(h: FlightPlanPtr) -> c_int;

    /// `GetPointName` — name of the point at `index`. Borrowed, NUL-terminated,
    /// ANSI.
    pub fn es_extractedroute_point_name(h: FlightPlanPtr, index: c_int) -> *const c_char;

    /// `GetPointPosition` — coordinates of the point at `index`, written to the
    /// `lat`/`lon` out-params (decimal degrees).
    pub fn es_extractedroute_point_position(
        h: FlightPlanPtr,
        index: c_int,
        lat: *mut f64,
        lon: *mut f64,
    );

    /// `GetPointAirwayName` — name of the airway or SID/STAR leading into the
    /// point at `index` (empty for index 0). Borrowed, NUL-terminated, ANSI.
    pub fn es_extractedroute_point_airway_name(h: FlightPlanPtr, index: c_int) -> *const c_char;

    /// `GetPointAirwayClassification` — the airway classification
    /// (`AIRWAY_CLASS_...`) of the leg into the point at `index`.
    pub fn es_extractedroute_point_airway_classification(h: FlightPlanPtr, index: c_int) -> c_int;

    /// `GetPointDistanceInMinutes` — distance to the point at `index` in minutes
    /// from the aircraft's current position, or `-1` if already passed.
    pub fn es_extractedroute_point_distance_in_minutes(h: FlightPlanPtr, index: c_int) -> c_int;

    /// `GetPointCalculatedProfileAltitude` — altitude calculated from the route,
    /// climb/descend profile and COPX constraints for the point at `index`.
    pub fn es_extractedroute_point_calculated_profile_altitude(
        h: FlightPlanPtr,
        index: c_int,
    ) -> c_int;
}
