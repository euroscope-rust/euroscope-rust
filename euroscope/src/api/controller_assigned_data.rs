//! `CFlightPlanControllerAssignedData` — the controller-assigned data attached
//! to a flight plan (squawk, cleared/final altitude, assigned speed/heading,
//! scratch pad, flight-strip annotations, …).

use std::marker::PhantomData;

use euroscope_sys::EsHandle;

use crate::{
    CommunicationType,
    utils::{cstr, cstr_lossy},
};

/// The controller-assigned data view of a flight plan.
///
/// A thin handle over EuroScope's `CFlightPlanControllerAssignedData`. It is a
/// sub-object of a `CFlightPlan` and shares that flight plan's pointer, so it is
/// valid only for the duration of the callback that delivered it (`'cb`).
#[derive(Clone, Copy)]
pub struct ControllerAssignedData<'cb> {
    raw: EsHandle,
    _marker: PhantomData<&'cb ()>,
}

impl ControllerAssignedData<'_> {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw owning `CFlightPlan*`. Escape hatch for `euroscope-sys` calls not
    /// yet wrapped here.
    #[expect(dead_code)]
    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    /// The assigned squawk (may be empty). Borrows EuroScope-owned memory; valid
    /// for this callback only.
    pub fn squawk(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or null.
        unsafe { cstr(euroscope_sys::es_assigneddata_squawk(self.raw)) }
    }

    /// Assign a new squawk. Mutates EuroScope state; returns `true` on success.
    pub fn set_squawk(&self, squawk: &str) -> bool {
        // SAFETY: `squawk` is a valid NUL-terminated string for the call.
        unsafe { euroscope_sys::es_assigneddata_set_squawk(self.raw, cstr_lossy(squawk).as_ptr()) }
    }

    /// The controller-overridden final altitude (no ALT/FL change).
    pub fn final_altitude(&self) -> i32 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_final_altitude(self.raw) }
    }

    /// Set the final altitude. Mutates EuroScope state; returns `true` on success.
    pub fn set_final_altitude(&self, final_altitude: i32) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_set_final_altitude(self.raw, final_altitude) }
    }

    /// The cleared altitude. Special values: `0` = none (use final), `1` =
    /// cleared for ILS approach, `2` = cleared for visual approach.
    pub fn cleared_altitude(&self) -> i32 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_cleared_altitude(self.raw) }
    }

    /// Set the cleared altitude (see [`cleared_altitude`](Self::cleared_altitude)
    /// for special values). Mutates EuroScope state; returns `true` on success.
    pub fn set_cleared_altitude(&self, cleared_altitude: i32) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_set_cleared_altitude(self.raw, cleared_altitude) }
    }

    /// The controller-assigned communications type.
    pub fn communication_type(&self) -> CommunicationType {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        CommunicationType::from_raw(unsafe {
            euroscope_sys::es_assigneddata_communication_type(self.raw)
        })
    }

    /// Set the communications type. Mutates EuroScope state; returns `true` on
    /// success.
    pub fn set_communication_type(&self, communication_type: CommunicationType) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe {
            euroscope_sys::es_assigneddata_set_communication_type(
                self.raw,
                communication_type.to_raw(),
            )
        }
    }

    /// The scratch pad string value. Borrows EuroScope-owned memory; valid for
    /// this callback only.
    pub fn scratch_pad_string(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or null.
        unsafe { cstr(euroscope_sys::es_assigneddata_scratch_pad_string(self.raw)) }
    }

    /// Set the scratch pad string. Mutates EuroScope state; returns `true` on success.
    pub fn set_scratch_pad_string(&self, string: &str) -> bool {
        // SAFETY: `string` is a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_assigneddata_set_scratch_pad_string(
                self.raw,
                cstr_lossy(string).as_ptr(),
            )
        }
    }

    /// The controller-assigned speed. `0` indicates no assignment.
    pub fn assigned_speed(&self) -> i32 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_assigned_speed(self.raw) }
    }

    /// Set the assigned speed. Mutates EuroScope state; returns `true` on success.
    pub fn set_assigned_speed(&self, assigned_speed: i32) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_set_assigned_speed(self.raw, assigned_speed) }
    }

    /// The controller-assigned Mach number multiplied by 100 (750 = Mach 0.75).
    /// `0` indicates no assignment.
    pub fn assigned_mach(&self) -> i32 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_assigned_mach(self.raw) }
    }

    /// Set the assigned Mach number (multiplied by 100). Mutates EuroScope state;
    /// returns `true` on success.
    pub fn set_assigned_mach(&self, assigned_mach: i32) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_set_assigned_mach(self.raw, assigned_mach) }
    }

    /// The controller-assigned climb/descend rate. `0` indicates no assignment.
    pub fn assigned_rate(&self) -> i32 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_assigned_rate(self.raw) }
    }

    /// Set the assigned rate. Mutates EuroScope state; returns `true` on success.
    pub fn set_assigned_rate(&self, assigned_rate: i32) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_set_assigned_rate(self.raw, assigned_rate) }
    }

    /// The controller-assigned heading. `0` indicates no assignment.
    pub fn assigned_heading(&self) -> i32 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_assigned_heading(self.raw) }
    }

    /// Set the assigned heading. Mutates EuroScope state; returns `true` on success.
    pub fn set_assigned_heading(&self, assigned_heading: i32) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_assigneddata_set_assigned_heading(self.raw, assigned_heading) }
    }

    /// The assigned direct-to point name. Borrows EuroScope-owned memory; valid
    /// for this callback only.
    pub fn direct_to_point_name(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or null.
        unsafe {
            cstr(euroscope_sys::es_assigneddata_direct_to_point_name(
                self.raw,
            ))
        }
    }

    /// Set the direct-to point name. Mutates EuroScope state; returns `true` on success.
    pub fn set_direct_to_point_name(&self, point_name: &str) -> bool {
        // SAFETY: `point_name` is a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_assigneddata_set_direct_to_point_name(
                self.raw,
                cstr_lossy(point_name).as_ptr(),
            )
        }
    }

    /// The flight-strip annotation at `index` (0-8). Borrows EuroScope-owned
    /// memory; valid for this callback only.
    pub fn flight_strip_annotation(&self, index: i32) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or null.
        unsafe {
            cstr(euroscope_sys::es_assigneddata_flight_strip_annotation(
                self.raw, index,
            ))
        }
    }

    /// Set the flight-strip annotation at `index` (0-8). Mutates EuroScope state;
    /// returns `true` on success.
    pub fn set_flight_strip_annotation(&self, index: i32, annotation: &str) -> bool {
        // SAFETY: `annotation` is a valid NUL-terminated string for the call.
        unsafe {
            euroscope_sys::es_assigneddata_set_flight_strip_annotation(
                self.raw,
                index,
                cstr_lossy(annotation).as_ptr(),
            )
        }
    }
}
