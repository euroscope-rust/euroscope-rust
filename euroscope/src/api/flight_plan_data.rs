//! `CFlightPlanData` — the filed flight-plan data of a flight plan.

use std::marker::PhantomData;

use euroscope_sys::FlightPlanPtr;

use crate::{
    AircraftCategory, CommunicationType, EngineType, NavCapability, WakeCategory,
    utils::{cstr, cstr_lossy},
};

/// The filed flight-plan data belonging to a [`FlightPlan`](crate::FlightPlan).
///
/// A thin handle over EuroScope's `CFlightPlanData`, reached through its owning
/// `CFlightPlan` and valid only for the duration of the callback that delivered
/// the flight plan (`'cb`).
///
/// The getters borrow EuroScope-owned strings; the setters (and
/// [`amend_flight_plan`](Self::amend_flight_plan)) mutate live EuroScope state.
#[derive(Clone, Copy)]
pub struct FlightPlanData<'cb> {
    raw: FlightPlanPtr,
    _marker: PhantomData<&'cb ()>,
}

impl FlightPlanData<'_> {
    pub(crate) fn from_raw(raw: FlightPlanPtr) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw owning `CFlightPlan*`. Escape hatch for `euroscope-sys` calls not
    /// yet wrapped here.
    #[expect(dead_code)]
    pub(crate) fn as_ptr(&self) -> FlightPlanPtr {
        self.raw
    }

    /// Whether any kind of flight plan has been received from the servers.
    pub fn is_received(&self) -> bool {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_is_received(self.raw) }
    }

    /// Whether the flight plan has been amended by a controller.
    pub fn is_amended(&self) -> bool {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_is_amended(self.raw) }
    }

    /// Amend the flight plan. Mutates EuroScope state; returns whether it
    /// succeeded.
    pub fn amend_flight_plan(&self) -> bool {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_amend_flight_plan(self.raw) }
    }

    /// The flight-plan type, `"V"` or `"I"`. Borrows EuroScope memory; valid for
    /// this callback only.
    pub fn plan_type(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or null.
        unsafe { cstr(euroscope_sys::es_flightplandata_plan_type(self.raw)) }
    }

    /// Change the flight-plan type (`"V"` or `"I"`). Mutates EuroScope state;
    /// returns whether it succeeded.
    pub fn set_plan_type(&self, plan_type: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_plan_type(self.raw, cstr_lossy(plan_type).as_ptr())
        }
    }

    /// The unextracted aircraft information string. Borrowed; this callback only.
    pub fn aircraft_info(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_aircraft_info(self.raw)) }
    }

    /// Change the complete aircraft information. Mutates EuroScope state; returns
    /// whether it succeeded.
    pub fn set_aircraft_info(&self, info: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_aircraft_info(self.raw, cstr_lossy(info).as_ptr())
        }
    }

    /// The wake turbulence category.
    pub fn aircraft_wtc(&self) -> WakeCategory {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        WakeCategory::from_raw(unsafe { euroscope_sys::es_flightplandata_aircraft_wtc(self.raw) })
    }

    /// The airframe category.
    pub fn aircraft_type(&self) -> AircraftCategory {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        AircraftCategory::from_raw(unsafe {
            euroscope_sys::es_flightplandata_aircraft_type(self.raw)
        })
    }

    /// The number of engines.
    pub fn engine_number(&self) -> i32 {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_engine_number(self.raw) }
    }

    /// The engine type.
    pub fn engine_type(&self) -> EngineType {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        EngineType::from_raw(unsafe { euroscope_sys::es_flightplandata_engine_type(self.raw) })
    }

    /// The navigation/equipment capability. Note the SDK's original spelling of
    /// the method name is preserved.
    pub fn capibilities(&self) -> NavCapability {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        NavCapability::from_raw(unsafe { euroscope_sys::es_flightplandata_capibilities(self.raw) })
    }

    /// Whether the aircraft is RVSM capable.
    pub fn is_rvsm(&self) -> bool {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_is_rvsm(self.raw) }
    }

    /// The manufacturer's type description. Borrowed; this callback only.
    pub fn manufacturer_type(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_manufacturer_type(self.raw)) }
    }

    /// The unencoded aircraft type as written to the flight plan. Borrowed; this
    /// callback only.
    pub fn aircraft_fp_type(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_aircraft_fp_type(self.raw)) }
    }

    /// The filed true airspeed.
    pub fn true_airspeed(&self) -> i32 {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_true_airspeed(self.raw) }
    }

    /// Set the true airspeed. Mutates EuroScope state; returns whether it
    /// succeeded.
    pub fn set_true_airspeed(&self, true_airspeed: i32) -> bool {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_set_true_airspeed(self.raw, true_airspeed) }
    }

    /// The origin airport. Borrowed; this callback only.
    pub fn origin(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_origin(self.raw)) }
    }

    /// Set the origin airport. Mutates EuroScope state; returns whether it
    /// succeeded.
    pub fn set_origin(&self, origin: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_origin(self.raw, cstr_lossy(origin).as_ptr())
        }
    }

    /// The final requested altitude.
    pub fn final_altitude(&self) -> i32 {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_final_altitude(self.raw) }
    }

    /// Set the final altitude. Mutates EuroScope state; returns whether it
    /// succeeded.
    pub fn set_final_altitude(&self, final_altitude: i32) -> bool {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_set_final_altitude(self.raw, final_altitude) }
    }

    /// The destination airport. Borrowed; this callback only.
    pub fn destination(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_destination(self.raw)) }
    }

    /// Set the destination airport. Mutates EuroScope state; returns whether it
    /// succeeded.
    pub fn set_destination(&self, destination: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_destination(
                self.raw,
                cstr_lossy(destination).as_ptr(),
            )
        }
    }

    /// The alternate airport. Borrowed; this callback only.
    pub fn alternate(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_alternate(self.raw)) }
    }

    /// Set the alternate airport. Mutates EuroScope state; returns whether it
    /// succeeded.
    pub fn set_alternate(&self, alternate: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_alternate(self.raw, cstr_lossy(alternate).as_ptr())
        }
    }

    /// The remarks field. Borrowed; this callback only.
    pub fn remarks(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_remarks(self.raw)) }
    }

    /// Set the remarks field. Mutates EuroScope state; returns whether it
    /// succeeded.
    pub fn set_remarks(&self, remarks: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_remarks(self.raw, cstr_lossy(remarks).as_ptr())
        }
    }

    /// The communication type.
    pub fn communication_type(&self) -> CommunicationType {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        CommunicationType::from_raw(unsafe {
            euroscope_sys::es_flightplandata_communication_type(self.raw)
        })
    }

    /// The route field. Borrowed; this callback only.
    pub fn route(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_route(self.raw)) }
    }

    /// Set the route field. Mutates EuroScope state; returns whether it
    /// succeeded.
    pub fn set_route(&self, route: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe { euroscope_sys::es_flightplandata_set_route(self.raw, cstr_lossy(route).as_ptr()) }
    }

    /// The extracted or assigned SID name. Borrowed; this callback only.
    pub fn sid_name(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_sid_name(self.raw)) }
    }

    /// The extracted or assigned STAR name. Borrowed; this callback only.
    pub fn star_name(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_star_name(self.raw)) }
    }

    /// The extracted or assigned departure runway. Borrowed; this callback only.
    pub fn departure_rwy(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_departure_rwy(self.raw)) }
    }

    /// The extracted or assigned arrival runway. Borrowed; this callback only.
    pub fn arrival_rwy(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_arrival_rwy(self.raw)) }
    }

    /// The pilot-filed estimated departure time (raw uncompiled string).
    /// Borrowed; this callback only.
    pub fn estimated_departure_time(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe {
            cstr(euroscope_sys::es_flightplandata_estimated_departure_time(
                self.raw,
            ))
        }
    }

    /// Set the estimated departure time (raw uncompiled string). Mutates
    /// EuroScope state; returns whether it succeeded.
    pub fn set_estimated_departure_time(&self, dep_time: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_estimated_departure_time(
                self.raw,
                cstr_lossy(dep_time).as_ptr(),
            )
        }
    }

    /// The pilot-filed actual departure time (raw uncompiled string). Borrowed;
    /// this callback only.
    pub fn actual_departure_time(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe {
            cstr(euroscope_sys::es_flightplandata_actual_departure_time(
                self.raw,
            ))
        }
    }

    /// Set the actual departure time (raw uncompiled string). Mutates EuroScope
    /// state; returns whether it succeeded.
    pub fn set_actual_departure_time(&self, dep_time: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_actual_departure_time(
                self.raw,
                cstr_lossy(dep_time).as_ptr(),
            )
        }
    }

    /// The pilot-filed enroute hours (raw uncompiled string). Borrowed; this
    /// callback only.
    pub fn enroute_hours(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_enroute_hours(self.raw)) }
    }

    /// Set the enroute hours field (raw uncompiled string). Mutates EuroScope
    /// state; returns whether it succeeded.
    pub fn set_enroute_hours(&self, enroute_hours: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_enroute_hours(
                self.raw,
                cstr_lossy(enroute_hours).as_ptr(),
            )
        }
    }

    /// The pilot-filed enroute minutes (raw uncompiled string). Borrowed; this
    /// callback only.
    pub fn enroute_minutes(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_enroute_minutes(self.raw)) }
    }

    /// Set the enroute minutes field (raw uncompiled string). Mutates EuroScope
    /// state; returns whether it succeeded.
    pub fn set_enroute_minutes(&self, enroute_minutes: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_enroute_minutes(
                self.raw,
                cstr_lossy(enroute_minutes).as_ptr(),
            )
        }
    }

    /// The pilot-filed available fuel hours (raw uncompiled string). Borrowed;
    /// this callback only.
    pub fn fuel_hours(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_fuel_hours(self.raw)) }
    }

    /// Set the available fuel hours field (raw uncompiled string). Mutates
    /// EuroScope state; returns whether it succeeded.
    pub fn set_fuel_hours(&self, fuel_hours: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_fuel_hours(
                self.raw,
                cstr_lossy(fuel_hours).as_ptr(),
            )
        }
    }

    /// The pilot-filed available fuel minutes (raw uncompiled string). Borrowed;
    /// this callback only.
    pub fn fuel_minutes(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplandata_fuel_minutes(self.raw)) }
    }

    /// Set the available fuel minutes field (raw uncompiled string). Mutates
    /// EuroScope state; returns whether it succeeded.
    pub fn set_fuel_minutes(&self, fuel_minutes: &str) -> bool {
        // SAFETY: `cstr_lossy` yields a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_flightplandata_set_fuel_minutes(
                self.raw,
                cstr_lossy(fuel_minutes).as_ptr(),
            )
        }
    }

    /// The modelled IAS at the given altitude (or flight level) and vertical
    /// direction (`-1` descend, `0` level, `+1` climb).
    pub fn performance_get_ias(&self, altitude: i32, vertical_speed: i32) -> i32 {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe {
            euroscope_sys::es_flightplandata_performance_get_ias(self.raw, altitude, vertical_speed)
        }
    }

    /// The modelled Mach number (multiplied by 100) at the given altitude and
    /// vertical direction (`-1` descend, `0` level, `+1` climb).
    pub fn performance_get_mach(&self, altitude: i32, vertical_speed: i32) -> i32 {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe {
            euroscope_sys::es_flightplandata_performance_get_mach(
                self.raw,
                altitude,
                vertical_speed,
            )
        }
    }

    /// The modelled climb rate at the given flight level.
    pub fn performance_get_climb_rate(&self, altitude: i32) -> i32 {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_performance_get_climb_rate(self.raw, altitude) }
    }

    /// The modelled descent rate at the given flight level.
    pub fn performance_get_descent_rate(&self, altitude: i32) -> i32 {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_flightplandata_performance_get_descent_rate(self.raw, altitude) }
    }
}
