// Rust -> EuroScope wrappers for CRadarTarget.

#include "common.h"

extern "C" bool
es_radartarget_is_valid(void* h)
{
  return ES_AS(CRadarTarget, h)->IsValid();
}

extern "C" const char*
es_radartarget_callsign(void* h)
{
  return ES_AS(CRadarTarget, h)->GetCallsign();
}

extern "C" const char*
es_radartarget_system_id(void* h)
{
  return ES_AS(CRadarTarget, h)->GetSystemID();
}

extern "C" int
es_radartarget_vertical_speed(void* h)
{
  return ES_AS(CRadarTarget, h)->GetVerticalSpeed();
}

extern "C" double
es_radartarget_track_heading(void* h)
{
  return ES_AS(CRadarTarget, h)->GetTrackHeading();
}

extern "C" int
es_radartarget_gs(void* h)
{
  return ES_AS(CRadarTarget, h)->GetGS();
}

extern "C" bool
es_radartarget_correlate_with_flight_plan(void* h, void* fp)
{
  return ES_AS(CRadarTarget, h)
    ->CorrelateWithFlightPlan(*ES_AS(CFlightPlan, fp));
}

extern "C" void
es_radartarget_uncorrelate(void* h)
{
  ES_AS(CRadarTarget, h)->Uncorrelate();
}

// --- Owned selectors & cross-navigation (CPlugIn / CRadarTarget) ----------

extern "C" void
es_radartarget_free(void* rt)
{
  delete ES_AS(CRadarTarget, rt);
}

extern "C" void*
es_plugin_radartarget_select_first(void* plugin)
{
  return own(ES_AS(CPlugIn, plugin)->RadarTargetSelectFirst());
}

extern "C" void*
es_plugin_radartarget_select_next(void* plugin, void* current)
{
  return own(ES_AS(CPlugIn, plugin)
               ->RadarTargetSelectNext(*ES_AS(CRadarTarget, current)));
}

extern "C" void*
es_plugin_radartarget_select_asel(void* plugin)
{
  return own(ES_AS(CPlugIn, plugin)->RadarTargetSelectASEL());
}

extern "C" void*
es_plugin_radartarget_select(void* plugin, const char* callsign)
{
  return own(ES_AS(CPlugIn, plugin)->RadarTargetSelect(callsign));
}

extern "C" void*
es_radartarget_correlated_flight_plan(void* rt)
{
  return own(ES_AS(CRadarTarget, rt)->GetCorrelatedFlightPlan());
}

// Owned CRadarTargetPositionData snapshots (freed via es_rtposdata_free).

extern "C" void*
es_radartarget_current_position(void* rt)
{
  return own(ES_AS(CRadarTarget, rt)->GetPosition());
}

extern "C" void*
es_radartarget_previous_position(void* rt, void* current)
{
  return own(
    ES_AS(CRadarTarget, rt)
      ->GetPreviousPosition(*ES_AS(CRadarTargetPositionData, current)));
}

extern "C" void
es_plugin_set_asel_radartarget(void* plugin, void* rt)
{
  ES_AS(CPlugIn, plugin)->SetASELAircraft(*ES_AS(CRadarTarget, rt));
}
