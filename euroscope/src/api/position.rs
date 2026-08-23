//! `CPosition` — a geographic coordinate.

/// A geographic position (latitude/longitude in decimal degrees).
///
/// Unlike the other API types this is an **owned value**, not a borrowed
/// handle: EuroScope's `CPosition` is a plain lat/lon pair, so we copy it out.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// Latitude in decimal degrees (north positive).
    pub latitude: f64,
    /// Longitude in decimal degrees (east positive).
    pub longitude: f64,
}

impl Position {
    /// Construct from decimal degrees.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Parse EuroScope coordinate strings, e.g.
    /// `Position::from_strings("E019.17.35.260", "N047.25.24.615")`. Note the
    /// **longitude-then-latitude** argument order (matching the SDK). Returns
    /// `None` if either string is malformed.
    pub fn from_strings(longitude: &str, latitude: &str) -> Option<Self> {
        let lon_c = std::ffi::CString::new(longitude).ok()?;
        let lat_c = std::ffi::CString::new(latitude).ok()?;
        let (mut lat, mut lon) = (0.0_f64, 0.0_f64);
        // SAFETY: valid NUL-terminated strings and out-param slots.
        let ok = unsafe {
            euroscope_sys::es_position_from_strings(
                lon_c.as_ptr(),
                lat_c.as_ptr(),
                &raw mut lat,
                &raw mut lon,
            )
        };
        ok.then_some(Self {
            latitude: lat,
            longitude: lon,
        })
    }

    /// Great-circle distance to another position, in nautical miles, using
    /// EuroScope's formula.
    pub fn distance_to(self, other: Self) -> f64 {
        // SAFETY: pure computation, no pointers involved.
        unsafe {
            euroscope_sys::es_position_distance(
                self.latitude,
                self.longitude,
                other.latitude,
                other.longitude,
            )
        }
    }

    /// Magnetic heading (degrees) from this position to another, corrected by
    /// the sector file's magnetic variation.
    pub fn direction_to(self, other: Self) -> f64 {
        // SAFETY: pure computation, no pointers involved.
        unsafe {
            euroscope_sys::es_position_direction(
                self.latitude,
                self.longitude,
                other.latitude,
                other.longitude,
            )
        }
    }
}
