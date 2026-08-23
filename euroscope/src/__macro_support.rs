//! Internal glue used by [`register_plugin!`](crate::register_plugin).
//!
//! Not part of the public API — everything here is `#[doc(hidden)]` and may
//! change. It exists so the macro expansion stays tiny and all `unsafe` FFI
//! handling lives in one reviewed place, monomorphized per plugin type `P`.

use std::ffi::CString;
pub use std::ffi::{c_char, c_int, c_void};

pub use euroscope_sys::{EsHandle, FlightPlanPtr, PluginPtr, RustPluginMeta};

use crate::{
    Context, Controller, FlightPlan, FlightPlanList, GroundToAirChannel, Hdc, Plugin, Point,
    RadarScreen, RadarScreenContext, RadarTarget, Rect, RefreshPhase,
    api::tag::TagItem,
    utils::{cstr, cstr_lossy},
};

/// Everything the plugin instance and its owned resources live in. Boxed and
/// handed to the shim as an opaque `*mut c_void`.
struct State<P: Plugin> {
    plugin: P,
    ctx: Context,
}

fn leak_cstr(s: &str) -> *const c_char {
    CString::into_raw(cstr_lossy(s)).cast_const()
}

/// # Safety
/// `out` must point to a valid `RustPluginMeta` (the shim provides one).
pub unsafe fn fill_metadata<P: Plugin>(out: *mut RustPluginMeta) {
    unsafe {
        (*out).name = leak_cstr(P::NAME);
        (*out).version = leak_cstr(P::VERSION);
        (*out).author = leak_cstr(P::AUTHOR);
        (*out).copyright = leak_cstr(P::COPYRIGHT);
    }
}

/// # Safety
/// `plugin` must be the shim's `CPlugIn*`.
pub unsafe fn create<P: Plugin>(plugin: PluginPtr) -> *mut c_void {
    let mut ctx = Context::from_raw(plugin);
    let plugin = P::new(&mut ctx);
    // Flush anything logged during `new` (the subscriber is installed there).
    #[cfg(feature = "tracing")]
    crate::tracing::drain(&ctx, P::NAME);
    Box::into_raw(Box::new(State { plugin, ctx })).cast::<c_void>()
}

/// # Safety
/// `state` must be a pointer returned by [`create::<P>`] and not used again.
pub unsafe fn destroy<P: Plugin>(state: *mut c_void) {
    if state.is_null() {
        return;
    }
    // SAFETY: reclaim the Box we leaked in `create`.
    let state = unsafe { Box::from_raw(state.cast::<State<P>>()) };
    // Flush anything logged during teardown (we still hold the context here),
    // then release the channel so the next load starts clean.
    #[cfg(feature = "tracing")]
    {
        crate::tracing::drain(&state.ctx, P::NAME);
        crate::tracing::cleanup();
    }
    drop(state);
}

/// # Safety
/// `state` must be a live pointer from [`create::<P>`].
unsafe fn state<'a, P: Plugin>(state: *mut c_void) -> &'a mut State<P> {
    // SAFETY: EuroScope is single-threaded and does not re-enter a plugin
    // callback, so a fresh exclusive borrow per call does not alias.
    unsafe { &mut *state.cast::<State<P>>() }
}

// Each `on_*` below is `#[doc(hidden)]` glue: reconstruct handles/strings from
// the raw C args, then dispatch to the user's `Plugin` method. All are
// `# Safety`: `s` must be live from `create::<P>` and pointer args valid for
// the call.

pub unsafe fn on_timer<P: Plugin>(s: *mut c_void, counter: c_int) {
    let st = unsafe { state::<P>(s) };
    st.plugin.on_timer(&mut st.ctx, counter);

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);
}

pub unsafe fn on_controller_position_update<P: Plugin>(s: *mut c_void, controller: EsHandle) {
    let st = unsafe { state::<P>(s) };
    st.plugin
        .on_controller_position_update(&mut st.ctx, Controller::from_raw(controller));
}

pub unsafe fn on_controller_disconnect<P: Plugin>(s: *mut c_void, controller: EsHandle) {
    let st = unsafe { state::<P>(s) };
    st.plugin
        .on_controller_disconnect(&mut st.ctx, Controller::from_raw(controller));

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);
}

