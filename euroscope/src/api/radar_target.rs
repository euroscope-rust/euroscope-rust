//! `CRadarTarget` — a reference to a live aircraft radar track.

use std::marker::PhantomData;

use euroscope_sys::EsHandle;

use crate::{
    Context, FlightPlan,
    api::handle::{iter, scoped},
    utils::{cstr, cstr_lossy},
};

/// A reference to a radar target (a tracked aircraft).
///
/// A thin handle over EuroScope's `CRadarTarget`, valid only for the duration of
/// the callback / query block that delivered it (`'cb`).
#[derive(Clone, Copy)]
pub struct RadarTarget<'cb> {
    raw: EsHandle,
    _marker: PhantomData<&'cb ()>,
}

impl RadarTarget<'_> {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw `CRadarTarget*`. Escape hatch for `euroscope-sys` calls not yet
    /// wrapped here, and used to build sibling handles that share this pointer.
    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    /// Whether this handle references a live aircraft.
    pub fn is_valid(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_radartarget_is_valid(self.raw) }
    }

    /// The aircraft callsign. Empty if the handle is invalid. Borrows
    /// EuroScope-owned memory; valid for this callback only.
    pub fn callsign(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_radartarget_callsign(self.raw)) }
    }

    /// The system-assigned target ID. Stable across all instances since it is
    /// generated from the callsign. Borrows EuroScope-owned memory.
    pub fn system_id(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_radartarget_system_id(self.raw)) }
    }

    /// Calculated vertical speed in feet/minute. Protocol-inaccurate; use only
    /// for climb/descend/level tests.
    pub fn vertical_speed(&self) -> i32 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_radartarget_vertical_speed(self.raw) }
    }

    /// Calculated track direction in degrees.
    pub fn track_heading(&self) -> f64 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_radartarget_track_heading(self.raw) }
    }

    /// Ground speed in knots. Uses the plane-reported value, falling back to
    /// the calculated one when that is zero.
    pub fn gs(&self) -> i32 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_radartarget_gs(self.raw) }
    }

    /// Correlate this radar target with `flight_plan`, returning whether it
    /// succeeded. **Mutates EuroScope state**; afterwards you need not call the
    /// reciprocal correlate on the flight plan.
    pub fn correlate_with_flight_plan(&self, flight_plan: FlightPlan<'_>) -> bool {
        // SAFETY: both `raw` and the flight-plan pointer are handles EuroScope
        // handed us for this callback; the SDK copies the (pointer-sized)
        // flight-plan handle by value.
        unsafe {
            euroscope_sys::es_radartarget_correlate_with_flight_plan(self.raw, flight_plan.as_ptr())
        }
    }

    /// Uncorrelate this radar target from its flight plan. **Mutates EuroScope
    /// state**; afterwards the system will never re-correlate it automatically.
    pub fn uncorrelate(&self) {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_radartarget_uncorrelate(self.raw) }
    }

    /// The latest position snapshot of this target. `None` if invalid.
    pub fn position(&self) -> Option<crate::RadarTargetPositionHandle<'_>> {
        // SAFETY: `raw` valid for this callback; shim returns a heap copy or null.
        crate::RadarTargetPositionHandle::from_owned(unsafe {
            euroscope_sys::es_radartarget_current_position(self.as_ptr())
        })
    }

    /// Walk this target's position history, newest first (up to ~20 snapshots).
    pub fn position_history(&self) -> crate::PositionHistory<'_> {
        crate::PositionHistory::new(self.as_ptr())
    }

    /// The flight plan correlated with this radar target, if any.
    pub fn correlated_flight_plan(&self) -> Option<crate::FlightPlanHandle<'_>> {
        // SAFETY: `raw` valid for this callback; shim returns a heap copy or null.
        crate::FlightPlanHandle::from_owned(unsafe {
            euroscope_sys::es_radartarget_correlated_flight_plan(self.as_ptr())
        })
    }
}

scoped!(RadarTargetHandle, RadarTarget, es_radartarget_free);
iter!(
    RadarTargets,
    RadarTargetHandle,
    RadarTarget,
    es_radartarget_free,
    es_plugin_radartarget_select_first,
    es_plugin_radartarget_select_next
);

/// Radar-target selection and iteration.
impl Context {
    /// Iterate over all radar targets.
    pub fn radar_targets(&self) -> RadarTargets<'_> {
        // SAFETY: `Context` holds a live CPlugIn* for the plugin's lifetime.
        RadarTargets::new(unsafe { self.as_ptr() })
    }

    /// Select a radar target by callsign.
    pub fn radar_target(&self, callsign: &str) -> Option<RadarTargetHandle<'_>> {
        // SAFETY: live CPlugIn*; the CString outlives the call.
        RadarTargetHandle::from_owned(unsafe {
            euroscope_sys::es_plugin_radartarget_select(
                self.as_ptr(),
                cstr_lossy(callsign).as_ptr(),
            )
        })
    }

    /// The currently ASEL (selected) radar target.
    pub fn selected_radar_target(&self) -> Option<RadarTargetHandle<'_>> {
        // SAFETY: live CPlugIn*.
        RadarTargetHandle::from_owned(unsafe {
            euroscope_sys::es_plugin_radartarget_select_asel(self.as_ptr())
        })
    }

    /// Set the ASEL (selected) aircraft to the given radar target.
    pub fn set_asel_radar_target(&self, radar_target: RadarTarget<'_>) {
        // SAFETY: live CPlugIn*; the handle is valid for this call.
        unsafe {
            euroscope_sys::es_plugin_set_asel_radartarget(self.as_ptr(), radar_target.as_ptr());
        }
    }
}
