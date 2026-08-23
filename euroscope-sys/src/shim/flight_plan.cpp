// Rust -> EuroScope wrappers for CFlightPlan.

#include "common.h"

extern "C" bool
es_flightplan_is_valid(void* fp)
{
  return ES_AS(CFlightPlan, fp)->IsValid();
}

extern "C" const char*
es_flightplan_callsign(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetCallsign();
}

extern "C" const char*
es_flightplan_pilot_name(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetPilotName();
}

extern "C" int
es_flightplan_state(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetState();
}

extern "C" int
es_flightplan_fp_state(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetFPState();
}

extern "C" bool
es_flightplan_simulated(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetSimulated();
}

extern "C" const char*
es_flightplan_tracking_controller_callsign(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetTrackingControllerCallsign();
}

extern "C" const char*
es_flightplan_tracking_controller_id(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetTrackingControllerId();
}

extern "C" bool
es_flightplan_tracking_controller_is_me(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetTrackingControllerIsMe();
}

extern "C" const char*
es_flightplan_handoff_target_controller_callsign(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetHandoffTargetControllerCallsign();
}

extern "C" const char*
es_flightplan_handoff_target_controller_id(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetHandoffTargetControllerId();
}

extern "C" double
es_flightplan_distance_to_destination(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetDistanceToDestination();
}

extern "C" double
es_flightplan_distance_from_origin(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetDistanceFromOrigin();
}

extern "C" const char*
es_flightplan_next_copx_point_name(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetNextCopxPointName();
}

extern "C" const char*
es_flightplan_next_fir_copx_point_name(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetNextFirCopxPointName();
}

extern "C" int
es_flightplan_sector_entry_minutes(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetSectorEntryMinutes();
}

extern "C" int
es_flightplan_sector_exit_minutes(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetSectorExitMinutes();
}

extern "C" bool
es_flightplan_ram_flag(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetRAMFlag();
}

extern "C" bool
es_flightplan_clam_flag(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetCLAMFlag();
}

extern "C" const char*
es_flightplan_ground_state(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetGroundState();
}

extern "C" bool
es_flightplan_clearence_flag(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetClearenceFlag();
}

extern "C" bool
es_flightplan_is_text_communication(void* fp)
{
  return ES_AS(CFlightPlan, fp)->IsTextCommunication();
}

extern "C" int
es_flightplan_final_altitude(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetFinalAltitude();
}

extern "C" int
es_flightplan_cleared_altitude(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetClearedAltitude();
}

extern "C" int
es_flightplan_entry_coordination_point_state(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetEntryCoordinationPointState();
}

extern "C" const char*
es_flightplan_entry_coordination_point_name(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetEntryCoordinationPointName();
}

extern "C" int
es_flightplan_entry_coordination_altitude_state(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetEntryCoordinationAltitudeState();
}

extern "C" int
es_flightplan_entry_coordination_altitude(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetEntryCoordinationAltitude();
}

extern "C" int
es_flightplan_exit_coordination_name_state(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetExitCoordinationNameState();
}

extern "C" const char*
es_flightplan_exit_coordination_point_name(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetExitCoordinationPointName();
}

extern "C" int
es_flightplan_exit_coordination_altitude_state(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetExitCoordinationAltitudeState();
}

extern "C" int
es_flightplan_exit_coordination_altitude(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetExitCoordinationAltitude();
}

extern "C" const char*
es_flightplan_coordinated_next_controller(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetCoordinatedNextController();
}

extern "C" int
es_flightplan_coordinated_next_controller_state(void* fp)
{
  return ES_AS(CFlightPlan, fp)->GetCoordinatedNextControllerState();
}

extern "C" void
es_flightplan_uncorrelate(void* fp)
{
  ES_AS(CFlightPlan, fp)->Uncorrelate();
}

extern "C" bool
es_flightplan_start_tracking(void* fp)
{
  return ES_AS(CFlightPlan, fp)->StartTracking();
}

extern "C" bool
es_flightplan_end_tracking(void* fp)
{
  return ES_AS(CFlightPlan, fp)->EndTracking();
}

extern "C" bool
es_flightplan_initiate_handoff(void* fp, const char* target_controller)
{
  return ES_AS(CFlightPlan, fp)->InitiateHandoff(target_controller);
}

extern "C" void
es_flightplan_accept_handoff(void* fp)
{
  ES_AS(CFlightPlan, fp)->AcceptHandoff();
}

extern "C" void
es_flightplan_refuse_handoff(void* fp)
{
  ES_AS(CFlightPlan, fp)->RefuseHandoff();
}

extern "C" bool
es_flightplan_initiate_coordination(void* fp,
                                    const char* target_controller,
                                    const char* point_name,
                                    int altitude)
{
  return ES_AS(CFlightPlan, fp)
    ->InitiateCoordination(target_controller, point_name, altitude);
}

extern "C" void
es_flightplan_accept_coordination(void* fp)
{
  ES_AS(CFlightPlan, fp)->AcceptCoordination();
}

extern "C" void
es_flightplan_refuse_coordination(void* fp)
{
  ES_AS(CFlightPlan, fp)->RefuseCoordination();
}

extern "C" void
es_flightplan_push_flight_strip(void* fp, const char* target_controller)
{
  ES_AS(CFlightPlan, fp)->PushFlightStrip(target_controller);
}

extern "C" void
es_flightplan_set_estimation(void* fp, const char* point_name, const char* time)
{
  ES_AS(CFlightPlan, fp)->SetEstimation(point_name, time);
}

// NOTE: CFlightPlan::ClearEstimation is declared in the header but not
// exported by EuroScopePlugInDll.lib, so it cannot be wrapped (would fail to
// link).

// --- Owned selectors & cross-navigation (CPlugIn / CFlightPlan) -----------

extern "C" void
es_flightplan_free(void* fp)
{
  delete ES_AS(CFlightPlan, fp);
}

extern "C" void*
es_plugin_flightplan_select_first(void* plugin)
{
  return own(ES_AS(CPlugIn, plugin)->FlightPlanSelectFirst());
}

extern "C" void*
es_plugin_flightplan_select_next(void* plugin, void* current)
{
  return own(
    ES_AS(CPlugIn, plugin)->FlightPlanSelectNext(*ES_AS(CFlightPlan, current)));
}

extern "C" void*
es_plugin_flightplan_select_asel(void* plugin)
{
  return own(ES_AS(CPlugIn, plugin)->FlightPlanSelectASEL());
}

extern "C" void*
es_plugin_flightplan_select(void* plugin, const char* callsign)
{
  return own(ES_AS(CPlugIn, plugin)->FlightPlanSelect(callsign));
}

extern "C" void*
es_flightplan_correlated_radar_target(void* fp)
{
  return own(ES_AS(CFlightPlan, fp)->GetCorrelatedRadarTarget());
}

extern "C" void*
es_flightplan_track_position(void* fp)
{
  return own(ES_AS(CFlightPlan, fp)->GetFPTrackPosition());
}

extern "C" void
es_plugin_set_asel_flightplan(void* plugin, void* fp)
{
  ES_AS(CPlugIn, plugin)->SetASELAircraft(*ES_AS(CFlightPlan, fp));
}