pub unsafe fn on_radar_target_position_update<P: Plugin>(s: *mut c_void, rt: EsHandle) {
    let st = unsafe { state::<P>(s) };
    st.plugin
        .on_radar_target_position_update(&mut st.ctx, RadarTarget::from_raw(rt));
}

pub unsafe fn on_flightplan_disconnect<P: Plugin>(s: *mut c_void, fp: FlightPlanPtr) {
    let st = unsafe { state::<P>(s) };
    st.plugin
        .on_flight_plan_disconnect(&mut st.ctx, FlightPlan::from_raw(fp));

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);
}

pub unsafe fn on_flightplan_data_update<P: Plugin>(s: *mut c_void, fp: FlightPlanPtr) {
    let st = unsafe { state::<P>(s) };
    st.plugin
        .on_flight_plan_data_update(&mut st.ctx, FlightPlan::from_raw(fp));
}

pub unsafe fn on_plane_information_update<P: Plugin>(
    s: *mut c_void,
    callsign: *const c_char,
    livery: *const c_char,
    plane_type: *const c_char,
) {
    let st = unsafe { state::<P>(s) };
    unsafe {
        st.plugin.on_plane_information_update(
            &mut st.ctx,
            cstr(callsign),
            cstr(livery),
            cstr(plane_type),
        );
    }
}

pub unsafe fn on_controller_assigned_data_update<P: Plugin>(
    s: *mut c_void,
    fp: FlightPlanPtr,
    data_type: c_int,
) {
    let st = unsafe { state::<P>(s) };
    st.plugin.on_controller_assigned_data_update(
        &mut st.ctx,
        FlightPlan::from_raw(fp),
        crate::ControllerDataType::from_raw(data_type),
    );
}

pub unsafe fn on_flight_strip_pushed<P: Plugin>(
    s: *mut c_void,
    fp: FlightPlanPtr,
    sender: *const c_char,
    target: *const c_char,
) {
    let st = unsafe { state::<P>(s) };
    unsafe {
        st.plugin.on_flight_strip_pushed(
            &mut st.ctx,
            FlightPlan::from_raw(fp),
            cstr(sender),
            cstr(target),
        );
    }
}

pub unsafe fn on_compile_command<P: Plugin>(s: *mut c_void, command: *const c_char) -> bool {
    let st = unsafe { state::<P>(s) };
    let ret = st
        .plugin
        .on_compile_command(&mut st.ctx, unsafe { cstr(command) });

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);

    ret
}

pub unsafe fn on_frequency_chat<P: Plugin>(
    s: *mut c_void,
    sender: *const c_char,
    frequency: f64,
    message: *const c_char,
) {
    let st = unsafe { state::<P>(s) };
    unsafe {
        st.plugin
            .on_frequency_chat(&mut st.ctx, cstr(sender), frequency, cstr(message));
    }

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);
}

pub unsafe fn on_private_chat<P: Plugin>(
    s: *mut c_void,
    sender: *const c_char,
    receiver: *const c_char,
    message: *const c_char,
) {
    let st = unsafe { state::<P>(s) };
    unsafe {
        st.plugin
            .on_private_chat(&mut st.ctx, cstr(sender), cstr(receiver), cstr(message));
    }

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);
}

pub unsafe fn on_refresh_fp_list<P: Plugin>(s: *mut c_void, list: EsHandle) {
    let st = unsafe { state::<P>(s) };
    st.plugin
        .on_refresh_fp_list(&mut st.ctx, FlightPlanList::from_raw(list));
}

pub unsafe fn on_new_metar<P: Plugin>(
    s: *mut c_void,
    station: *const c_char,
    full_metar: *const c_char,
) {
    let st = unsafe { state::<P>(s) };
    unsafe {
        st.plugin
            .on_new_metar(&mut st.ctx, cstr(station), cstr(full_metar));
    }

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);
}

#[expect(clippy::too_many_arguments)]
pub unsafe fn on_function_call<P: Plugin>(
    s: *mut c_void,
    function_id: c_int,
    item_string: *const c_char,
    x: c_int,
    y: c_int,
    left: c_int,
    top: c_int,
    right: c_int,
    bottom: c_int,
) {
    let st = unsafe { state::<P>(s) };
    st.plugin.on_function_call(
        &mut st.ctx,
        function_id,
        unsafe { cstr(item_string) },
        Point::new(x, y),
        Rect::new(left, top, right, bottom),
    );

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);
}

