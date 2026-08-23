// Rust -> EuroScope wrappers for CFlightPlanData.
//
// CFlightPlanData is a by-value sub-object of CFlightPlan, obtained via
// CFlightPlan::GetFlightPlanData(). The handle Rust passes is the owning
// CFlightPlan*, so every wrapper re-derives the data object from it. The data
// object indexes back into the live aircraft, so the setters mutate ES state
// even though they act on the temporary returned by GetFlightPlanData().

#include "common.h"

extern "C" bool
es_flightplandata_is_received(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().IsReceived();
}

extern "C" bool
es_flightplandata_is_amended(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().IsAmended();
}

extern "C" bool
es_flightplandata_amend_flight_plan(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().AmendFlightPlan();
}

extern "C" const char*
es_flightplandata_plan_type(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetPlanType();
}

extern "C" bool
es_flightplandata_set_plan_type(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetPlanType(s);
}

extern "C" const char*
es_flightplandata_aircraft_info(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetAircraftInfo();
}

extern "C" bool
es_flightplandata_set_aircraft_info(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetAircraftInfo(s);
}

extern "C" char
es_flightplandata_aircraft_wtc(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetAircraftWtc();
}

extern "C" char
es_flightplandata_aircraft_type(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetAircraftType();
}

extern "C" int
es_flightplandata_engine_number(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetEngineNumber();
}

extern "C" char
es_flightplandata_engine_type(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetEngineType();
}

extern "C" char
es_flightplandata_capibilities(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetCapibilities();
}

extern "C" bool
es_flightplandata_is_rvsm(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().IsRvsm();
}

extern "C" const char*
es_flightplandata_manufacturer_type(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetManufacturerType();
}

extern "C" const char*
es_flightplandata_aircraft_fp_type(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetAircraftFPType();
}

extern "C" int
es_flightplandata_true_airspeed(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetTrueAirspeed();
}

extern "C" bool
es_flightplandata_set_true_airspeed(void* h, int v)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetTrueAirspeed(v);
}

extern "C" const char*
es_flightplandata_origin(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetOrigin();
}

extern "C" bool
es_flightplandata_set_origin(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetOrigin(s);
}

extern "C" int
es_flightplandata_final_altitude(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetFinalAltitude();
}

extern "C" bool
es_flightplandata_set_final_altitude(void* h, int v)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetFinalAltitude(v);
}

extern "C" const char*
es_flightplandata_destination(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetDestination();
}

extern "C" bool
es_flightplandata_set_destination(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetDestination(s);
}

extern "C" const char*
es_flightplandata_alternate(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetAlternate();
}

extern "C" bool
es_flightplandata_set_alternate(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetAlternate(s);
}

extern "C" const char*
es_flightplandata_remarks(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetRemarks();
}

extern "C" bool
es_flightplandata_set_remarks(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetRemarks(s);
}

extern "C" char
es_flightplandata_communication_type(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetCommunicationType();
}

extern "C" const char*
es_flightplandata_route(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetRoute();
}

extern "C" bool
es_flightplandata_set_route(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetRoute(s);
}

extern "C" const char*
es_flightplandata_sid_name(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetSidName();
}

extern "C" const char*
es_flightplandata_star_name(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetStarName();
}

extern "C" const char*
es_flightplandata_departure_rwy(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetDepartureRwy();
}

extern "C" const char*
es_flightplandata_arrival_rwy(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetArrivalRwy();
}

extern "C" const char*
es_flightplandata_estimated_departure_time(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetEstimatedDepartureTime();
}

extern "C" bool
es_flightplandata_set_estimated_departure_time(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)
    ->GetFlightPlanData()
    .SetEstimatedDepartureTime(s);
}

extern "C" const char*
es_flightplandata_actual_departure_time(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetActualDepartureTime();
}

extern "C" bool
es_flightplandata_set_actual_departure_time(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetActualDepartureTime(s);
}

extern "C" const char*
es_flightplandata_enroute_hours(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetEnrouteHours();
}

extern "C" bool
es_flightplandata_set_enroute_hours(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetEnrouteHours(s);
}

extern "C" const char*
es_flightplandata_enroute_minutes(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetEnrouteMinutes();
}

extern "C" bool
es_flightplandata_set_enroute_minutes(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetEnrouteMinutes(s);
}

extern "C" const char*
es_flightplandata_fuel_hours(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetFuelHours();
}

extern "C" bool
es_flightplandata_set_fuel_hours(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetFuelHours(s);
}

extern "C" const char*
es_flightplandata_fuel_minutes(void* h)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().GetFuelMinutes();
}

extern "C" bool
es_flightplandata_set_fuel_minutes(void* h, const char* s)
{
  return ES_AS(CFlightPlan, h)->GetFlightPlanData().SetFuelMinutes(s);
}

extern "C" int
es_flightplandata_performance_get_ias(void* h, int altitude, int vertical_speed)
{
  return ES_AS(CFlightPlan, h)
    ->GetFlightPlanData()
    .PerformanceGetIas(altitude, vertical_speed);
}

extern "C" int
es_flightplandata_performance_get_mach(void* h,
                                       int altitude,
                                       int vertical_speed)
{
  return ES_AS(CFlightPlan, h)
    ->GetFlightPlanData()
    .PerformanceGetMach(altitude, vertical_speed);
}

extern "C" int
es_flightplandata_performance_get_climb_rate(void* h, int altitude)
{
  return ES_AS(CFlightPlan, h)
    ->GetFlightPlanData()
    .PerformanceGetClimbRate(altitude);
}

extern "C" int
es_flightplandata_performance_get_descent_rate(void* h, int altitude)
{
  return ES_AS(CFlightPlan, h)
    ->GetFlightPlanData()
    .PerformanceGetDescentRate(altitude);
}
