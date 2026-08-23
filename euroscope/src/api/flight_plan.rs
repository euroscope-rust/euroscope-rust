//! `CFlightPlan` — a reference to a flight plan.

use std::marker::PhantomData;

use euroscope_sys::FlightPlanPtr;

use crate::{
    Context, ControllerAssignedData, ExtractedRoute, FlightPlanData, PositionPredictions,
    api::handle::{iter, scoped},
    utils::{cstr, cstr_lossy},
};

/// A reference to a flight plan.
///
/// A thin handle over EuroScope's `CFlightPlan`, valid only for the duration of
/// the callback that delivered it (`'cb`).
#[derive(Clone, Copy)]
pub struct FlightPlan<'cb> {
    raw: FlightPlanPtr,
    _marker: PhantomData<&'cb ()>,
}

impl<'cb> FlightPlan<'cb> {
    pub(crate) fn from_raw(raw: FlightPlanPtr) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw `CFlightPlan*`. Escape hatch for `euroscope-sys` calls not yet
    /// wrapped here.
    pub(crate) fn as_ptr(&self) -> FlightPlanPtr {
        self.raw
    }

    /// Whether this handle references a live aircraft.
    pub fn is_valid(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_flightplan_is_valid(self.raw) }
    }

    /// The aircraft callsign. Empty if the handle is invalid. Borrows
    /// EuroScope-owned memory; valid for this callback only.
    pub fn callsign(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplan_callsign(self.raw)) }
    }

    /// The pilot name extracted from statistics data. Empty if unavailable.
    pub fn pilot_name(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplan_pilot_name(self.raw)) }
    }

    /// The aircraft state (`FLIGHT_PLAN_STATE_...`). `0` if no selected AC.
    pub fn state(&self) -> crate::FlightPlanState {
        // SAFETY: `raw` is valid for this callback.
        crate::FlightPlanState::from_raw(unsafe { euroscope_sys::es_flightplan_state(self.raw) })
    }

    /// The flight-plan state (not-started / simulated / terminated). `0` if no
    /// selected AC.
    pub fn fp_state(&self) -> crate::SimulationState {
        // SAFETY: `raw` is valid for this callback.
        crate::SimulationState::from_raw(unsafe { euroscope_sys::es_flightplan_fp_state(self.raw) })
    }

    /// Whether the aircraft is out of range and ES simulates its movements.
    pub fn simulated(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_simulated(self.raw) }
    }

    /// The callsign of the controller currently tracking. Empty if untracked.
    pub fn tracking_controller_callsign(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe {
            cstr(euroscope_sys::es_flightplan_tracking_controller_callsign(
                self.raw,
            ))
        }
    }

    /// The position ID of the controller currently tracking. Empty if
    /// untracked.
    pub fn tracking_controller_id(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe {
            cstr(euroscope_sys::es_flightplan_tracking_controller_id(
                self.raw,
            ))
        }
    }

    /// Whether the plane is tracked by me.
    pub fn tracking_controller_is_me(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_tracking_controller_is_me(self.raw) }
    }

    /// The callsign of the controller who is the target of the current
    /// handoff. Empty if none.
    pub fn handoff_target_controller_callsign(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplan_handoff_target_controller_callsign(self.raw)) }
    }

    /// The position ID of the controller who is the target of the current
    /// handoff. Empty if none.
    pub fn handoff_target_controller_id(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe {
            cstr(euroscope_sys::es_flightplan_handoff_target_controller_id(
                self.raw,
            ))
        }
    }

    /// The full calculated distance to the destination airport (nautical
    /// miles).
    pub fn distance_to_destination(&self) -> f64 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_distance_to_destination(self.raw) }
    }

    /// The full calculated distance from the origin airport (nautical miles).
    pub fn distance_from_origin(&self) -> f64 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_distance_from_origin(self.raw) }
    }

    /// The next active COPX point name along the extracted route. May be empty.
    pub fn next_copx_point_name(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplan_next_copx_point_name(self.raw)) }
    }

    /// The next active FIR COPX point name along the extracted route. May be
    /// empty.
    pub fn next_fir_copx_point_name(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe {
            cstr(euroscope_sys::es_flightplan_next_fir_copx_point_name(
                self.raw,
            ))
        }
    }

    /// Minutes until the plane enters my sectors: `0` if already inside, `-1`
    /// if it never will.
    pub fn sector_entry_minutes(&self) -> i32 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_sector_entry_minutes(self.raw) }
    }

    /// Minutes until the plane leaves my sectors, or `-1` if it never enters.
    pub fn sector_exit_minutes(&self) -> i32 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_sector_exit_minutes(self.raw) }
    }

    /// The RAM flag, set when a RAM warning is to be displayed.
    pub fn ram_flag(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_ram_flag(self.raw) }
    }

    /// The CLAM flag, set when a CLAM warning is to be displayed.
    pub fn clam_flag(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_clam_flag(self.raw) }
    }

    /// The ground state (`STUP`, `PUSH`, `TAXI`, `DEPA`, `ARR`, `TAXIIN`,
    /// `PARK`, or empty).
    pub fn ground_state(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_flightplan_ground_state(self.raw)) }
    }

    /// Whether the clearance has been received.
    pub fn clearence_flag(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_clearence_flag(self.raw) }
    }

    /// Whether the pilot or a controller indicated text communication.
    pub fn is_text_communication(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_is_text_communication(self.raw) }
    }

    /// The final altitude from the FP or from controller override (no ALT/FL
    /// change).
    pub fn final_altitude(&self) -> i32 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_final_altitude(self.raw) }
    }

    /// The cleared altitude, or the final one if missing (no ALT/FL change).
    pub fn cleared_altitude(&self) -> i32 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_cleared_altitude(self.raw) }
    }

    /// The entry point coordination state (`COORDINATION_STATE_...`).
    pub fn entry_coordination_point_state(&self) -> crate::CoordinationState {
        // SAFETY: `raw` is valid for this callback.
        crate::CoordinationState::from_raw(unsafe {
            euroscope_sys::es_flightplan_entry_coordination_point_state(self.raw)
        })
    }

    /// The coordination point name for the entry point.
    pub fn entry_coordination_point_name(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe {
            cstr(euroscope_sys::es_flightplan_entry_coordination_point_name(
                self.raw,
            ))
        }
    }

    /// The entry point altitude coordination state (`COORDINATION_STATE_...`).
    pub fn entry_coordination_altitude_state(&self) -> crate::CoordinationState {
        // SAFETY: `raw` is valid for this callback.
        crate::CoordinationState::from_raw(unsafe {
            euroscope_sys::es_flightplan_entry_coordination_altitude_state(self.raw)
        })
    }

    /// The coordination point altitude for the entry point.
    pub fn entry_coordination_altitude(&self) -> i32 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_entry_coordination_altitude(self.raw) }
    }

    /// The exit point name coordination state (`COORDINATION_STATE_...`).
    pub fn exit_coordination_name_state(&self) -> crate::CoordinationState {
        // SAFETY: `raw` is valid for this callback.
        crate::CoordinationState::from_raw(unsafe {
            euroscope_sys::es_flightplan_exit_coordination_name_state(self.raw)
        })
    }

    /// The coordination point name for the exit point.
    pub fn exit_coordination_point_name(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe {
            cstr(euroscope_sys::es_flightplan_exit_coordination_point_name(
                self.raw,
            ))
        }
    }

    /// The exit point altitude coordination state (`COORDINATION_STATE_...`).
    pub fn exit_coordination_altitude_state(&self) -> crate::CoordinationState {
        // SAFETY: `raw` is valid for this callback.
        crate::CoordinationState::from_raw(unsafe {
            euroscope_sys::es_flightplan_exit_coordination_altitude_state(self.raw)
        })
    }

    /// The coordination point altitude for the exit point.
    pub fn exit_coordination_altitude(&self) -> i32 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_exit_coordination_altitude(self.raw) }
    }

    /// The callsign of the coordinated next controller, or empty if none.
    pub fn coordinated_next_controller(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe {
            cstr(euroscope_sys::es_flightplan_coordinated_next_controller(
                self.raw,
            ))
        }
    }

    /// The next controller coordination state (`COORDINATION_STATE_...`).
    pub fn coordinated_next_controller_state(&self) -> crate::CoordinationState {
        // SAFETY: `raw` is valid for this callback.
        crate::CoordinationState::from_raw(unsafe {
            euroscope_sys::es_flightplan_coordinated_next_controller_state(self.raw)
        })
    }

    // --- Navigators to sibling handles (share this flight plan's pointer) ---

    /// The flight-plan data sub-object. Shares this handle's pointer.
    pub fn flight_plan_data(&self) -> FlightPlanData<'cb> {
        FlightPlanData::from_raw(self.as_ptr())
    }

    /// The controller-assigned data sub-object. Shares this handle's pointer.
    pub fn controller_assigned_data(&self) -> ControllerAssignedData<'cb> {
        ControllerAssignedData::from_raw(self.as_ptr())
    }

    /// The extracted route sub-object. Shares this handle's pointer.
    pub fn extracted_route(&self) -> ExtractedRoute<'cb> {
        ExtractedRoute::from_raw(self.as_ptr())
    }

    /// The position predictions sub-object. Shares this handle's pointer.
    pub fn position_predictions(&self) -> PositionPredictions<'cb> {
        PositionPredictions::from_raw(self.as_ptr())
    }

    // --- Actions (mutate EuroScope state) ---

    /// Uncorrelate the flight plan from its current radar target. Mutates
    /// EuroScope state.
    pub fn uncorrelate(&self) {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_uncorrelate(self.raw) }
    }

    /// Start tracking the aircraft if it was untracked. Returns whether it
    /// succeeded. Mutates EuroScope state.
    pub fn start_tracking(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_start_tracking(self.raw) }
    }

    /// Release (drop) the target. Returns whether it succeeded. Mutates
    /// EuroScope state.
    pub fn end_tracking(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_end_tracking(self.raw) }
    }

    /// Initiate a handoff to the specified target controller. Returns whether
    /// it succeeded. Mutates EuroScope state.
    pub fn initiate_handoff(&self, target_controller: &str) -> bool {
        // SAFETY: `raw` is valid; the CString outlives the call.
        unsafe {
            euroscope_sys::es_flightplan_initiate_handoff(
                self.raw,
                cstr_lossy(target_controller).as_ptr(),
            )
        }
    }

    /// Accept a handoff initiated to me. Mutates EuroScope state.
    pub fn accept_handoff(&self) {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_accept_handoff(self.raw) }
    }

    /// Refuse a handoff initiated to me. Mutates EuroScope state.
    pub fn refuse_handoff(&self) {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_refuse_handoff(self.raw) }
    }

    /// Initiate a COPN/COPX request. `point_name` must not be empty; pass
    /// `altitude` of `0` to request only the point. Returns whether it
    /// succeeded. Mutates EuroScope state.
    pub fn initiate_coordination(
        &self,
        target_controller: &str,
        point_name: &str,
        altitude: i32,
    ) -> bool {
        // SAFETY: `raw` is valid; the CStrings outlive the call.
        unsafe {
            euroscope_sys::es_flightplan_initiate_coordination(
                self.raw,
                cstr_lossy(target_controller).as_ptr(),
                cstr_lossy(point_name).as_ptr(),
                altitude,
            )
        }
    }

    /// Accept a coordination initiated to me. Mutates EuroScope state.
    pub fn accept_coordination(&self) {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_accept_coordination(self.raw) }
    }

    /// Refuse a coordination initiated to me. Mutates EuroScope state.
    pub fn refuse_coordination(&self) {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_flightplan_refuse_coordination(self.raw) }
    }

    /// Send the flight strip to a controller. Mutates EuroScope state.
    pub fn push_flight_strip(&self, target_controller: &str) {
        // SAFETY: `raw` is valid; the CString outlives the call.
        unsafe {
            euroscope_sys::es_flightplan_push_flight_strip(
                self.raw,
                cstr_lossy(target_controller).as_ptr(),
            );
        }
    }

    /// Set the estimated arrival time (`HHMM` zulu) at a point along the route.
    /// Mutates EuroScope state.
    pub fn set_estimation(&self, point_name: &str, time: &str) {
        // SAFETY: `raw` is valid; the CStrings outlive the call.
        unsafe {
            euroscope_sys::es_flightplan_set_estimation(
                self.raw,
                cstr_lossy(point_name).as_ptr(),
                cstr_lossy(time).as_ptr(),
            );
        }
    }

    /// The FP-track position snapshot for this flight plan. `None` if
    /// unavailable.
    pub fn track_position(&self) -> Option<crate::RadarTargetPositionHandle<'_>> {
        // SAFETY: `raw` valid for this callback; shim returns a heap copy or null.
        crate::RadarTargetPositionHandle::from_owned(unsafe {
            euroscope_sys::es_flightplan_track_position(self.as_ptr())
        })
    }

    /// The radar target correlated with this flight plan, if any.
    pub fn correlated_radar_target(&self) -> Option<crate::RadarTargetHandle<'_>> {
        // SAFETY: `raw` is valid for this callback; shim returns a heap copy or null.
        crate::RadarTargetHandle::from_owned(unsafe {
            euroscope_sys::es_flightplan_correlated_radar_target(self.as_ptr())
        })
    }

    /// Clear the flight plan's estimations.
    ///
    /// **Always panics.** `CFlightPlan::ClearEstimation` is declared in
    /// `EuroScopePlugIn.h` but is *not exported* by `EuroScopePlugInDll.lib`, so
    /// there is no symbol to link against — wrapping it for real would make the
    /// whole plugin fail to link. It is kept here (panicking) to document the
    /// gap and preserve API shape; if a future SDK exports it, replace the body
    /// with a `euroscope-sys` call.
    #[expect(
        clippy::unimplemented,
        reason = "Declared in the header but not exported in the lib."
    )]
    pub fn clear_estimation(&self) {
        unimplemented!(
            "CFlightPlan::ClearEstimation is not exported by EuroScopePlugInDll.lib, so it cannot \
             be called"
        )
    }
}