pub unsafe fn on_runway_activity_changed<P: Plugin>(s: *mut c_void) {
    let st = unsafe { state::<P>(s) };
    st.plugin.on_runway_activity_changed(&mut st.ctx);

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);
}

pub unsafe fn on_voice_transmit_started<P: Plugin>(s: *mut c_void, on_primary: bool) {
    let st = unsafe { state::<P>(s) };
    st.plugin.on_voice_transmit_started(&mut st.ctx, on_primary);
}

pub unsafe fn on_voice_transmit_ended<P: Plugin>(s: *mut c_void, on_primary: bool) {
    let st = unsafe { state::<P>(s) };
    st.plugin.on_voice_transmit_ended(&mut st.ctx, on_primary);
}

pub unsafe fn on_voice_receive_started<P: Plugin>(s: *mut c_void, channel: EsHandle) {
    let st = unsafe { state::<P>(s) };
    st.plugin
        .on_voice_receive_started(&mut st.ctx, GroundToAirChannel::from_raw(channel));
}

pub unsafe fn on_voice_receive_ended<P: Plugin>(s: *mut c_void, channel: EsHandle) {
    let st = unsafe { state::<P>(s) };
    st.plugin
        .on_voice_receive_ended(&mut st.ctx, GroundToAirChannel::from_raw(channel));
}

#[expect(clippy::too_many_arguments)]
pub unsafe fn on_get_tag_item<P: Plugin>(
    s: *mut c_void,
    fp: EsHandle,
    rt: EsHandle,
    item_code: c_int,
    tag_data: c_int,
    item_string: *mut c_char,
    color_code: *mut c_int,
    rgb: *mut u32,
    font_size: *mut f64,
) {
    let st = unsafe { state::<P>(s) };
    let mut item = unsafe { TagItem::new(item_string, color_code, rgb, font_size) };
    st.plugin.on_get_tag_item(
        &mut st.ctx,
        FlightPlan::from_raw(fp),
        RadarTarget::from_raw(rt),
        item_code,
        crate::TagData::from_raw(tag_data),
        &mut item,
    );
}

// --- Radar screen ---------------------------------------------------------
//
// A radar screen's state is a `Box<dyn RadarScreen>` (a trait object, since the
// plugin chooses the concrete type). The shim holds it as an opaque
// `*mut Box<dyn RadarScreen>`. These dispatchers are NOT generic over `P`.

/// # Safety
/// `s` must be a live `*mut Box<dyn RadarScreen>` from `on_radar_screen_created`.
unsafe fn screen<'a>(s: *mut c_void) -> &'a mut Box<dyn RadarScreen> {
    unsafe { &mut *s.cast::<Box<dyn RadarScreen>>() }
}

/// # Safety
/// `plugin` must be the shim's `CPlugIn*`.
pub unsafe fn on_radar_screen_created<P: Plugin>(
    state_ptr: *mut c_void,
    name: *const c_char,
    need_radar_content: bool,
    geo_referenced: bool,
    can_be_saved: bool,
    can_be_created: bool,
) -> *mut c_void {
    let st = unsafe { state::<P>(state_ptr) };
    let created = st.plugin.on_radar_screen_created(
        &mut st.ctx,
        unsafe { cstr(name) },
        need_radar_content,
        geo_referenced,
        can_be_saved,
        can_be_created,
    );

    #[cfg(feature = "tracing")]
    crate::tracing::drain(&st.ctx, P::NAME);

    match created {
        Some(screen) => Box::into_raw(Box::new(screen)).cast::<c_void>(),
        None => core::ptr::null_mut(),
    }
}

/// # # Safety `s` from `on_radar_screen_created`, not used again.
pub unsafe fn screen_destroy(s: *mut c_void) {
    if !s.is_null() {
        // SAFETY: reclaim the boxed trait object; its Drop runs the screen's.
        unsafe {
            drop(Box::from_raw(s.cast::<Box<dyn RadarScreen>>()));
        }
    }
}

pub unsafe fn screen_on_asr_loaded(s: *mut c_void, rs: EsHandle, loaded: bool) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_asr_content_loaded(&mut ctx, loaded);
}

pub unsafe fn screen_on_asr_to_be_saved(s: *mut c_void, rs: EsHandle) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_asr_content_to_be_saved(&mut ctx);
}

pub unsafe fn screen_on_refresh(s: *mut c_void, rs: EsHandle, hdc: *mut c_void, phase: c_int) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_refresh(&mut ctx, Hdc::from_raw(hdc), RefreshPhase::from_raw(phase));
}

