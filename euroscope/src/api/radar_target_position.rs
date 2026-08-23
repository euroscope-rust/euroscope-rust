//! `CRadarTargetPositionData` — a single radar/track position snapshot of an
//! aircraft.

use std::marker::PhantomData;

use euroscope_sys::EsHandle;

use crate::{api::handle::scoped, utils::cstr};

/// A radar target position snapshot.
///
/// A thin handle over EuroScope's `CRadarTargetPositionData`. It is reached
/// through a `CRadarTarget`, so the underlying pointer is the radar target's;
/// like the other handles it is valid only for the duration of the callback /
/// query block that produced it (`'cb`).
#[derive(Clone, Copy)]
pub struct RadarTargetPosition<'cb> {
    raw: EsHandle,
    _marker: PhantomData<&'cb ()>,
}

impl RadarTargetPosition<'_> {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw `CRadarTarget*` this snapshot reads from. Escape hatch for
    /// `euroscope-sys` calls not yet wrapped here.
    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    /// Whether the position reference is valid.
    pub fn is_valid(&self) -> bool {
        // SAFETY: `raw` is the radar-target pointer EuroScope handed us.
        unsafe { euroscope_sys::es_rtpos_is_valid(self.raw) }
    }

    /// Whether the position is a reference to the flight-plan track.
    pub fn is_fp_track_position(&self) -> bool {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_is_fp_track_position(self.raw) }
    }

    /// Seconds elapsed since this position data was received.
    pub fn received_time(&self) -> i32 {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_received_time(self.raw) }
    }

    /// The lat/lon coordinates of the plane.
    pub fn position(&self) -> crate::Position {
        let mut lat = 0.0_f64;
        let mut lon = 0.0_f64;
        // SAFETY: `raw` is valid; the out-params are live stack locals.
        unsafe {
            euroscope_sys::es_rtpos_position(self.raw, &raw mut lat, &raw mut lon);
        }
        crate::Position::new(lat, lon)
    }

    /// The squawk sent by the pilot. Borrows EuroScope-owned memory; valid for
    /// this callback only.
    pub fn squawk(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_rtpos_squawk(self.raw)) }
    }

    /// Whether the plane transponder is in C mode.
    pub fn transponder_c(&self) -> bool {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_transponder_c(self.raw) }
    }

    /// Whether the plane transponder is in IDENT mode.
    pub fn transponder_i(&self) -> bool {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_transponder_i(self.raw) }
    }

    /// The true altitude of the plane, in feet.
    pub fn pressure_altitude(&self) -> i32 {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_pressure_altitude(self.raw) }
    }

    /// The altitude calculated using standard pressure, in feet.
    pub fn flight_level(&self) -> i32 {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_flight_level(self.raw) }
    }

    /// The ground speed reported by the pilot client.
    pub fn reported_gs(&self) -> i32 {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_reported_gs(self.raw) }
    }

    /// The heading (not tracking) reported by the pilot client.
    pub fn reported_heading(&self) -> i32 {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_reported_heading(self.raw) }
    }

    /// The reported heading referenced to true north.
    pub fn reported_heading_true_north(&self) -> i32 {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_reported_heading_true_north(self.raw) }
    }

    /// The pitch reported by the pilot client (-180..+180; negative is above
    /// horizon).
    pub fn reported_pitch(&self) -> i32 {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_reported_pitch(self.raw) }
    }

    /// The bank reported by the pilot client (-180..+180; negative is right
    /// bank).
    pub fn reported_bank(&self) -> i32 {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_reported_bank(self.raw) }
    }

    /// The radar response flags: an OR of the `RADAR_POSITION_*` values
    /// (primary, secondary C, secondary S).
    pub fn radar_flags(&self) -> i32 {
        // SAFETY: see `is_valid`.
        unsafe { euroscope_sys::es_rtpos_radar_flags(self.raw) }
    }
}

scoped!(
    RadarTargetPositionHandle,
    RadarTargetPosition,
    es_rtposdata_free
);

/// Iterator over a radar target's position history (newest first, up to the ~20
/// snapshots EuroScope keeps), yielding callback-scoped position snapshots.
pub struct PositionHistory<'cb> {
    radar_target: EsHandle,
    next: Option<EsHandle>,
    _marker: PhantomData<&'cb ()>,
}

impl PositionHistory<'_> {
    pub(crate) fn new(radar_target: EsHandle) -> Self {
        // SAFETY: `radar_target` is a live CRadarTarget*.
        let first = unsafe { euroscope_sys::es_radartarget_current_position(radar_target) };
        Self {
            radar_target,
            next: (!first.is_null()).then_some(first),
            _marker: PhantomData,
        }
    }
}

#[expect(
    clippy::missing_trait_methods,
    reason = "We don't need the extra methods for this type"
)]
impl<'cb> Iterator for PositionHistory<'cb> {
    type Item = RadarTargetPositionHandle<'cb>;

    fn next(&mut self) -> Option<RadarTargetPositionHandle<'cb>> {
        let cur = self.next.take()?;
        // GetPreviousPosition needs `cur` alive; it is (about to be yielded).
        // SAFETY: `radar_target`/`cur` are live; shim returns a heap copy or null.
        let following =
            unsafe { euroscope_sys::es_radartarget_previous_position(self.radar_target, cur) };
        self.next = (!following.is_null()).then_some(following);
        Some(RadarTargetPositionHandle {
            inner: RadarTargetPosition::from_raw(cur),
        })
    }
}

#[expect(clippy::missing_trait_methods, reason = "We don't need pin_drop")]
impl Drop for PositionHistory<'_> {
    fn drop(&mut self) {
        if let Some(p) = self.next.take() {
            // SAFETY: `p` is an owned position snapshot from the shim.
            unsafe { euroscope_sys::es_rtposdata_free(p) }
        }
    }
}
