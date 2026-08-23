// Rust -> EuroScope wrappers for CRadarTargetPositionData.
//
// This is a sub-object of CRadarTarget: the handle passed in is an owned
// CRadarTargetPositionData*, and every wrapper reads the snapshot directly;
// ownership is freed via es_rtposdata_free.

#include "common.h"

extern "C" bool
es_rtpos_is_valid(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->IsValid();
}

extern "C" bool
es_rtpos_is_fp_track_position(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->IsFPTrackPosition();
}

extern "C" int
es_rtpos_received_time(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetReceivedTime();
}

extern "C" void
es_rtpos_position(void* h, double* lat, double* lon)
{
  CPosition p = ES_AS(CRadarTargetPositionData, h)->GetPosition();
  *lat = p.m_Latitude;
  *lon = p.m_Longitude;
}

extern "C" const char*
es_rtpos_squawk(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetSquawk();
}

extern "C" bool
es_rtpos_transponder_c(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetTransponderC();
}

extern "C" bool
es_rtpos_transponder_i(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetTransponderI();
}

extern "C" int
es_rtpos_pressure_altitude(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetPressureAltitude();
}

extern "C" int
es_rtpos_flight_level(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetFlightLevel();
}

extern "C" int
es_rtpos_reported_gs(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetReportedGS();
}

extern "C" int
es_rtpos_reported_heading(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetReportedHeading();
}

extern "C" int
es_rtpos_reported_heading_true_north(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetReportedHeadingTrueNorth();
}

extern "C" int
es_rtpos_reported_pitch(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetReportedPitch();
}

extern "C" int
es_rtpos_reported_bank(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetReportedBank();
}

extern "C" int
es_rtpos_radar_flags(void* h)
{
  return ES_AS(CRadarTargetPositionData, h)->GetRadarFlags();
}

extern "C" void
es_rtposdata_free(void* h)
{
  delete ES_AS(CRadarTargetPositionData, h);
}
