// Rust -> EuroScope wrappers for CPosition.
//
// CPosition is a value type (two public doubles + geodesic helpers), not a
// handle. Rust holds the lat/lon itself; these wrappers only expose
// EuroScope's distance/bearing formulas, which are computed inside EuroScope.

#include "common.h"

static CPosition
make(double lat, double lon)
{
  CPosition p;
  p.m_Latitude = lat;
  p.m_Longitude = lon;
  return p;
}

extern "C" double
es_position_distance(double lat1, double lon1, double lat2, double lon2)
{
  return make(lat1, lon1).DistanceTo(make(lat2, lon2));
}

extern "C" double
es_position_direction(double lat1, double lon1, double lat2, double lon2)
{
  return make(lat1, lon1).DirectionTo(make(lat2, lon2));
}

// Parse EuroScope coordinate strings ("N047.25.24.615" / "E019.17.35.260").
// Returns whether parsing succeeded; on success fills *lat / *lon.
extern "C" bool
es_position_from_strings(const char* longitude,
                         const char* latitude,
                         double* lat,
                         double* lon)
{
  CPosition p;
  // NOTE: LoadFromStrings takes (longitude, latitude) in that order.
  if (!p.LoadFromStrings(longitude, latitude)) {
    return false;
  }
  *lat = p.m_Latitude;
  *lon = p.m_Longitude;
  return true;
}
