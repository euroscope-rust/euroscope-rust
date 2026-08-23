//! `CPosition` FFI declarations (geodesic helpers only; the coordinates
//! themselves are held on the Rust side).

unsafe extern "C" {
    /// Great-circle distance in nautical miles, EuroScope's formula.
    pub fn es_position_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64;

    /// Magnetic heading in degrees from point 1 to point 2.
    pub fn es_position_direction(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64;

    /// Parse EuroScope coordinate strings. Returns success; fills `lat`/`lon`.
    pub fn es_position_from_strings(
        longitude: *const std::ffi::c_char,
        latitude: *const std::ffi::c_char,
        lat: *mut f64,
        lon: *mut f64,
    ) -> bool;
}
