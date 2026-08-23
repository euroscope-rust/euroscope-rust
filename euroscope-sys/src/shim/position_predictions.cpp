// Rust -> EuroScope wrappers for CFlightPlanPositionPredictions.
//
// Sub-object of CFlightPlan: the handle passed in is a CFlightPlan*, and every
// wrapper derives the prediction array from it via GetPositionPredictions().

#include "common.h"

extern "C" int
es_positionpredictions_points_number(void* h)
{
  return ES_AS(CFlightPlan, h)->GetPositionPredictions().GetPointsNumber();
}

extern "C" void
es_positionpredictions_position(void* h, int index, double* lat, double* lon)
{
  CPosition p =
    ES_AS(CFlightPlan, h)->GetPositionPredictions().GetPosition(index);
  *lat = p.m_Latitude;
  *lon = p.m_Longitude;
}

extern "C" int
es_positionpredictions_altitude(void* h, int index)
{
  return ES_AS(CFlightPlan, h)->GetPositionPredictions().GetAltitude(index);
}

extern "C" const char*
es_positionpredictions_controller_id(void* h, int index)
{
  return ES_AS(CFlightPlan, h)->GetPositionPredictions().GetControllerId(index);
}
