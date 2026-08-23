use euroscope_sys::PluginPtr;

use crate::{ConnectionType, PopupCheck, Rect, utils::cstr_lossy};

/// Handle to the running plugin, through which you call back into EuroScope.
///
/// Passed to every [`Plugin`](crate::Plugin) callback. It wraps the shim's
/// `CPlugIn` instance; the pointer is stable for the plugin's lifetime.
///
/// # Validity
///
/// A `Context` is only usable **during a callback, on EuroScope's main
/// thread**. You are only ever handed a `&mut Context` borrowed for the
/// duration of the callback: it is neither `Clone` nor `Send`, so the borrow
/// checker already stops you from storing it or moving it to another thread.
/// The one escape hatch is [`as_ptr`](Self::as_ptr), which is `unsafe`
/// precisely so that persisting the raw pointer is a deliberate, auditable act.
///
/// The handles returned by its query/iteration methods
/// ([`controller`](Self::controller), [`controllers`](Self::controllers), …)
/// carry the same callback lifetime, so they can't be stored past the callback
/// either. To "remember" an entity, store a stable key (e.g. its callsign) and
/// re-query it next callback. This does not compile:
///
/// ```compile_fail
/// use euroscope::{Context, ControllerHandle};
///
/// // Trying to smuggle a handle out with a longer lifetime is rejected:
/// // the handle borrows `*ctx`, which is only alive for the callback.
/// fn escape(ctx: &Context) -> ControllerHandle<'static> {
///     ctx.controller_myself().unwrap()
/// }
/// ```
///
/// As the API grows this gains methods for registering tag items / display
/// types and iterating flight plans, radar targets and controllers.
pub struct Context {
    raw: PluginPtr,
}

impl Context {
    pub(crate) fn from_raw(raw: PluginPtr) -> Self {
        Self { raw }
    }

    /// The underlying `CPlugIn*`. Escape hatch for calling `euroscope-sys`
    /// APIs not yet wrapped by the safe layer.
    ///
    /// # Safety
    ///
    /// The returned pointer is only valid while EuroScope is inside a callback
    /// on its main thread — the same window in which this `Context` is valid.
    /// You must not store it, send it to another thread, or use it after the
    /// callback returns; doing so is undefined behaviour. The `unsafe` marker is
    /// here to make you think twice before extracting and holding onto it.
    pub unsafe fn as_ptr(&self) -> PluginPtr {
        self.raw
    }

    /// Print a line into the controller's message/chat area.
    ///
    /// * `handler` — the label shown above the chat area (typically your plugin name); the message
    ///   appears under this handler.
    /// * `sender` — the sender name; if empty, it looks as if the controller sent it themselves.
    /// * `message` — the text.
    ///
    /// The handler is made visible and highlighted as unread, but does not
    /// flash and needs no confirmation. Interior NUL bytes are stripped.
    pub fn display_message(&self, handler: &str, sender: &str, message: &str) {
        let handler = cstr_lossy(handler);
        let sender = cstr_lossy(sender);
        let message = cstr_lossy(message);
        // SAFETY: `raw` is EuroScope's live CPlugIn*; the three pointers are
        // valid NUL-terminated strings for the duration of the call.
        unsafe {
            euroscope_sys::es_plugin_display_user_message(
                self.raw,
                handler.as_ptr(),
                sender.as_ptr(),
                message.as_ptr(),
                true,  // show_handler
                true,  // show_unread
                false, // show_unread_even_if_busy
                false, // start_flashing
                false, // need_confirmation
            );
        }
    }

    /// EuroScope's current network connection type.
    pub fn connection_type(&self) -> ConnectionType {
        // SAFETY: `raw` is EuroScope's live CPlugIn*.
        ConnectionType::from_raw(unsafe { euroscope_sys::es_plugin_connection_type(self.raw) })
    }

    /// Whether EuroScope is connected to the network in any way.
    pub fn is_connected(&self) -> bool {
        self.connection_type().is_connected()
    }

    /// Number of flight plans currently tracked by me.
    pub fn aircraft_tracked_by_me(&self) -> u32 {
        // SAFETY: `raw` is EuroScope's live CPlugIn*; the count is non-negative.
        unsafe {
            euroscope_sys::es_plugin_count_tracked_by_me(self.raw)
                .max(0)
                .cast_unsigned()
        }
    }