pub unsafe fn screen_on_compile_command(s: *mut c_void, rs: EsHandle, cmd: *const c_char) -> bool {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_compile_command(&mut ctx, unsafe { cstr(cmd) })
}

#[expect(clippy::too_many_arguments)]
pub unsafe fn screen_on_function_call(
    s: *mut c_void,
    rs: EsHandle,
    function_id: c_int,
    item_string: *const c_char,
    x: c_int,
    y: c_int,
    l: c_int,
    t: c_int,
    r: c_int,
    b: c_int,
) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_function_call(
        &mut ctx,
        function_id,
        unsafe { cstr(item_string) },
        Point::new(x, y),
        Rect::new(l, t, r, b),
    );
}

// The object callbacks share a shape: (type, id, point, area[, button|released]).

#[expect(clippy::too_many_arguments)]
pub unsafe fn screen_on_object_over(
    s: *mut c_void,
    rs: EsHandle,
    object_type: c_int,
    object_id: *const c_char,
    x: c_int,
    y: c_int,
    l: c_int,
    t: c_int,
    r: c_int,
    b: c_int,
) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_screen_object_over(
        &mut ctx,
        object_type,
        unsafe { cstr(object_id) },
        Point::new(x, y),
        Rect::new(l, t, r, b),
    );
}

macro_rules! screen_object_button_dispatch {
    ($name:ident, $method:ident) => {
        #[expect(clippy::too_many_arguments)]
        pub unsafe fn $name(
            s: *mut c_void,
            rs: EsHandle,
            object_type: c_int,
            object_id: *const c_char,
            x: c_int,
            y: c_int,
            l: c_int,
            t: c_int,
            r: c_int,
            b: c_int,
            button: c_int,
        ) {
            let sc = unsafe { screen(s) };
            let mut ctx = RadarScreenContext::from_raw(rs);
            sc.$method(
                &mut ctx,
                object_type,
                unsafe { cstr(object_id) },
                Point::new(x, y),
                Rect::new(l, t, r, b),
                crate::MouseButton::from_raw(button),
            );
        }
    };
}

screen_object_button_dispatch!(screen_on_object_button_down, on_screen_object_button_down);
screen_object_button_dispatch!(screen_on_object_button_up, on_screen_object_button_up);
screen_object_button_dispatch!(screen_on_object_click, on_screen_object_click);
screen_object_button_dispatch!(screen_on_object_double_click, on_screen_object_double_click);

#[expect(clippy::too_many_arguments)]
pub unsafe fn screen_on_object_move(
    s: *mut c_void,
    rs: EsHandle,
    object_type: c_int,
    object_id: *const c_char,
    x: c_int,
    y: c_int,
    l: c_int,
    t: c_int,
    r: c_int,
    b: c_int,
    released: bool,
) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_screen_object_move(
        &mut ctx,
        object_type,
        unsafe { cstr(object_id) },
        Point::new(x, y),
        Rect::new(l, t, r, b),
        released,
    );
}

// Screen-scoped update callbacks (mirror the plugin-level ones).

pub unsafe fn screen_on_controller_position_update(s: *mut c_void, rs: EsHandle, c: EsHandle) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_controller_position_update(&mut ctx, Controller::from_raw(c));
}

pub unsafe fn screen_on_controller_disconnect(s: *mut c_void, rs: EsHandle, c: EsHandle) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_controller_disconnect(&mut ctx, Controller::from_raw(c));
}

pub unsafe fn screen_on_radar_target_position_update(s: *mut c_void, rs: EsHandle, rt: EsHandle) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_radar_target_position_update(&mut ctx, RadarTarget::from_raw(rt));
}

pub unsafe fn screen_on_flightplan_disconnect(s: *mut c_void, rs: EsHandle, fp: EsHandle) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_flight_plan_disconnect(&mut ctx, FlightPlan::from_raw(fp));
}

pub unsafe fn screen_on_flightplan_data_update(s: *mut c_void, rs: EsHandle, fp: EsHandle) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_flight_plan_data_update(&mut ctx, FlightPlan::from_raw(fp));
}

pub unsafe fn screen_on_controller_assigned_data_update(
    s: *mut c_void,
    rs: EsHandle,
    fp: EsHandle,
    data_type: c_int,
) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_controller_assigned_data_update(
        &mut ctx,
        FlightPlan::from_raw(fp),
        crate::ControllerDataType::from_raw(data_type),
    );
}

