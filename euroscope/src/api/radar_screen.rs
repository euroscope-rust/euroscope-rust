//! `CRadarScreen` — a custom radar display.
//!
//! Register a display type with
//! [`Context::register_display_type`](crate::Context::register_display_type), then return a
//! `Box<dyn RadarScreen>` from
//! [`Plugin::on_radar_screen_created`](crate::Plugin::on_radar_screen_created)
//! when EuroScope creates a screen of that type. EuroScope owns the screen and
//! drops it (via the SDK's `OnAsrContentToBeClosed`) automatically.

use std::ffi::{c_int, c_void};

use euroscope_sys::EsHandle;

use crate::{
    Context, Controller, ControllerDataType, FlightPlan, Point, Position, RadarTarget, Rect,
    RefreshPhase, SectorElement,
    utils::{cstr, cstr_lossy},
};

/// A Win32 device-context handle (`HDC`) for drawing in
/// [`RadarScreen::on_refresh`]. Draw with your preferred GDI bindings via
/// [`as_raw`](Self::as_raw); the pointer is valid only for that call.
#[derive(Clone, Copy)]
pub struct Hdc(*mut c_void);

impl Hdc {
    pub(crate) fn from_raw(raw: *mut c_void) -> Self {
        Self(raw)
    }

    /// The raw `HDC` pointer, for passing to GDI functions.
    pub fn as_raw(self) -> *mut c_void {
        self.0
    }
}

/// A custom radar display. Implement this and return a boxed instance from
/// [`Plugin::on_radar_screen_created`](crate::Plugin::on_radar_screen_created).
///
/// All methods have empty defaults; each receives a [`RadarScreenContext`] to
/// query areas, convert coordinates, register screen objects and draw. As
/// always, **never block** — these run on EuroScope's main thread.
pub trait RadarScreen: 'static {
    /// ASR content is available: load your per-screen settings from it here.
    fn on_asr_content_loaded(&mut self, screen: &mut RadarScreenContext, loaded: bool) {
        let _ = (screen, loaded);
    }

    /// The ASR is about to be saved: write your settings via
    /// [`RadarScreenContext::save_asr_data`].
    fn on_asr_content_to_be_saved(&mut self, screen: &mut RadarScreenContext) {
        let _ = screen;
    }

    /// Repaint. Called once per [`RefreshPhase`] each redraw. Draw onto `hdc`.
    fn on_refresh(&mut self, screen: &mut RadarScreenContext, hdc: Hdc, phase: RefreshPhase) {
        let _ = (screen, hdc, phase);
    }

    /// A `.command` line no other handler consumed. Return `true` if handled.
    fn on_compile_command(&mut self, screen: &mut RadarScreenContext, command: &str) -> bool {
        let _ = (screen, command);
        false
    }

    /// A registered tag-item function fired on this screen.
    fn on_function_call(
        &mut self,
        screen: &mut RadarScreenContext,
        function_id: i32,
        item_string: &str,
        point: Point,
        area: Rect,
    ) {
        let _ = (screen, function_id, item_string, point, area);
    }

    /// The mouse is hovering over a registered screen object.
    fn on_screen_object_over(
        &mut self,
        screen: &mut RadarScreenContext,
        object_type: i32,
        object_id: &str,
        point: Point,
        area: Rect,
    ) {
        let _ = (screen, object_type, object_id, point, area);
    }

    /// A mouse button was pressed on a screen object. `button` is the SDK's
    /// button code (e.g. left/right).
    fn on_screen_object_button_down(
        &mut self,
        screen: &mut RadarScreenContext,
        object_type: i32,
        object_id: &str,
        point: Point,
        area: Rect,
        button: crate::MouseButton,
    ) {
        let _ = (screen, object_type, object_id, point, area, button);
    }

    /// A mouse button was released on a screen object.
    fn on_screen_object_button_up(
        &mut self,
        screen: &mut RadarScreenContext,
        object_type: i32,
        object_id: &str,
        point: Point,
        area: Rect,
        button: crate::MouseButton,
    ) {
        let _ = (screen, object_type, object_id, point, area, button);
    }

    /// A screen object was clicked.
    fn on_screen_object_click(
        &mut self,
        screen: &mut RadarScreenContext,
        object_type: i32,
        object_id: &str,
        point: Point,
        area: Rect,
        button: crate::MouseButton,
    ) {
        let _ = (screen, object_type, object_id, point, area, button);
    }

    /// A screen object was double-clicked.
    fn on_screen_object_double_click(
        &mut self,
        screen: &mut RadarScreenContext,
        object_type: i32,
        object_id: &str,
        point: Point,
        area: Rect,
        button: crate::MouseButton,
    ) {
        let _ = (screen, object_type, object_id, point, area, button);
    }

    /// A moveable screen object is being dragged; `released` marks the drop.
    fn on_screen_object_move(
        &mut self,
        screen: &mut RadarScreenContext,
        object_type: i32,
        object_id: &str,
        point: Point,
        area: Rect,
        released: bool,
    ) {
        let _ = (screen, object_type, object_id, point, area, released);
    }

    /// A controller's position was updated (screen-scoped).
    fn on_controller_position_update(
        &mut self,
        screen: &mut RadarScreenContext,
        controller: Controller<'_>,
    ) {
        let _ = (screen, controller);
    }

    /// A controller disconnected (screen-scoped).
    fn on_controller_disconnect(
        &mut self,
        screen: &mut RadarScreenContext,
        controller: Controller<'_>,
    ) {
        let _ = (screen, controller);
    }

    /// A radar target's position was updated (screen-scoped).
    fn on_radar_target_position_update(
        &mut self,
        screen: &mut RadarScreenContext,
        radar_target: RadarTarget<'_>,
    ) {
        let _ = (screen, radar_target);
    }

    /// A flight plan disconnected (screen-scoped).
    fn on_flight_plan_disconnect(
        &mut self,
        screen: &mut RadarScreenContext,
        flight_plan: FlightPlan<'_>,
    ) {
        let _ = (screen, flight_plan);
    }

    /// A flight plan's data was updated (screen-scoped).
    fn on_flight_plan_data_update(
        &mut self,
        screen: &mut RadarScreenContext,
        flight_plan: FlightPlan<'_>,
    ) {
        let _ = (screen, flight_plan);
    }

    /// Controller-assigned data changed (screen-scoped).
    fn on_controller_assigned_data_update(
        &mut self,
        screen: &mut RadarScreenContext,
        flight_plan: FlightPlan<'_>,
        data_type: ControllerDataType,
    ) {
        let _ = (screen, flight_plan, data_type);
    }

    /// A flight strip was pushed to another controller (screen-scoped).
    fn on_flight_strip_pushed(
        &mut self,
        screen: &mut RadarScreenContext,
        flight_plan: FlightPlan<'_>,
        sender: &str,
        target: &str,
    ) {
        let _ = (screen, flight_plan, sender, target);
    }
}

