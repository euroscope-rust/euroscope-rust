//! `CFlightPlanList` FFI declarations.

use std::ffi::{c_char, c_int};

use crate::{EsHandle, FlightPlanPtr, PluginPtr};

unsafe extern "C" {
    /// Whether the flight-plan-list handle is valid (`CFlightPlanList::IsValid`).
    pub fn es_fplist_is_valid(h: EsHandle) -> bool;

    /// Number of columns defined so far (`CFlightPlanList::GetColumnNumber`).
    pub fn es_fplist_column_number(h: EsHandle) -> c_int;

    /// Delete all previous column definitions (`CFlightPlanList::DeleteAllColumns`).
    pub fn es_fplist_delete_all_columns(h: EsHandle);

    /// Add one column definition to the list (`CFlightPlanList::AddColumnDefinition`).
    /// String pointers are borrowed NUL-terminated ANSI; `""` selects ES built-ins.
    pub fn es_fplist_add_column_definition(
        h: EsHandle,
        column_title: *const c_char,
        width: c_int,
        centered: bool,
        item_provider: *const c_char,
        item_code: c_int,
        left_button_function_provider: *const c_char,
        left_button_function: c_int,
        right_button_function_provider: *const c_char,
        right_button_function: c_int,
    );

    /// Add a flight plan to the list (`CFlightPlanList::AddFpToTheList`).
    pub fn es_fplist_add_fp_to_the_list(h: EsHandle, fp: FlightPlanPtr);

    /// Remove a flight plan from the list (`CFlightPlanList::RemoveFpFromTheList`).
    pub fn es_fplist_remove_fp_from_the_list(h: EsHandle, fp: FlightPlanPtr);

    /// Show or hide the flight-plan list (`CFlightPlanList::ShowFpList`).
    pub fn es_fplist_show_fp_list(h: EsHandle, show: bool);

    /// Free a flight-plan-list handle allocated by `es_plugin_register_fp_list`.
    pub fn es_fplist_free(list: EsHandle);

    /// Register a new flight-plan list and return an owned handle, or null.
    /// Wraps `CPlugIn::RegisterFpList`.
    pub fn es_plugin_register_fp_list(plugin: PluginPtr, name: *const c_char) -> EsHandle;
}
