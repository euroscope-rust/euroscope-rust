//! `CPlugIn`-level FFI declarations.

use std::ffi::{c_char, c_int};

use crate::PluginPtr;

unsafe extern "C" {
    /// `CPlugIn::DisplayUserMessage` — print a line into the controller's
    /// message/chat area. All strings are borrowed NUL-terminated ANSI.
    pub fn es_plugin_display_user_message(
        plugin: PluginPtr,
        handler: *const c_char,
        sender: *const c_char,
        message: *const c_char,
        show_handler: bool,
        show_unread: bool,
        show_unread_even_if_busy: bool,
        start_flashing: bool,
        need_confirmation: bool,
    );

    /// `CPlugIn::GetConnectionType` — one of the `CONNECTION_TYPE_*` codes.
    pub fn es_plugin_connection_type(plugin: PluginPtr) -> c_int;

    /// Number of flight plans currently tracked by me.
    pub fn es_plugin_count_tracked_by_me(plugin: PluginPtr) -> c_int;

    /// Number of radar targets currently in range.
    pub fn es_plugin_count_radar_targets(plugin: PluginPtr) -> c_int;

    /// `CPlugIn::RegisterTagItemType` — declare a custom tag item.
    pub fn es_plugin_register_tag_item_type(plugin: PluginPtr, name: *const c_char, code: c_int);

    /// `CPlugIn::RegisterTagItemFunction` — declare a custom tag-item function.
    pub fn es_plugin_register_tag_item_function(
        plugin: PluginPtr,
        name: *const c_char,
        code: c_int,
    );

    /// `CPlugIn::AddAlias` — register a `.chat` alias expansion.
    pub fn es_plugin_add_alias(plugin: PluginPtr, name: *const c_char, value: *const c_char);

    /// `CPlugIn::GetPlugInName` — the plugin's own name (borrowed).
    pub fn es_plugin_name(plugin: PluginPtr) -> *const c_char;

    /// `CPlugIn::RegisterToolbarItem`.
    pub fn es_plugin_register_toolbar_item(plugin: PluginPtr, item_id: c_int, name: *const c_char);

    /// `CPlugIn::RefreshToolbar`.
    pub fn es_plugin_refresh_toolbar(plugin: PluginPtr, resize_too: bool);

    /// `CPlugIn::SelectActiveSectorfile`.
    pub fn es_plugin_select_active_sectorfile(plugin: PluginPtr);

    /// `CPlugIn::SelectScreenSectorfile` — `screen` is a `CRadarScreen*`.
    pub fn es_plugin_select_screen_sectorfile(plugin: PluginPtr, screen: crate::EsHandle);

    /// `CPlugIn::RegisterDisplayType` — register a custom radar display type.
    pub fn es_plugin_register_display_type(
        plugin: PluginPtr,
        name: *const c_char,
        need_radar_content: bool,
        geo_referenced: bool,
        can_be_saved: bool,
        can_be_created: bool,
    );

    /// `CPlugIn::OpenPopupList` — begin a popup list at the given rectangle.
    pub fn es_plugin_open_popup_list(
        plugin: PluginPtr,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        title: *const c_char,
        column_number: c_int,
    );

    /// `CPlugIn::OpenPopupEdit` — open an inline edit box at the given rectangle.
    pub fn es_plugin_open_popup_edit(
        plugin: PluginPtr,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        function_id: c_int,
        initial_value: *const c_char,
    );

    /// `CPlugIn::AddPopupListElement` — add an element to the open popup list.
    pub fn es_plugin_add_popup_list_element(
        plugin: PluginPtr,
        string1: *const c_char,
        string2: *const c_char,
        function_id: c_int,
        selected: bool,
        checked: c_int,
        disabled: bool,
        fixed: bool,
    );

    /// The current transition altitude, in feet.
    /// Wraps `CPlugIn::GetTransitionAltitude`.
    pub fn es_plugin_transition_altitude(plugin: PluginPtr) -> c_int;

    /// Read a persisted plugin setting by key; borrowed NUL-terminated string
    /// (empty if unset). Wraps `CPlugIn::GetDataFromSettings`.
    pub fn es_plugin_get_data_from_settings(plugin: PluginPtr, key: *const c_char)
    -> *const c_char;

    /// Persist a plugin setting. Wraps `CPlugIn::SaveDataToSettings`.
    pub fn es_plugin_save_data_to_settings(
        plugin: PluginPtr,
        key: *const c_char,
        description: *const c_char,
        value: *const c_char,
    );
}