/// Handle to a live radar screen, passed to every [`RadarScreen`] callback.
/// Wraps the SDK `CRadarScreen*`; valid only for the callback.
pub struct RadarScreenContext {
    raw: EsHandle,
}

type RectFn = unsafe extern "C" fn(EsHandle, *mut c_int, *mut c_int, *mut c_int, *mut c_int);

impl RadarScreenContext {
    pub(crate) fn from_raw(raw: EsHandle) -> Self {
        Self { raw }
    }

    pub(crate) fn as_ptr(&self) -> EsHandle {
        self.raw
    }

    fn rect(&self, f: RectFn) -> Rect {
        let (mut l, mut t, mut r, mut b) = (0_i32, 0_i32, 0_i32, 0_i32);
        // SAFETY: `raw` is the live CRadarScreen*; out-params are valid slots.
        unsafe {
            f(self.raw, &raw mut l, &raw mut t, &raw mut r, &raw mut b);
        }
        Rect::new(l, t, r, b)
    }

    /// The owning plugin.
    pub fn plugin(&self) -> Context {
        // SAFETY: `raw` is the live CRadarScreen*.
        Context::from_raw(unsafe { euroscope_sys::es_screen_get_plugin(self.raw) })
    }

    /// The radar drawing area, in pixels.
    pub fn radar_area(&self) -> Rect {
        self.rect(euroscope_sys::es_screen_radar_area)
    }

    /// The toolbar area, in pixels.
    pub fn toolbar_area(&self) -> Rect {
        self.rect(euroscope_sys::es_screen_toolbar_area)
    }

    /// The chat area, in pixels.
    pub fn chat_area(&self) -> Rect {
        self.rect(euroscope_sys::es_screen_chat_area)
    }

    /// Convert a geographic position to screen pixels.
    pub fn position_to_pixel(&self, position: Position) -> Point {
        let (mut x, mut y) = (0_i32, 0_i32);
        // SAFETY: `raw` is live; out-params are valid slots.
        unsafe {
            euroscope_sys::es_screen_position_to_pixel(
                self.raw,
                position.latitude,
                position.longitude,
                &raw mut x,
                &raw mut y,
            );
        }
        Point::new(x, y)
    }

    /// Convert screen pixels to a geographic position.
    pub fn pixel_to_position(&self, point: Point) -> Position {
        let (mut lat, mut lon) = (0.0_f64, 0.0_f64);
        // SAFETY: `raw` is live; out-params are valid slots.
        unsafe {
            euroscope_sys::es_screen_pixel_to_position(
                self.raw,
                point.x,
                point.y,
                &raw mut lat,
                &raw mut lon,
            );
        }
        Position::new(lat, lon)
    }

