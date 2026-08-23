//! `CFlightPlanPositionPredictions` — the predicted future track of a flight
//! plan.

use std::marker::PhantomData;

use euroscope_sys::EsHandle;

use crate::{Position, utils::cstr};

/// A reference to a flight plan's position predictions.
///
/// A thin handle over EuroScope's `CFlightPlanPositionPredictions`. It is a
/// sub-object of `CFlightPlan` and shares that handle's pointer, so it is valid
/// only for the duration of the callback that delivered the flight plan
/// (`'cb`).
///
/// The prediction array is indexed by minutes from now: index `0` is the
/// current position, index `1` the position one minute ahead, and so on up to
/// [`points_number`](Self::points_number) `- 1`.
#[derive(Clone, Copy)]
pub struct PositionPredictions<'cb> {
    raw: EsHandle,
    _marker: PhantomData<&'cb ()>,
}

impl PositionPredictions<'_> {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw owning `CFlightPlan*`. Escape hatch for `euroscope-sys` calls
    /// not yet wrapped here.
    #[expect(dead_code)]
    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    /// The number of points in the prediction array. Valid indices for the
    /// other accessors are `0..points_number`.
    pub fn points_number(&self) -> i32 {
        // SAFETY: `raw` is the flight-plan pointer EuroScope handed us for this
        // callback.
        unsafe { euroscope_sys::es_positionpredictions_points_number(self.raw) }
    }

    /// The predicted position of the aircraft `index` minutes from now.
    ///
    /// `index` must be in `0..points_number`.
    pub fn position(&self, index: i32) -> Position {
        let mut lat = 0.0_f64;
        let mut lon = 0.0_f64;
        // SAFETY: `raw` is the flight-plan pointer for this callback; the
        // out-params are valid local pointers.
        unsafe {
            euroscope_sys::es_positionpredictions_position(
                self.raw,
                index,
                &raw mut lat,
                &raw mut lon,
            );
        }
        Position::new(lat, lon)
    }

    /// The predicted altitude/level of the aircraft `index` minutes from now.
    /// There is no ALT/FL conversion applied.
    ///
    /// `index` must be in `0..points_number`.
    pub fn altitude(&self, index: i32) -> i32 {
        // SAFETY: `raw` is the flight-plan pointer for this callback.
        unsafe { euroscope_sys::es_positionpredictions_altitude(self.raw, index) }
    }

    /// The position ID of the predicted controller `index` minutes from now.
    /// Empty if unavailable. Borrows EuroScope-owned memory; valid for this
    /// callback only.
    ///
    /// `index` must be in `0..points_number`.
    pub fn controller_id(&self, index: i32) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe {
            cstr(euroscope_sys::es_positionpredictions_controller_id(
                self.raw, index,
            ))
        }
    }
}
