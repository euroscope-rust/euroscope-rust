// Rust -> EuroScope wrappers for CFlightPlanControllerAssignedData.
//
// This is a sub-object of CFlightPlan: the handle passed in is a CFlightPlan*,
// and every wrapper recovers the controller-assigned-data view from it.

#include "common.h"

extern "C" const char*
es_assigneddata_squawk(void* h)
{
  return ES_AS(CFlightPlan, h)->GetControllerAssignedData().GetSquawk();
}

extern "C" bool
es_assigneddata_set_squawk(void* h, const char* sSquawk)
{
  return ES_AS(CFlightPlan, h)->GetControllerAssignedData().SetSquawk(sSquawk);
}

extern "C" int
es_assigneddata_final_altitude(void* h)
{
  return ES_AS(CFlightPlan, h)->GetControllerAssignedData().GetFinalAltitude();
}

extern "C" bool
es_assigneddata_set_final_altitude(void* h, int FinalAltitude)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetFinalAltitude(FinalAltitude);
}

extern "C" int
es_assigneddata_cleared_altitude(void* h)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .GetClearedAltitude();
}

extern "C" bool
es_assigneddata_set_cleared_altitude(void* h, int ClearedAltitude)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetClearedAltitude(ClearedAltitude);
}

extern "C" char
es_assigneddata_communication_type(void* h)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .GetCommunicationType();
}

extern "C" bool
es_assigneddata_set_communication_type(void* h, char CommunicationType)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetCommunicationType(CommunicationType);
}

extern "C" const char*
es_assigneddata_scratch_pad_string(void* h)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .GetScratchPadString();
}

extern "C" bool
es_assigneddata_set_scratch_pad_string(void* h, const char* sString)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetScratchPadString(sString);
}

extern "C" int
es_assigneddata_assigned_speed(void* h)
{
  return ES_AS(CFlightPlan, h)->GetControllerAssignedData().GetAssignedSpeed();
}

extern "C" bool
es_assigneddata_set_assigned_speed(void* h, int AssignedSpeed)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetAssignedSpeed(AssignedSpeed);
}

extern "C" int
es_assigneddata_assigned_mach(void* h)
{
  return ES_AS(CFlightPlan, h)->GetControllerAssignedData().GetAssignedMach();
}

extern "C" bool
es_assigneddata_set_assigned_mach(void* h, int AssignedMach)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetAssignedMach(AssignedMach);
}

extern "C" int
es_assigneddata_assigned_rate(void* h)
{
  return ES_AS(CFlightPlan, h)->GetControllerAssignedData().GetAssignedRate();
}

extern "C" bool
es_assigneddata_set_assigned_rate(void* h, int AssignedRate)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetAssignedRate(AssignedRate);
}

extern "C" int
es_assigneddata_assigned_heading(void* h)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .GetAssignedHeading();
}

extern "C" bool
es_assigneddata_set_assigned_heading(void* h, int AssignedHeading)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetAssignedHeading(AssignedHeading);
}

extern "C" const char*
es_assigneddata_direct_to_point_name(void* h)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .GetDirectToPointName();
}

extern "C" bool
es_assigneddata_set_direct_to_point_name(void* h, const char* sPointName)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetDirectToPointName(sPointName);
}

extern "C" const char*
es_assigneddata_flight_strip_annotation(void* h, int Index)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .GetFlightStripAnnotation(Index);
}

extern "C" bool
es_assigneddata_set_flight_strip_annotation(void* h,
                                            int Index,
                                            const char* sAnnotation)
{
  return ES_AS(CFlightPlan, h)
    ->GetControllerAssignedData()
    .SetFlightStripAnnotation(Index, sAnnotation);
}