    /// Number of radar targets currently in range.
    pub fn aircraft_in_range(&self) -> u32 {
        // SAFETY: `raw` is EuroScope's live CPlugIn*; the count is non-negative.
        unsafe {
            euroscope_sys::es_plugin_count_radar_targets(self.raw)
                .max(0)
                .cast_unsigned()
        }
    }

    /// Register a custom tag item under `code`, so it appears in EuroScope's
    /// tag editor and drives [`Plugin::on_get_tag_item`](crate::Plugin::on_get_tag_item).
    pub fn register_tag_item_type(&self, name: &str, code: i32) {
        let name = cstr_lossy(name);
        // SAFETY: live CPlugIn*; the CString outlives the call.
        unsafe { euroscope_sys::es_plugin_register_tag_item_type(self.raw, name.as_ptr(), code) }
    }

    /// Register a custom tag-item function under `code`; invocations arrive via
    /// [`Plugin::on_function_call`](crate::Plugin::on_function_call).
    pub fn register_tag_item_function(&self, name: &str, code: i32) {
        let name = cstr_lossy(name);
        // SAFETY: live CPlugIn*; the CString outlives the call.
        unsafe {
            euroscope_sys::es_plugin_register_tag_item_function(self.raw, name.as_ptr(), code);
        }
    }

    /// This plugin's own name.
    pub fn name(&self) -> &str {
        // SAFETY: live CPlugIn*; borrowed NUL-terminated string.
        unsafe { crate::utils::cstr(euroscope_sys::es_plugin_name(self.raw)) }
    }

    /// Register a toolbar item (button). Clicks arrive via
    /// [`Plugin::on_function_call`](crate::Plugin::on_function_call) under
    /// `item_id`.
    pub fn register_toolbar_item(&self, item_id: i32, name: &str) {
        let name = cstr_lossy(name);
        // SAFETY: live CPlugIn*; the CString outlives the call.
        unsafe { euroscope_sys::es_plugin_register_toolbar_item(self.raw, item_id, name.as_ptr()) }
    }

    /// Refresh the toolbar; `resize` also re-lays-out the items.
    pub fn refresh_toolbar(&self, resize: bool) {
        // SAFETY: live CPlugIn*.
        unsafe { euroscope_sys::es_plugin_refresh_toolbar(self.raw, resize) }
    }

    /// Open the active-sectorfile selection dialog.
    pub fn select_active_sectorfile(&self) {
        // SAFETY: live CPlugIn*.
        unsafe { euroscope_sys::es_plugin_select_active_sectorfile(self.raw) }
    }

    /// Select the sectorfile for a specific radar screen.
    pub fn select_screen_sectorfile(&self, screen: &crate::RadarScreenContext) {
        // SAFETY: live CPlugIn*; `screen` wraps a live CRadarScreen*.
        unsafe { euroscope_sys::es_plugin_select_screen_sectorfile(self.raw, screen.as_ptr()) }
    }

    /// Register a custom radar display type. When the user creates a screen of
    /// this type, [`Plugin::on_radar_screen_created`](crate::Plugin::on_radar_screen_created)
    /// is called to build the [`RadarScreen`](crate::RadarScreen).
    ///
    /// * `need_radar_content` — the display shows radar targets.
    /// * `geo_referenced` — lat/lon coordinates are meaningful.
    /// * `can_be_saved` — persist the display to ASR files.
    /// * `can_be_created` — offer it in the *Sector file / New* menu.
    pub fn register_display_type(
        &self,
        name: &str,
        need_radar_content: bool,
        geo_referenced: bool,
        can_be_saved: bool,
        can_be_created: bool,
    ) {
        let name = cstr_lossy(name);
        // SAFETY: live CPlugIn*; the CString outlives the call.
        unsafe {
            euroscope_sys::es_plugin_register_display_type(
                self.raw,
                name.as_ptr(),
                need_radar_content,
                geo_referenced,
                can_be_saved,
                can_be_created,
            );
        }
    }

