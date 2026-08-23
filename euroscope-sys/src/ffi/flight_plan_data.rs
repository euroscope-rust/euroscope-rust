//! `CFlightPlanData` FFI declarations.
//!
//! `CFlightPlanData` is reached through its owning `CFlightPlan`, so every
//! wrapper takes a [`FlightPlanPtr`] and the shim re-derives the data object.

use std::ffi::{c_char, c_int};

use crate::FlightPlanPtr;

unsafe extern "C" {
    /// `CFlightPlanData::IsReceived` — whether any FP was received from servers.
    pub fn es_flightplandata_is_received(h: FlightPlanPtr) -> bool;

    /// `CFlightPlanData::IsAmended` — whether the FP was amended by a controller.
    pub fn es_flightplandata_is_amended(h: FlightPlanPtr) -> bool;

    /// `CFlightPlanData::AmendFlightPlan` — amend the FP. Mutates ES state.
    pub fn es_flightplandata_amend_flight_plan(h: FlightPlanPtr) -> bool;

    /// `CFlightPlanData::GetPlanType` — the FP type (`V` or `I`). Borrowed ANSI.
    pub fn es_flightplandata_plan_type(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetPlanType` — change the FP type. Mutates ES state.
    pub fn es_flightplandata_set_plan_type(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetAircraftInfo` — unextracted aircraft info. Borrowed.
    pub fn es_flightplandata_aircraft_info(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetAircraftInfo` — change the AC info. Mutates ES state.
    pub fn es_flightplandata_set_aircraft_info(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetAircraftWtc` — weight category code (`?LMHJ`).
    pub fn es_flightplandata_aircraft_wtc(h: FlightPlanPtr) -> c_char;

    /// `CFlightPlanData::GetAircraftType` — aircraft type code (`?LSAHGT`).
    pub fn es_flightplandata_aircraft_type(h: FlightPlanPtr) -> c_char;

    /// `CFlightPlanData::GetEngineNumber` — number of engines.
    pub fn es_flightplandata_engine_number(h: FlightPlanPtr) -> c_int;

    /// `CFlightPlanData::GetEngineType` — engine type code (`?PTJE`).
    pub fn es_flightplandata_engine_type(h: FlightPlanPtr) -> c_char;

    /// `CFlightPlanData::GetCapibilities` — navigation capabilities code.
    pub fn es_flightplandata_capibilities(h: FlightPlanPtr) -> c_char;

    /// `CFlightPlanData::IsRvsm` — whether the aircraft is RVSM capable.
    pub fn es_flightplandata_is_rvsm(h: FlightPlanPtr) -> bool;

    /// `CFlightPlanData::GetManufacturerType` — manufacturer type description.
    pub fn es_flightplandata_manufacturer_type(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::GetAircraftFPType` — unencoded AC type as filed.
    pub fn es_flightplandata_aircraft_fp_type(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::GetTrueAirspeed` — filed true airspeed.
    pub fn es_flightplandata_true_airspeed(h: FlightPlanPtr) -> c_int;

    /// `CFlightPlanData::SetTrueAirspeed` — set the true airspeed. Mutates ES.
    pub fn es_flightplandata_set_true_airspeed(h: FlightPlanPtr, v: c_int) -> bool;

    /// `CFlightPlanData::GetOrigin` — origin airport. Borrowed ANSI.
    pub fn es_flightplandata_origin(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetOrigin` — change the origin airport. Mutates ES.
    pub fn es_flightplandata_set_origin(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetFinalAltitude` — final requested altitude.
    pub fn es_flightplandata_final_altitude(h: FlightPlanPtr) -> c_int;

    /// `CFlightPlanData::SetFinalAltitude` — set the final altitude. Mutates ES.
    pub fn es_flightplandata_set_final_altitude(h: FlightPlanPtr, v: c_int) -> bool;

    /// `CFlightPlanData::GetDestination` — destination airport. Borrowed ANSI.
    pub fn es_flightplandata_destination(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetDestination` — set the destination. Mutates ES.
    pub fn es_flightplandata_set_destination(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetAlternate` — alternate airport. Borrowed ANSI.
    pub fn es_flightplandata_alternate(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetAlternate` — set the alternate airport. Mutates ES.
    pub fn es_flightplandata_set_alternate(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetRemarks` — remarks field. Borrowed ANSI.
    pub fn es_flightplandata_remarks(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetRemarks` — set the remarks field. Mutates ES.
    pub fn es_flightplandata_set_remarks(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetCommunicationType` — comms type code (`?VRT`).
    pub fn es_flightplandata_communication_type(h: FlightPlanPtr) -> c_char;

    /// `CFlightPlanData::GetRoute` — route field. Borrowed ANSI.
    pub fn es_flightplandata_route(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetRoute` — set the route field. Mutates ES.
    pub fn es_flightplandata_set_route(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetSidName` — extracted or assigned SID name.
    pub fn es_flightplandata_sid_name(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::GetStarName` — extracted or assigned STAR name.
    pub fn es_flightplandata_star_name(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::GetDepartureRwy` — extracted or assigned departure RWY.
    pub fn es_flightplandata_departure_rwy(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::GetArrivalRwy` — extracted or assigned arrival RWY.
    pub fn es_flightplandata_arrival_rwy(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::GetEstimatedDepartureTime` — filed EOBT (raw string).
    pub fn es_flightplandata_estimated_departure_time(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetEstimatedDepartureTime` — set the EOBT. Mutates ES.
    pub fn es_flightplandata_set_estimated_departure_time(
        h: FlightPlanPtr,
        s: *const c_char,
    ) -> bool;

    /// `CFlightPlanData::GetActualDepartureTime` — actual departure (raw string).
    pub fn es_flightplandata_actual_departure_time(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetActualDepartureTime` — set actual departure. Mutates ES.
    pub fn es_flightplandata_set_actual_departure_time(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetEnrouteHours` — enroute hours (raw string).
    pub fn es_flightplandata_enroute_hours(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetEnrouteHours` — set enroute hours. Mutates ES.
    pub fn es_flightplandata_set_enroute_hours(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetEnrouteMinutes` — enroute minutes (raw string).
    pub fn es_flightplandata_enroute_minutes(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetEnrouteMinutes` — set enroute minutes. Mutates ES.
    pub fn es_flightplandata_set_enroute_minutes(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetFuelHours` — available fuel hours (raw string).
    pub fn es_flightplandata_fuel_hours(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetFuelHours` — set available fuel hours. Mutates ES.
    pub fn es_flightplandata_set_fuel_hours(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::GetFuelMinutes` — available fuel minutes (raw string).
    pub fn es_flightplandata_fuel_minutes(h: FlightPlanPtr) -> *const c_char;

    /// `CFlightPlanData::SetFuelMinutes` — set available fuel minutes. Mutates ES.
    pub fn es_flightplandata_set_fuel_minutes(h: FlightPlanPtr, s: *const c_char) -> bool;

    /// `CFlightPlanData::PerformanceGetIas` — IAS at the given altitude/VS.
    pub fn es_flightplandata_performance_get_ias(
        h: FlightPlanPtr,
        altitude: c_int,
        vertical_speed: c_int,
    ) -> c_int;

    /// `CFlightPlanData::PerformanceGetMach` — Mach (x100) at altitude/VS.
    pub fn es_flightplandata_performance_get_mach(
        h: FlightPlanPtr,
        altitude: c_int,
        vertical_speed: c_int,
    ) -> c_int;

    /// `CFlightPlanData::PerformanceGetClimbRate` — climb rate at altitude.
    pub fn es_flightplandata_performance_get_climb_rate(h: FlightPlanPtr, altitude: c_int)
    -> c_int;

    /// `CFlightPlanData::PerformanceGetDescentRate` — descent rate at altitude.
    pub fn es_flightplandata_performance_get_descent_rate(
        h: FlightPlanPtr,
        altitude: c_int,
    ) -> c_int;
}
