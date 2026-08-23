//! `CFlightPlan` FFI declarations.

use std::ffi::{c_char, c_int};

use crate::{EsHandle, FlightPlanPtr, PluginPtr};

unsafe extern "C" {
    /// Whether the flight-plan handle references a live aircraft. Wraps
    /// `CFlightPlan::IsValid`.
    pub fn es_flightplan_is_valid(fp: FlightPlanPtr) -> bool;

    /// Callsign of the flight plan. Borrowed, NUL-terminated, ANSI. Wraps
    /// `CFlightPlan::GetCallsign`.
    pub fn es_flightplan_callsign(fp: FlightPlanPtr) -> *const c_char;

    /// Pilot name from statistics data. Wraps `CFlightPlan::GetPilotName`.
    pub fn es_flightplan_pilot_name(fp: FlightPlanPtr) -> *const c_char;

    /// Aircraft state (`FLIGHT_PLAN_STATE_...`). Wraps `CFlightPlan::GetState`.
    pub fn es_flightplan_state(fp: FlightPlanPtr) -> c_int;

    /// Flight-plan state. Wraps `CFlightPlan::GetFPState`.
    pub fn es_flightplan_fp_state(fp: FlightPlanPtr) -> c_int;

    /// Whether ES is simulating this out-of-range aircraft. Wraps
    /// `CFlightPlan::GetSimulated`.
    pub fn es_flightplan_simulated(fp: FlightPlanPtr) -> bool;

    /// Callsign of the tracking controller. Wraps
    /// `CFlightPlan::GetTrackingControllerCallsign`.
    pub fn es_flightplan_tracking_controller_callsign(fp: FlightPlanPtr) -> *const c_char;

    /// Position ID of the tracking controller. Wraps
    /// `CFlightPlan::GetTrackingControllerId`.
    pub fn es_flightplan_tracking_controller_id(fp: FlightPlanPtr) -> *const c_char;

    /// Whether the plane is tracked by me. Wraps
    /// `CFlightPlan::GetTrackingControllerIsMe`.
    pub fn es_flightplan_tracking_controller_is_me(fp: FlightPlanPtr) -> bool;

    /// Callsign of the handoff target controller. Wraps
    /// `CFlightPlan::GetHandoffTargetControllerCallsign`.
    pub fn es_flightplan_handoff_target_controller_callsign(fp: FlightPlanPtr) -> *const c_char;

    /// Position ID of the handoff target controller. Wraps
    /// `CFlightPlan::GetHandoffTargetControllerId`.
    pub fn es_flightplan_handoff_target_controller_id(fp: FlightPlanPtr) -> *const c_char;

    /// Calculated distance to the destination airport. Wraps
    /// `CFlightPlan::GetDistanceToDestination`.
    pub fn es_flightplan_distance_to_destination(fp: FlightPlanPtr) -> f64;

    /// Calculated distance from the origin airport. Wraps
    /// `CFlightPlan::GetDistanceFromOrigin`.
    pub fn es_flightplan_distance_from_origin(fp: FlightPlanPtr) -> f64;

    /// Next active COPX point name. Wraps `CFlightPlan::GetNextCopxPointName`.
    pub fn es_flightplan_next_copx_point_name(fp: FlightPlanPtr) -> *const c_char;

    /// Next active FIR COPX point name. Wraps
    /// `CFlightPlan::GetNextFirCopxPointName`.
    pub fn es_flightplan_next_fir_copx_point_name(fp: FlightPlanPtr) -> *const c_char;

    /// Minutes until the plane enters my sectors. Wraps
    /// `CFlightPlan::GetSectorEntryMinutes`.
    pub fn es_flightplan_sector_entry_minutes(fp: FlightPlanPtr) -> c_int;

    /// Minutes until the plane leaves my sectors. Wraps
    /// `CFlightPlan::GetSectorExitMinutes`.
    pub fn es_flightplan_sector_exit_minutes(fp: FlightPlanPtr) -> c_int;

    /// RAM warning flag. Wraps `CFlightPlan::GetRAMFlag`.
    pub fn es_flightplan_ram_flag(fp: FlightPlanPtr) -> bool;

    /// CLAM warning flag. Wraps `CFlightPlan::GetCLAMFlag`.
    pub fn es_flightplan_clam_flag(fp: FlightPlanPtr) -> bool;

    /// Ground state (STUP, PUSH, TAXI, ...). Wraps
    /// `CFlightPlan::GetGroundState`.
    pub fn es_flightplan_ground_state(fp: FlightPlanPtr) -> *const c_char;

    /// Whether the clearance has been received. Wraps
    /// `CFlightPlan::GetClearenceFlag`.
    pub fn es_flightplan_clearence_flag(fp: FlightPlanPtr) -> bool;

    /// Whether text communication was indicated. Wraps
    /// `CFlightPlan::IsTextCommunication`.
    pub fn es_flightplan_is_text_communication(fp: FlightPlanPtr) -> bool;

    /// Final altitude. Wraps `CFlightPlan::GetFinalAltitude`.
    pub fn es_flightplan_final_altitude(fp: FlightPlanPtr) -> c_int;

    /// Cleared altitude. Wraps `CFlightPlan::GetClearedAltitude`.
    pub fn es_flightplan_cleared_altitude(fp: FlightPlanPtr) -> c_int;

    /// Entry point coordination state. Wraps
    /// `CFlightPlan::GetEntryCoordinationPointState`.
    pub fn es_flightplan_entry_coordination_point_state(fp: FlightPlanPtr) -> c_int;

    /// Entry coordination point name. Wraps
    /// `CFlightPlan::GetEntryCoordinationPointName`.
    pub fn es_flightplan_entry_coordination_point_name(fp: FlightPlanPtr) -> *const c_char;

    /// Entry point altitude coordination state. Wraps
    /// `CFlightPlan::GetEntryCoordinationAltitudeState`.
    pub fn es_flightplan_entry_coordination_altitude_state(fp: FlightPlanPtr) -> c_int;

    /// Entry coordination point altitude. Wraps
    /// `CFlightPlan::GetEntryCoordinationAltitude`.
    pub fn es_flightplan_entry_coordination_altitude(fp: FlightPlanPtr) -> c_int;

    /// Exit point name coordination state. Wraps
    /// `CFlightPlan::GetExitCoordinationNameState`.
    pub fn es_flightplan_exit_coordination_name_state(fp: FlightPlanPtr) -> c_int;

    /// Exit coordination point name. Wraps
    /// `CFlightPlan::GetExitCoordinationPointName`.
    pub fn es_flightplan_exit_coordination_point_name(fp: FlightPlanPtr) -> *const c_char;

    /// Exit point altitude coordination state. Wraps
    /// `CFlightPlan::GetExitCoordinationAltitudeState`.
    pub fn es_flightplan_exit_coordination_altitude_state(fp: FlightPlanPtr) -> c_int;

    /// Exit coordination point altitude. Wraps
    /// `CFlightPlan::GetExitCoordinationAltitude`.
    pub fn es_flightplan_exit_coordination_altitude(fp: FlightPlanPtr) -> c_int;

    /// Callsign of the coordinated next controller. Wraps
    /// `CFlightPlan::GetCoordinatedNextController`.
    pub fn es_flightplan_coordinated_next_controller(fp: FlightPlanPtr) -> *const c_char;

    /// Next controller coordination state. Wraps
    /// `CFlightPlan::GetCoordinatedNextControllerState`.
    pub fn es_flightplan_coordinated_next_controller_state(fp: FlightPlanPtr) -> c_int;

    /// Uncorrelate the flight plan from its current target. Wraps
    /// `CFlightPlan::Uncorrelate`.
    pub fn es_flightplan_uncorrelate(fp: FlightPlanPtr);

    /// Start tracking the aircraft. Wraps `CFlightPlan::StartTracking`.
    pub fn es_flightplan_start_tracking(fp: FlightPlanPtr) -> bool;

    /// Release (drop) the target. Wraps `CFlightPlan::EndTracking`.
    pub fn es_flightplan_end_tracking(fp: FlightPlanPtr) -> bool;

    /// Initiate a handoff to the target controller. Wraps
    /// `CFlightPlan::InitiateHandoff`.
    pub fn es_flightplan_initiate_handoff(
        fp: FlightPlanPtr,
        target_controller: *const c_char,
    ) -> bool;

    /// Accept a handoff initiated to me. Wraps `CFlightPlan::AcceptHandoff`.
    pub fn es_flightplan_accept_handoff(fp: FlightPlanPtr);

    /// Refuse a handoff initiated to me. Wraps `CFlightPlan::RefuseHandoff`.
    pub fn es_flightplan_refuse_handoff(fp: FlightPlanPtr);

    /// Initiate a COPN/COPX request. Wraps
    /// `CFlightPlan::InitiateCoordination`.
    pub fn es_flightplan_initiate_coordination(
        fp: FlightPlanPtr,
        target_controller: *const c_char,
        point_name: *const c_char,
        altitude: c_int,
    ) -> bool;

    /// Accept a coordination initiated to me. Wraps
    /// `CFlightPlan::AcceptCoordination`.
    pub fn es_flightplan_accept_coordination(fp: FlightPlanPtr);

    /// Refuse a coordination initiated to me. Wraps
    /// `CFlightPlan::RefuseCoordination`.
    pub fn es_flightplan_refuse_coordination(fp: FlightPlanPtr);

    /// Send the flight strip to a controller. Wraps
    /// `CFlightPlan::PushFlightStrip`.
    pub fn es_flightplan_push_flight_strip(fp: FlightPlanPtr, target_controller: *const c_char);

    /// Set the estimated arrival time at a point. Wraps
    /// `CFlightPlan::SetEstimation`.
    pub fn es_flightplan_set_estimation(
        fp: FlightPlanPtr,
        point_name: *const c_char,
        time: *const c_char,
    );
    // NOTE: CFlightPlan::ClearEstimation is not exported by the SDK import
    // library, so it is intentionally not wrapped.

    /// Free a flight-plan handle allocated by a `*flightplan_select*` /
    /// `*_correlated_*` / `*_track_position` call.
    pub fn es_flightplan_free(fp: EsHandle);

    /// First flight plan in the session, or null.
    /// Wraps `CPlugIn::FlightPlanSelectFirst`.
    pub fn es_plugin_flightplan_select_first(plugin: PluginPtr) -> EsHandle;

    /// Flight plan after `current`, or null. Wraps `CPlugIn::FlightPlanSelectNext`.
    pub fn es_plugin_flightplan_select_next(plugin: PluginPtr, current: EsHandle) -> EsHandle;

    /// The ASEL (selected) flight plan, or null. Wraps `CPlugIn::FlightPlanSelectASEL`.
    pub fn es_plugin_flightplan_select_asel(plugin: PluginPtr) -> EsHandle;

    /// Select a flight plan by callsign, or null. Wraps `CPlugIn::FlightPlanSelect`.
    pub fn es_plugin_flightplan_select(plugin: PluginPtr, callsign: *const c_char) -> EsHandle;

    /// The radar target correlated with `fp`, or null (owned; free with
    /// `es_radartarget_free`). Wraps `CFlightPlan::GetCorrelatedRadarTarget`.
    pub fn es_flightplan_correlated_radar_target(fp: EsHandle) -> EsHandle;

    /// Owned FP-track position snapshot (free with `es_rtposdata_free`).
    /// Wraps `CFlightPlan::GetFPTrackPosition`.
    pub fn es_flightplan_track_position(fp: EsHandle) -> EsHandle;

    /// Set the ASEL (selected) aircraft to `fp`. Wraps `CPlugIn::SetASELAircraft`.
    pub fn es_plugin_set_asel_flightplan(plugin: PluginPtr, fp: EsHandle);
}