    /// Register a rectangular screen object that becomes clickable/hoverable and
    /// drives the `on_screen_object_*` callbacks. `object_type`/`object_id` are
    /// yours to interpret; `tooltip` shows on hover.
    pub fn add_screen_object(
        &self,
        object_type: i32,
        object_id: &str,
        area: Rect,
        moveable: bool,
        tooltip: &str,
    ) {
        let object_id = cstr_lossy(object_id);
        let tooltip = cstr_lossy(tooltip);
        // SAFETY: `raw` is live; the CStrings outlive the call.
        unsafe {
            euroscope_sys::es_screen_add_screen_object(
                self.raw,
                object_type,
                object_id.as_ptr(),
                area.left,
                area.top,
                area.right,
                area.bottom,
                moveable,
                tooltip.as_ptr(),
            );
        }
    }

    /// Request an asynchronous repaint of this screen.
    pub fn request_refresh(&self) {
        // SAFETY: `raw` is live.
        unsafe { euroscope_sys::es_screen_request_refresh(self.raw) }
    }

    /// Rebuild this screen's cached map content.
    pub fn refresh_map_content(&self) {
        // SAFETY: `raw` is live.
        unsafe { euroscope_sys::es_screen_refresh_map_content(self.raw) }
    }

    /// Read an ASR variable. `None` if unset/empty.
    pub fn get_asr_data(&self, name: &str) -> Option<String> {
        let name = cstr_lossy(name);
        // SAFETY: `raw` is live; the CString outlives the call; the returned
        // pointer is borrowed and copied immediately.
        unsafe {
            let s = cstr(euroscope_sys::es_screen_get_data_from_asr(
                self.raw,
                name.as_ptr(),
            ));
            (!s.is_empty()).then(|| s.to_owned())
        }
    }

    /// Persist an ASR variable (stored in the .asr file).
    pub fn save_asr_data(&self, name: &str, description: &str, value: &str) {
        let name = cstr_lossy(name);
        let description = cstr_lossy(description);
        let value = cstr_lossy(value);
        // SAFETY: `raw` is live; the CStrings outlive the call.
        unsafe {
            euroscope_sys::es_screen_save_data_to_asr(
                self.raw,
                name.as_ptr(),
                description.as_ptr(),
                value.as_ptr(),
            );
        }
    }

    /// The current display bounds as `(left_down, right_up)` corners.
    pub fn display_area(&self) -> (Position, Position) {
        let (mut a, mut b, mut c, mut d) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
        // SAFETY: `raw` is live; out-params are valid slots.
        unsafe {
            euroscope_sys::es_screen_get_display_area(
                self.raw, &raw mut a, &raw mut b, &raw mut c, &raw mut d,
            );
        }
        (Position::new(a, b), Position::new(c, d))
    }

    /// Set the display bounds from two corners.
    pub fn set_display_area(&self, left_down: Position, right_up: Position) {
        // SAFETY: `raw` is live.
        unsafe {
            euroscope_sys::es_screen_set_display_area(
                self.raw,
                left_down.latitude,
                left_down.longitude,
                right_up.latitude,
                right_up.longitude,
            );
        }
    }

    /// Show or hide a sector-file element's named component on this screen.
    pub fn show_sector_element(&self, element: &SectorElement<'_>, component: &str, show: bool) {
        let component = cstr_lossy(component);
        // SAFETY: `raw` and the element pointer are live; CString outlives call.
        unsafe {
            euroscope_sys::es_screen_show_sector_element(
                self.raw,
                element.as_ptr(),
                component.as_ptr(),
                show,
            );
        }
    }

    /// Trigger another plugin's tag function on a target aircraft.
    #[expect(clippy::too_many_arguments)]
    pub fn start_tag_function(
        &self,
        callsign: &str,
        item_plugin: &str,
        item_code: i32,
        item_string: &str,
        function_plugin: &str,
        function_id: i32,
        point: Point,
        area: Rect,
    ) {
        let callsign = cstr_lossy(callsign);
        let item_plugin = cstr_lossy(item_plugin);
        let item_string = cstr_lossy(item_string);
        let function_plugin = cstr_lossy(function_plugin);
        // SAFETY: `raw` is live; the CStrings outlive the call.
        unsafe {
            euroscope_sys::es_screen_start_tag_function(
                self.raw,
                callsign.as_ptr(),
                item_plugin.as_ptr(),
                item_code,
                item_string.as_ptr(),
                function_plugin.as_ptr(),
                function_id,
                point.x,
                point.y,
                area.left,
                area.top,
                area.right,
                area.bottom,
            );
        }
    }
}