pub unsafe fn screen_on_flight_strip_pushed(
    s: *mut c_void,
    rs: EsHandle,
    fp: EsHandle,
    sender: *const c_char,
    target: *const c_char,
) {
    let sc = unsafe { screen(s) };
    let mut ctx = RadarScreenContext::from_raw(rs);
    sc.on_flight_strip_pushed(
        &mut ctx,
        FlightPlan::from_raw(fp),
        unsafe { cstr(sender) },
        unsafe { cstr(target) },
    );
}

/// Register a [`Plugin`](crate::Plugin) implementation as the exported
/// EuroScope plugin for this `cdylib`.
///
/// Emits `#[no_mangle]` definitions for the `rust_*` callbacks the C++ shim
/// imports. Invoke exactly once per plugin crate: `register_plugin!(MyPlugin);`
#[macro_export]
macro_rules! register_plugin {
    ($plugin:ty) => {
        const _: () = {
            use $crate::__macro_support as __es;

            #[unsafe(no_mangle)]
            extern "C" fn rust_plugin_metadata(out: *mut __es::RustPluginMeta) {
                unsafe { __es::fill_metadata::<$plugin>(out) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_plugin_create(plugin: __es::PluginPtr) -> *mut __es::c_void {
                unsafe { __es::create::<$plugin>(plugin) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_plugin_destroy(state: *mut __es::c_void) {
                unsafe { __es::destroy::<$plugin>(state) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_timer(state: *mut __es::c_void, counter: __es::c_int) {
                unsafe { __es::on_timer::<$plugin>(state, counter) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_controller_position_update(
                state: *mut __es::c_void,
                controller: __es::EsHandle,
            ) {
                unsafe { __es::on_controller_position_update::<$plugin>(state, controller) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_controller_disconnect(
                state: *mut __es::c_void,
                controller: __es::EsHandle,
            ) {
                unsafe { __es::on_controller_disconnect::<$plugin>(state, controller) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_radar_target_position_update(
                state: *mut __es::c_void,
                rt: __es::EsHandle,
            ) {
                unsafe { __es::on_radar_target_position_update::<$plugin>(state, rt) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_flightplan_disconnect(
                state: *mut __es::c_void,
                fp: __es::FlightPlanPtr,
            ) {
                unsafe { __es::on_flightplan_disconnect::<$plugin>(state, fp) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_flightplan_data_update(
                state: *mut __es::c_void,
                fp: __es::FlightPlanPtr,
            ) {
                unsafe { __es::on_flightplan_data_update::<$plugin>(state, fp) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_plane_information_update(
                state: *mut __es::c_void,
                callsign: *const __es::c_char,
                livery: *const __es::c_char,
                plane_type: *const __es::c_char,
            ) {
                unsafe {
                    __es::on_plane_information_update::<$plugin>(
                        state, callsign, livery, plane_type,
                    )
                }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_controller_assigned_data_update(
                state: *mut __es::c_void,
                fp: __es::FlightPlanPtr,
                data_type: __es::c_int,
            ) {
                unsafe { __es::on_controller_assigned_data_update::<$plugin>(state, fp, data_type) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_flight_strip_pushed(
                state: *mut __es::c_void,
                fp: __es::FlightPlanPtr,
                sender: *const __es::c_char,
                target: *const __es::c_char,
            ) {
                unsafe { __es::on_flight_strip_pushed::<$plugin>(state, fp, sender, target) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_compile_command(
                state: *mut __es::c_void,
                command_line: *const __es::c_char,
            ) -> bool {
                unsafe { __es::on_compile_command::<$plugin>(state, command_line) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_frequency_chat(
                state: *mut __es::c_void,
                sender: *const __es::c_char,
                frequency: f64,
                message: *const __es::c_char,
            ) {
                unsafe { __es::on_frequency_chat::<$plugin>(state, sender, frequency, message) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_private_chat(
                state: *mut __es::c_void,
                sender: *const __es::c_char,
                receiver: *const __es::c_char,
                message: *const __es::c_char,
            ) {
                unsafe { __es::on_private_chat::<$plugin>(state, sender, receiver, message) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_refresh_fp_list(
                state: *mut __es::c_void,
                ac_list: __es::EsHandle,
            ) {
                unsafe { __es::on_refresh_fp_list::<$plugin>(state, ac_list) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_new_metar(
                state: *mut __es::c_void,
                station: *const __es::c_char,
                full_metar: *const __es::c_char,
            ) {
                unsafe { __es::on_new_metar::<$plugin>(state, station, full_metar) }
            }
            #[unsafe(no_mangle)]
            #[allow(clippy::too_many_arguments)]
            extern "C" fn rust_on_function_call(
                state: *mut __es::c_void,
                function_id: __es::c_int,
                item_string: *const __es::c_char,
                point_x: __es::c_int,
                point_y: __es::c_int,
                area_left: __es::c_int,
                area_top: __es::c_int,
                area_right: __es::c_int,
                area_bottom: __es::c_int,
            ) {
                unsafe {
                    __es::on_function_call::<$plugin>(
                        state,
                        function_id,
                        item_string,
                        point_x,
                        point_y,
                        area_left,
                        area_top,
                        area_right,
                        area_bottom,
                    )
                }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_runway_activity_changed(state: *mut __es::c_void) {
                unsafe { __es::on_runway_activity_changed::<$plugin>(state) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_voice_transmit_started(
                state: *mut __es::c_void,
                on_primary: bool,
            ) {
                unsafe { __es::on_voice_transmit_started::<$plugin>(state, on_primary) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_voice_transmit_ended(state: *mut __es::c_void, on_primary: bool) {
                unsafe { __es::on_voice_transmit_ended::<$plugin>(state, on_primary) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_voice_receive_started(
                state: *mut __es::c_void,
                channel: __es::EsHandle,
            ) {
                unsafe { __es::on_voice_receive_started::<$plugin>(state, channel) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_voice_receive_ended(
                state: *mut __es::c_void,
                channel: __es::EsHandle,
            ) {
                unsafe { __es::on_voice_receive_ended::<$plugin>(state, channel) }
            }
            #[unsafe(no_mangle)]
            #[allow(clippy::too_many_arguments)]
            extern "C" fn rust_on_get_tag_item(
                state: *mut __es::c_void,
                flightplan: __es::EsHandle,
                radar_target: __es::EsHandle,
                item_code: __es::c_int,
                tag_data: __es::c_int,
                item_string: *mut __es::c_char,
                color_code: *mut __es::c_int,
                rgb: *mut u32,
                font_size: *mut f64,
            ) {
                unsafe {
                    __es::on_get_tag_item::<$plugin>(
                        state,
                        flightplan,
                        radar_target,
                        item_code,
                        tag_data,
                        item_string,
                        color_code,
                        rgb,
                        font_size,
                    )
                }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_on_radar_screen_created(
                state: *mut __es::c_void,
                display_name: *const __es::c_char,
                need_radar_content: bool,
                geo_referenced: bool,
                can_be_saved: bool,
                can_be_created: bool,
            ) -> *mut __es::c_void {
                unsafe {
                    __es::on_radar_screen_created::<$plugin>(
                        state,
                        display_name,
                        need_radar_content,
                        geo_referenced,
                        can_be_saved,
                        can_be_created,
                    )
                }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_destroy(screen: *mut __es::c_void) {
                unsafe { __es::screen_destroy(screen) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_asr_loaded(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                loaded: bool,
            ) {
                unsafe { __es::screen_on_asr_loaded(screen, rs, loaded) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_asr_to_be_saved(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
            ) {
                unsafe { __es::screen_on_asr_to_be_saved(screen, rs) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_refresh(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                hdc: *mut __es::c_void,
                phase: __es::c_int,
            ) {
                unsafe { __es::screen_on_refresh(screen, rs, hdc, phase) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_compile_command(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                command: *const __es::c_char,
            ) -> bool {
                unsafe { __es::screen_on_compile_command(screen, rs, command) }
            }
            #[unsafe(no_mangle)]
            #[allow(clippy::too_many_arguments)]
            extern "C" fn rust_screen_on_function_call(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                function_id: __es::c_int,
                item_string: *const __es::c_char,
                x: __es::c_int,
                y: __es::c_int,
                l: __es::c_int,
                t: __es::c_int,
                r: __es::c_int,
                b: __es::c_int,
            ) {
                unsafe {
                    __es::screen_on_function_call(
                        screen,
                        rs,
                        function_id,
                        item_string,
                        x,
                        y,
                        l,
                        t,
                        r,
                        b,
                    )
                }
            }
            #[unsafe(no_mangle)]
            #[allow(clippy::too_many_arguments)]
            extern "C" fn rust_screen_on_object_over(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                object_type: __es::c_int,
                object_id: *const __es::c_char,
                x: __es::c_int,
                y: __es::c_int,
                l: __es::c_int,
                t: __es::c_int,
                r: __es::c_int,
                b: __es::c_int,
            ) {
                unsafe {
                    __es::screen_on_object_over(
                        screen,
                        rs,
                        object_type,
                        object_id,
                        x,
                        y,
                        l,
                        t,
                        r,
                        b,
                    )
                }
            }
            $crate::__es_screen_object_button_arm!(
                rust_screen_on_object_button_down,
                screen_on_object_button_down
            );
            $crate::__es_screen_object_button_arm!(
                rust_screen_on_object_button_up,
                screen_on_object_button_up
            );
            $crate::__es_screen_object_button_arm!(
                rust_screen_on_object_click,
                screen_on_object_click
            );
            $crate::__es_screen_object_button_arm!(
                rust_screen_on_object_double_click,
                screen_on_object_double_click
            );
            #[unsafe(no_mangle)]
            #[allow(clippy::too_many_arguments)]
            extern "C" fn rust_screen_on_object_move(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                object_type: __es::c_int,
                object_id: *const __es::c_char,
                x: __es::c_int,
                y: __es::c_int,
                l: __es::c_int,
                t: __es::c_int,
                r: __es::c_int,
                b: __es::c_int,
                released: bool,
            ) {
                unsafe {
                    __es::screen_on_object_move(
                        screen,
                        rs,
                        object_type,
                        object_id,
                        x,
                        y,
                        l,
                        t,
                        r,
                        b,
                        released,
                    )
                }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_controller_position_update(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                controller: __es::EsHandle,
            ) {
                unsafe { __es::screen_on_controller_position_update(screen, rs, controller) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_controller_disconnect(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                controller: __es::EsHandle,
            ) {
                unsafe { __es::screen_on_controller_disconnect(screen, rs, controller) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_radar_target_position_update(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                rt: __es::EsHandle,
            ) {
                unsafe { __es::screen_on_radar_target_position_update(screen, rs, rt) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_flightplan_disconnect(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                fp: __es::EsHandle,
            ) {
                unsafe { __es::screen_on_flightplan_disconnect(screen, rs, fp) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_flightplan_data_update(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                fp: __es::EsHandle,
            ) {
                unsafe { __es::screen_on_flightplan_data_update(screen, rs, fp) }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_controller_assigned_data_update(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                fp: __es::EsHandle,
                data_type: __es::c_int,
            ) {
                unsafe {
                    __es::screen_on_controller_assigned_data_update(screen, rs, fp, data_type)
                }
            }
            #[unsafe(no_mangle)]
            extern "C" fn rust_screen_on_flight_strip_pushed(
                screen: *mut __es::c_void,
                rs: __es::EsHandle,
                fp: __es::EsHandle,
                sender: *const __es::c_char,
                target: *const __es::c_char,
            ) {
                unsafe { __es::screen_on_flight_strip_pushed(screen, rs, fp, sender, target) }
            }
        };
    };
}

/// Emits one `#[no_mangle]` button-style screen-object callback arm. Separate
/// macro so the four near-identical arms stay readable.
#[doc(hidden)]
#[macro_export]
macro_rules! __es_screen_object_button_arm {
    ($export:ident, $dispatch:ident) => {
        #[unsafe(no_mangle)]
        #[allow(clippy::too_many_arguments)]
        extern "C" fn $export(
            screen: *mut $crate::__macro_support::c_void,
            rs: $crate::__macro_support::EsHandle,
            object_type: $crate::__macro_support::c_int,
            object_id: *const $crate::__macro_support::c_char,
            x: $crate::__macro_support::c_int,
            y: $crate::__macro_support::c_int,
            l: $crate::__macro_support::c_int,
            t: $crate::__macro_support::c_int,
            r: $crate::__macro_support::c_int,
            b: $crate::__macro_support::c_int,
            button: $crate::__macro_support::c_int,
        ) {
            unsafe {
                $crate::__macro_support::$dispatch(
                    screen,
                    rs,
                    object_type,
                    object_id,
                    x,
                    y,
                    l,
                    t,
                    r,
                    b,
                    button,
                )
            }
        }
    };
}