scoped!(FlightPlanHandle, FlightPlan, es_flightplan_free);
iter!(
    FlightPlans,
    FlightPlanHandle,
    FlightPlan,
    es_flightplan_free,
    es_plugin_flightplan_select_first,
    es_plugin_flightplan_select_next
);

/// Flight-plan selection and iteration.
impl Context {
    /// Iterate over all flight plans.
    pub fn flight_plans(&self) -> FlightPlans<'_> {
        // SAFETY: `Context` holds a live CPlugIn* for the plugin's lifetime.
        FlightPlans::new(unsafe { self.as_ptr() })
    }

    /// Select a flight plan by callsign.
    pub fn flight_plan(&self, callsign: &str) -> Option<FlightPlanHandle<'_>> {
        // SAFETY: live CPlugIn*; the CString outlives the call.
        FlightPlanHandle::from_owned(unsafe {
            euroscope_sys::es_plugin_flightplan_select(self.as_ptr(), cstr_lossy(callsign).as_ptr())
        })
    }

    /// The currently ASEL (selected) flight plan.
    pub fn selected_flight_plan(&self) -> Option<FlightPlanHandle<'_>> {
        // SAFETY: live CPlugIn*.
        FlightPlanHandle::from_owned(unsafe {
            euroscope_sys::es_plugin_flightplan_select_asel(self.as_ptr())
        })
    }

    /// Set the ASEL (selected) aircraft to the given flight plan.
    pub fn set_asel_flight_plan(&self, flight_plan: FlightPlan<'_>) {
        // SAFETY: live CPlugIn*; the handle is valid for this call.
        unsafe { euroscope_sys::es_plugin_set_asel_flightplan(self.as_ptr(), flight_plan.as_ptr()) }
    }
}
