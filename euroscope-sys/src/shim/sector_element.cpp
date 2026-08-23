// Rust -> EuroScope wrappers for CSectorElement.

#include "common.h"

extern "C" bool
es_sectorelement_is_valid(void* h)
{
  return ES_AS(CSectorElement, h)->IsValid();
}

extern "C" int
es_sectorelement_element_type(void* h)
{
  return ES_AS(CSectorElement, h)->GetElementType();
}

extern "C" const char*
es_sectorelement_name(void* h)
{
  return ES_AS(CSectorElement, h)->GetName();
}

// GetPosition fills an out CPosition and returns whether the index is valid.
extern "C" bool
es_sectorelement_position(void* h, int index, double* lat, double* lon)
{
  CPosition p;
  bool ok = ES_AS(CSectorElement, h)->GetPosition(&p, index);
  *lat = p.m_Latitude;
  *lon = p.m_Longitude;
  return ok;
}

extern "C" const char*
es_sectorelement_component_name(void* h, int index)
{
  return ES_AS(CSectorElement, h)->GetComponentName(index);
}

extern "C" double
es_sectorelement_frequency(void* h)
{
  return ES_AS(CSectorElement, h)->GetFrequency();
}

extern "C" const char*
es_sectorelement_runway_name(void* h, int index)
{
  return ES_AS(CSectorElement, h)->GetRunwayName(index);
}

extern "C" int
es_sectorelement_runway_heading(void* h, int index)
{
  return ES_AS(CSectorElement, h)->GetRunwayHeading(index);
}

extern "C" const char*
es_sectorelement_airport_name(void* h)
{
  return ES_AS(CSectorElement, h)->GetAirportName();
}

extern "C" bool
es_sectorelement_is_element_active(void* h, bool departure, int index)
{
  return ES_AS(CSectorElement, h)->IsElementActive(departure, index);
}

// --- Owned selectors, filtered by element type (CPlugIn -> CSectorElement) -

extern "C" void
es_sectorelement_free(void* el)
{
  delete ES_AS(CSectorElement, el);
}

extern "C" void*
es_plugin_sectorelement_select_first(void* plugin, int element_type)
{
  return own(
    ES_AS(CPlugIn, plugin)->SectorFileElementSelectFirst(element_type));
}

extern "C" void*
es_plugin_sectorelement_select_next(void* plugin,
                                    void* current,
                                    int element_type)
{
  return own(ES_AS(CPlugIn, plugin)
               ->SectorFileElementSelectNext(*ES_AS(CSectorElement, current),
                                             element_type));
}
