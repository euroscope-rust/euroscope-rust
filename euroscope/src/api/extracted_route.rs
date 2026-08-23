//! `CFlightPlanExtractedRoute` — the resolved list of route points for a flight
//! plan.

use std::marker::PhantomData;

use euroscope_sys::EsHandle;

use crate::{Position, utils::cstr};

/// The extracted (fully resolved) route of a flight plan.
///
/// A thin handle over EuroScope's `CFlightPlanExtractedRoute`. It is a
/// sub-object of `CFlightPlan` and shares its pointer, so it is valid only for
/// the duration of the callback that delivered the parent flight plan (`'cb`).
///
/// The route is an ordered array of points; most accessors take a point index
/// in `0..points_number()`.
#[derive(Clone, Copy)]
pub struct ExtractedRoute<'cb> {
    raw: EsHandle,
    _marker: PhantomData<&'cb ()>,
}

impl ExtractedRoute<'_> {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw `CFlightPlan*` this route was derived from. Escape hatch for
    /// `euroscope-sys` calls not yet wrapped here.
    #[expect(dead_code)]
    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    /// The number of points in the extracted route array.
    pub fn points_number(&self) -> i32 {
        // SAFETY: `raw` is the parent flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_extractedroute_points_number(self.raw) }
    }

    /// The index of the route edge whose start point is closest to the
    /// aircraft's current position (`0..=points_number()-2`), or `-1` if the
    /// state is invalid.
    pub fn points_calculated_index(&self) -> i32 {
        // SAFETY: `raw` is the parent flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_extractedroute_points_calculated_index(self.raw) }
    }

    /// The index of the point a controller assigned as next (direct to)
    /// (`0..=points_number()-1`), or `-1` if no direct was given.
    pub fn points_assigned_index(&self) -> i32 {
        // SAFETY: `raw` is the parent flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_extractedroute_points_assigned_index(self.raw) }
    }

    /// The name of the point at `index` (must be in `0..points_number()`).
    /// Borrows EuroScope-owned memory; valid for this callback only.
    pub fn point_name(&self, index: i32) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_extractedroute_point_name(self.raw, index)) }
    }

    /// The coordinates of the point at `index` (must be in
    /// `0..points_number()`).
    pub fn point_position(&self, index: i32) -> Position {
        let mut lat = 0.0_f64;
        let mut lon = 0.0_f64;
        // SAFETY: `raw` is the parent flight-plan pointer for this callback;
        // `lat`/`lon` are valid out-params.
        unsafe {
            euroscope_sys::es_extractedroute_point_position(
                self.raw,
                index,
                &raw mut lat,
                &raw mut lon,
            );
        }
        Position::new(lat, lon)
    }

    /// The name of the airway or SID/STAR from the previous point (`index-1`)
    /// to the point at `index` (must be in `1..points_number()`; always empty
    /// for point 0). Borrows EuroScope-owned memory; valid for this callback
    /// only.
    pub fn point_airway_name(&self, index: i32) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe {
            cstr(euroscope_sys::es_extractedroute_point_airway_name(
                self.raw, index,
            ))
        }
    }

    /// The airway classification (`AIRWAY_CLASS_...`) of the leg from the
    /// previous point (`index-1`) to the point at `index` (must be in
    /// `1..points_number()`; always direct-to for point 0).
    pub fn point_airway_classification(&self, index: i32) -> crate::AirwayClass {
        // SAFETY: `raw` is the parent flight-plan pointer for this callback.
        crate::AirwayClass::from_raw(unsafe {
            euroscope_sys::es_extractedroute_point_airway_classification(self.raw, index)
        })
    }

    /// The distance to the point at `index` in minutes from the aircraft's
    /// current position, or `-1` if the point has been passed (must be in
    /// `0..points_number()`).
    pub fn point_distance_in_minutes(&self, index: i32) -> i32 {
        // SAFETY: `raw` is the parent flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_extractedroute_point_distance_in_minutes(self.raw, index) }
    }

    /// The altitude calculated from the route, climb/descend profile and COPX
    /// altitude constraints for the point at `index` (must be in
    /// `0..points_number()`).
    pub fn point_calculated_profile_altitude(&self, index: i32) -> i32 {
        // SAFETY: `raw` is the parent flight-plan pointer for this callback.
        unsafe {
            euroscope_sys::es_extractedroute_point_calculated_profile_altitude(self.raw, index)
        }
    }
}
