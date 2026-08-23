//! `CFlightPlanList` — a EuroScope aircraft-list ("FP list") panel.

use std::{marker::PhantomData, ops::Deref};

use euroscope_sys::EsHandle;

use crate::{Context, FlightPlan, utils::cstr_lossy};

/// A reference to a EuroScope flight-plan list (the configurable "AC list"
/// panel).
///
/// A thin handle over EuroScope's `CFlightPlanList`, valid only for the
/// duration of the callback / query block that produced it (`'cb`).
#[derive(Clone, Copy)]
pub struct FlightPlanList<'cb> {
    raw: EsHandle,
    _marker: PhantomData<&'cb ()>,
}

impl FlightPlanList<'_> {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// The raw `CFlightPlanList*`. Escape hatch for `euroscope-sys` calls not
    /// yet wrapped here.
    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    /// Whether this handle references a live list.
    pub fn is_valid(&self) -> bool {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_fplist_is_valid(self.raw) }
    }

    /// The number of columns defined so far. Handy to test whether the list was
    /// already populated from the settings file before adding new columns.
    pub fn column_number(&self) -> i32 {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_fplist_column_number(self.raw) }
    }

    /// Delete all previous column definitions. Mutates EuroScope state.
    pub fn delete_all_columns(&self) {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_fplist_delete_all_columns(self.raw) }
    }

    /// Add one column definition to the list. Mutates EuroScope state.
    ///
    /// Call this immediately after registering the list; definitions added
    /// later — or any calls at all when the settings file already describes the
    /// list — are ignored. Pass `""` for `item_provider` to use EuroScope's
    /// built-in TAG items.
    #[expect(clippy::too_many_arguments)]
    pub fn add_column_definition(
        &self,
        column_title: &str,
        width: i32,
        centered: bool,
        item_provider: &str,
        item_code: i32,
        left_button_function_provider: &str,
        left_button_function: i32,
        right_button_function_provider: &str,
        right_button_function: i32,
    ) {
        // SAFETY: `raw` is live for this callback; the `CString`s outlive the
        // call (dropped at the end of this statement).
        unsafe {
            euroscope_sys::es_fplist_add_column_definition(
                self.raw,
                cstr_lossy(column_title).as_ptr(),
                width,
                centered,
                cstr_lossy(item_provider).as_ptr(),
                item_code,
                cstr_lossy(left_button_function_provider).as_ptr(),
                left_button_function,
                cstr_lossy(right_button_function_provider).as_ptr(),
                right_button_function,
            );
        }
    }

    /// Add a flight plan to the list. Mutates EuroScope state.
    pub fn add_fp_to_the_list(&self, flight_plan: &FlightPlan<'_>) {
        // SAFETY: both handles are live for this callback.
        unsafe { euroscope_sys::es_fplist_add_fp_to_the_list(self.raw, flight_plan.as_ptr()) }
    }

    /// Remove a flight plan from the list. Mutates EuroScope state.
    pub fn remove_fp_from_the_list(&self, flight_plan: &FlightPlan<'_>) {
        // SAFETY: both handles are live for this callback.
        unsafe { euroscope_sys::es_fplist_remove_fp_from_the_list(self.raw, flight_plan.as_ptr()) }
    }

    /// Show or hide the flight-plan list. Mutates EuroScope state.
    pub fn show_fp_list(&self, show: bool) {
        // SAFETY: `raw` is the pointer EuroScope handed us for this callback.
        unsafe { euroscope_sys::es_fplist_show_fp_list(self.raw, show) }
    }
}

/// A flight-plan list you registered via [`Context::register_fp_list`].
///
/// Unlike the callback-scoped `*Handle` types, this one is meant to be **kept**:
/// EuroScope keeps the registered list alive for the plugin's lifetime, so you
/// store it in your plugin and use it across callbacks to add columns and feed
/// aircraft. It owns its heap copy of the handle and frees it on drop, and
/// derefs to the borrowed [`FlightPlanList`] so every method is available.
pub struct RegisteredFlightPlanList {
    inner: FlightPlanList<'static>,
}

impl RegisteredFlightPlanList {
    fn from_owned(raw: EsHandle) -> Option<Self> {
        (!raw.is_null()).then(|| Self {
            inner: FlightPlanList::from_raw(raw),
        })
    }
}

impl Deref for RegisteredFlightPlanList {
    type Target = FlightPlanList<'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[expect(clippy::missing_trait_methods, reason = "We don't need pin_drop")]
impl Drop for RegisteredFlightPlanList {
    fn drop(&mut self) {
        // SAFETY: `inner` owns a heap handle the shim allocated for us.
        unsafe { euroscope_sys::es_fplist_free(self.inner.as_ptr()) }
    }
}

/// Flight-plan list registration.
impl Context {
    /// Register a new flight-plan list (appears in EuroScope) and return a
    /// persistent handle. Keep it in your plugin to add columns and feed
    /// aircraft; [`Plugin::on_refresh_fp_list`](crate::Plugin::on_refresh_fp_list)
    /// fires when it needs content.
    pub fn register_fp_list(&self, name: &str) -> Option<RegisteredFlightPlanList> {
        // SAFETY: live CPlugIn*; the CString outlives the call.
        RegisteredFlightPlanList::from_owned(unsafe {
            euroscope_sys::es_plugin_register_fp_list(self.as_ptr(), cstr_lossy(name).as_ptr())
        })
    }
}
