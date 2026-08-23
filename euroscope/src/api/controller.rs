//! `CController` — a reference to an online controller, plus the owned
//! "myself" snapshot.

use std::marker::PhantomData;

use euroscope_sys::EsHandle;

use crate::{
    Context, ControllerRating, Facility, Position,
    api::handle::{iter, scoped},
    utils::{cstr, cstr_lossy},
};

/// A reference to an online controller.
///
/// A thin handle over EuroScope's `CController`, valid only for the callback /
/// query block that produced it (`'cb`).
#[derive(Clone, Copy)]
pub struct Controller<'cb> {
    raw: EsHandle,
    _marker: PhantomData<&'cb ()>,
}

impl Controller<'_> {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw `CController*`. Escape hatch for `euroscope-sys` calls not yet
    /// wrapped here.
    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    /// Whether this handle references a valid controller reference.
    pub fn is_valid(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_controller_is_valid(self.raw) }
    }

    /// The controller callsign, e.g. `"LSGG_APP"`. Empty if invalid. Borrows
    /// EuroScope-owned memory; valid for this callback only.
    pub fn callsign(&self) -> &str {
        // SAFETY: the SDK returns a borrowed NUL-terminated ANSI string, or
        // null (guarded).
        unsafe { cstr(euroscope_sys::es_controller_callsign(self.raw)) }
    }

    /// The position identifier of the selected controller, e.g. `"GG"`.
    /// Borrows EuroScope-owned memory; valid for this callback only.
    pub fn position_id(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_controller_position_id(self.raw)) }
    }

    /// Whether the controller is identified in the position file.
    pub fn position_identified(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_controller_position_identified(self.raw) }
    }

    /// The primary frequency of the controller in MHz, e.g. `121.855`.
    /// Returns `199.980` if no primary frequency is selected.
    pub fn primary_frequency(&self) -> f64 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_controller_primary_frequency(self.raw) }
    }

    /// The full name of the controller. Borrows EuroScope-owned memory; valid
    /// for this callback only.
    pub fn full_name(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_controller_full_name(self.raw)) }
    }

    /// The network rating of the controller.
    pub fn rating(&self) -> ControllerRating {
        // SAFETY: `raw` is valid for this callback.
        ControllerRating::from_raw(unsafe { euroscope_sys::es_controller_rating(self.raw) })
    }

    /// The kind of position occupied by the controller.
    pub fn facility(&self) -> Facility {
        // SAFETY: `raw` is valid for this callback.
        Facility::from_raw(unsafe { euroscope_sys::es_controller_facility(self.raw) })
    }

    /// The name of the sector file used by the controller. Borrows
    /// EuroScope-owned memory; valid for this callback only.
    pub fn sector_file_name(&self) -> &str {
        // SAFETY: borrowed NUL-terminated ANSI string, or null (guarded).
        unsafe { cstr(euroscope_sys::es_controller_sector_file_name(self.raw)) }
    }

    /// Whether the controller is accepted as a controller by the servers (has
    /// the right to track aircraft, change flight plans, etc.).
    pub fn is_controller(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_controller_is_controller(self.raw) }
    }

    /// The center position of the controller (the main center only, never the
    /// additional visibility points).
    pub fn position(&self) -> Position {
        let mut lat = 0.0_f64;
        let mut lon = 0.0_f64;
        // SAFETY: `raw` is valid for this callback; the out-params are valid
        // stack slots the shim writes before returning.
        unsafe {
            euroscope_sys::es_controller_position(self.raw, &raw mut lat, &raw mut lon);
        }
        Position::new(lat, lon)
    }

    /// The visibility range of the controller, in nautical miles.
    pub fn range(&self) -> i32 {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_controller_range(self.raw) }
    }

    /// The breaking state of the controller.
    pub fn is_breaking(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_controller_is_breaking(self.raw) }
    }

    /// Whether the controller is ready for ongoing coordination (indicates
    /// EuroScope on the other side).
    pub fn is_ongoing_able(&self) -> bool {
        // SAFETY: `raw` is valid for this callback.
        unsafe { euroscope_sys::es_controller_is_ongoing_able(self.raw) }
    }
}

scoped!(ControllerHandle, Controller, es_controller_free);
iter!(
    Controllers,
    ControllerHandle,
    Controller,
    es_controller_free,
    es_plugin_controller_select_first,
    es_plugin_controller_select_next
);

/// Controller selection and iteration.
impl Context {
    /// Iterate over all online controllers.
    pub fn controllers(&self) -> Controllers<'_> {
        // SAFETY: `Context` holds a live CPlugIn* for the plugin's lifetime.
        Controllers::new(unsafe { self.as_ptr() })
    }

    /// Select a controller by callsign.
    pub fn controller(&self, callsign: &str) -> Option<ControllerHandle<'_>> {
        // SAFETY: live CPlugIn*; the CString outlives the call.
        ControllerHandle::from_owned(unsafe {
            euroscope_sys::es_plugin_controller_select(self.as_ptr(), cstr_lossy(callsign).as_ptr())
        })
    }

    /// Select a controller by position ID.
    pub fn controller_by_position_id(&self, position_id: &str) -> Option<ControllerHandle<'_>> {
        // SAFETY: live CPlugIn*; the CString outlives the call.
        ControllerHandle::from_owned(unsafe {
            euroscope_sys::es_plugin_controller_select_by_position_id(
                self.as_ptr(),
                cstr_lossy(position_id).as_ptr(),
            )
        })
    }

    /// The logged-in controller ("myself"), or `None` when not connected.
    pub fn controller_myself(&self) -> Option<ControllerHandle<'_>> {
        // SAFETY: live CPlugIn*.
        ControllerHandle::from_owned(unsafe {
            euroscope_sys::es_plugin_controller_myself(self.as_ptr())
        })
    }
}
