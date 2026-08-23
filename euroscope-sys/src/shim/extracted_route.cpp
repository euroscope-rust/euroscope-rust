// Rust -> EuroScope wrappers for CFlightPlanExtractedRoute.
//
// CFlightPlanExtractedRoute is a sub-object of CFlightPlan, obtained by value
// via CFlightPlan::GetExtractedRoute(). It shares the flight plan's underlying
// pointer, so every wrapper takes the CFlightPlan handle and re-derives the
// route on each call.

#include "common.h"

extern "C" int
es_extractedroute_points_number(void* h)
{
  return ES_AS(CFlightPlan, h)->GetExtractedRoute().GetPointsNumber();
}

extern "C" int
es_extractedroute_points_calculated_index(void* h)
{
  return ES_AS(CFlightPlan, h)->GetExtractedRoute().GetPointsCalculatedIndex();
}

extern "C" int
es_extractedroute_points_assigned_index(void* h)
{
  return ES_AS(CFlightPlan, h)->GetExtractedRoute().GetPointsAssignedIndex();
}

extern "C" const char*
es_extractedroute_point_name(void* h, int index)
{
  return ES_AS(CFlightPlan, h)->GetExtractedRoute().GetPointName(index);
}

extern "C" void
es_extractedroute_point_position(void* h, int index, double* lat, double* lon)
{
  CPosition p =
    ES_AS(CFlightPlan, h)->GetExtractedRoute().GetPointPosition(index);
  *lat = p.m_Latitude;
  *lon = p.m_Longitude;
}

extern "C" const char*
es_extractedroute_point_airway_name(void* h, int index)
{
  return ES_AS(CFlightPlan, h)->GetExtractedRoute().GetPointAirwayName(index);
}

extern "C" int
es_extractedroute_point_airway_classification(void* h, int index)
{
  return ES_AS(CFlightPlan, h)
    ->GetExtractedRoute()
    .GetPointAirwayClassification(index);
}

extern "C" int
es_extractedroute_point_distance_in_minutes(void* h, int index)
{
  return ES_AS(CFlightPlan, h)
    ->GetExtractedRoute()
    .GetPointDistanceInMinutes(index);
}

extern "C" int
es_extractedroute_point_calculated_profile_altitude(void* h, int index)
{
  return ES_AS(CFlightPlan, h)
    ->GetExtractedRoute()
    .GetPointCalculatedProfileAltitude(index);
}