    /// Register a `.chat` alias that expands `name` to `value`.
    pub fn add_alias(&self, name: &str, value: &str) {
        let name = cstr_lossy(name);
        let value = cstr_lossy(value);
        // SAFETY: live CPlugIn*; the CStrings outlive the call.
        unsafe { euroscope_sys::es_plugin_add_alias(self.raw, name.as_ptr(), value.as_ptr()) }
    }

    /// Begin a popup list at `area` (typically the `area` from
    /// [`Plugin::on_function_call`](crate::Plugin::on_function_call)). Follow
    /// with [`add_popup_list_element`](Self::add_popup_list_element) calls.
    /// `columns` is 1 or 2.
    pub fn open_popup_list(&self, area: Rect, title: &str, columns: i32) {
        let title = cstr_lossy(title);
        // SAFETY: live CPlugIn*; the CString outlives the call.
        unsafe {
            euroscope_sys::es_plugin_open_popup_list(
                self.raw,
                area.left,
                area.top,
                area.right,
                area.bottom,
                title.as_ptr(),
                columns,
            );
        }
    }

    /// Add an element to the currently open popup list. `function_id` is
    /// delivered to [`Plugin::on_function_call`](crate::Plugin::on_function_call)
    /// when the element is chosen. `second` is only shown in 2-column lists.
    #[expect(clippy::too_many_arguments)]
    pub fn add_popup_list_element(
        &self,
        first: &str,
        second: &str,
        function_id: i32,
        selected: bool,
        checked: PopupCheck,
        disabled: bool,
        fixed: bool,
    ) {
        let first = cstr_lossy(first);
        let second = cstr_lossy(second);
        // SAFETY: live CPlugIn*; the CStrings outlive the call.
        unsafe {
            euroscope_sys::es_plugin_add_popup_list_element(
                self.raw,
                first.as_ptr(),
                second.as_ptr(),
                function_id,
                selected,
                checked.to_raw(),
                disabled,
                fixed,
            );
        }
    }

    /// Convenience: add a plain single-column popup list item.
    pub fn add_popup_list_item(&self, text: &str, function_id: i32) {
        self.add_popup_list_element(
            text,
            "",
            function_id,
            false,
            PopupCheck::NoCheckbox,
            false,
            false,
        );
    }

    /// Open an inline edit box at `area`. The entered text is delivered to
    /// [`Plugin::on_function_call`](crate::Plugin::on_function_call) under
    /// `function_id`.
    pub fn open_popup_edit(&self, area: Rect, function_id: i32, initial_value: &str) {
        let initial = cstr_lossy(initial_value);
        // SAFETY: live CPlugIn*; the CString outlives the call.
        unsafe {
            euroscope_sys::es_plugin_open_popup_edit(
                self.raw,
                area.left,
                area.top,
                area.right,
                area.bottom,
                function_id,
                initial.as_ptr(),
            );
        }
    }

    /// The current transition altitude, in feet.
    pub fn transition_altitude(&self) -> i32 {
        // SAFETY: live CPlugIn*.
        unsafe { euroscope_sys::es_plugin_transition_altitude(self.raw) }
    }

    /// Read a persisted plugin setting by key. `None` if unset/empty.
    pub fn get_setting(&self, key: &str) -> Option<String> {
        // SAFETY: live CPlugIn*; the CString outlives the call; the returned
        // pointer is a borrowed NUL-terminated string we copy immediately.
        unsafe {
            let ptr =
                euroscope_sys::es_plugin_get_data_from_settings(self.raw, cstr_lossy(key).as_ptr());
            let s = crate::utils::cstr(ptr);
            (!s.is_empty()).then(|| s.to_owned())
        }
    }

    /// Persist a plugin setting (saved into the EuroScope settings file).
    pub fn save_setting(&self, key: &str, description: &str, value: &str) {
        // SAFETY: live CPlugIn*; the CStrings outlive the call.
        unsafe {
            euroscope_sys::es_plugin_save_data_to_settings(
                self.raw,
                cstr_lossy(key).as_ptr(),
                cstr_lossy(description).as_ptr(),
                cstr_lossy(value).as_ptr(),
            );
        }
    }
}
