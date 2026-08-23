// Rust -> EuroScope wrappers for CController.

#include "common.h"

extern "C" bool
es_controller_is_valid(void* h)
{
  return ES_AS(CController, h)->IsValid();
}

extern "C" const char*
es_controller_callsign(void* h)
{
  return ES_AS(CController, h)->GetCallsign();
}

extern "C" const char*
es_controller_position_id(void* h)
{
  return ES_AS(CController, h)->GetPositionId();
}

extern "C" bool
es_controller_position_identified(void* h)
{
  return ES_AS(CController, h)->GetPositionIdentified();
}

extern "C" double
es_controller_primary_frequency(void* h)
{
  return ES_AS(CController, h)->GetPrimaryFrequency();
}

extern "C" const char*
es_controller_full_name(void* h)
{
  return ES_AS(CController, h)->GetFullName();
}

extern "C" int
es_controller_rating(void* h)
{
  return ES_AS(CController, h)->GetRating();
}

extern "C" int
es_controller_facility(void* h)
{
  return ES_AS(CController, h)->GetFacility();
}

extern "C" const char*
es_controller_sector_file_name(void* h)
{
  return ES_AS(CController, h)->GetSectorFileName();
}

extern "C" bool
es_controller_is_controller(void* h)
{
  return ES_AS(CController, h)->IsController();
}

extern "C" void
es_controller_position(void* h, double* lat, double* lon)
{
  CPosition p = ES_AS(CController, h)->GetPosition();
  *lat = p.m_Latitude;
  *lon = p.m_Longitude;
}

extern "C" int
es_controller_range(void* h)
{
  return ES_AS(CController, h)->GetRange();
}

extern "C" bool
es_controller_is_breaking(void* h)
{
  return ES_AS(CController, h)->IsBreaking();
}

extern "C" bool
es_controller_is_ongoing_able(void* h)
{
  return ES_AS(CController, h)->IsOngoingAble();
}

// --- Owned selectors (CPlugIn -> CController) -----------------------------

extern "C" void
es_controller_free(void* c)
{
  delete ES_AS(CController, c);
}

extern "C" void*
es_plugin_controller_select_first(void* plugin)
{
  return own(ES_AS(CPlugIn, plugin)->ControllerSelectFirst());
}

extern "C" void*
es_plugin_controller_select_next(void* plugin, void* current)
{
  return own(
    ES_AS(CPlugIn, plugin)->ControllerSelectNext(*ES_AS(CController, current)));
}

extern "C" void*
es_plugin_controller_select(void* plugin, const char* callsign)
{
  return own(ES_AS(CPlugIn, plugin)->ControllerSelect(callsign));
}

extern "C" void*
es_plugin_controller_select_by_position_id(void* plugin,
                                           const char* position_id)
{
  return own(ES_AS(CPlugIn, plugin)->ControllerSelectByPositionId(position_id));
}

extern "C" void*
es_plugin_controller_myself(void* plugin)
{
  return own(ES_AS(CPlugIn, plugin)->ControllerMyself());
}
