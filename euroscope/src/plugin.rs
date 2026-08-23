use crate::{
    Context, Controller, ControllerDataType, FlightPlan, FlightPlanList, GroundToAirChannel, Point,
    RadarScreen, RadarTarget, Rect, TagData, TagItem,
};

/// A EuroScope plugin.
///
/// Implement this for your plugin type, then invoke
/// [`register_plugin!`](crate::register_plugin) once to wire it into
/// EuroScope's load/callback machinery.
///
/// Every `on_*` method has an empty default, so you override only the events
/// you care about. Each receives `&mut Context`, through which you call back
/// into EuroScope. **Never block** in a callback — offload slow work to a
/// worker thread.
pub trait Plugin: Sized + 'static {
    /// User-visible plugin name (shown in EuroScope's plugin list).
    const NAME: &'static str;
    /// Version string, e.g. `"1.0.3"`.
    const VERSION: &'static str;
    /// Author name.
    const AUTHOR: &'static str;
    /// Copyright / license line. Optional.
    const COPYRIGHT: &'static str = "";

    /// Construct the plugin. Called once when EuroScope loads the DLL. Do
    /// startup registration here (tag items, display types, lists) via `ctx`.
    fn new(ctx: &mut Context) -> Self;

    /// Called roughly once per second. `counter` increments each tick.
    fn on_timer(&mut self, ctx: &mut Context, counter: i32) {
        let _ = (ctx, counter);
    }

    /// A controller's position was updated.
    fn on_controller_position_update(&mut self, ctx: &mut Context, controller: Controller<'_>) {
        let _ = (ctx, controller);
    }

    /// A controller logged off or timed out.
    fn on_controller_disconnect(&mut self, ctx: &mut Context, controller: Controller<'_>) {
        let _ = (ctx, controller);
    }

    /// A radar target's position was updated.
    fn on_radar_target_position_update(
        &mut self,
        ctx: &mut Context,
        radar_target: RadarTarget<'_>,
    ) {
        let _ = (ctx, radar_target);
    }

    /// A flight plan disconnected (logged off or timed out).
    fn on_flight_plan_disconnect(&mut self, ctx: &mut Context, flight_plan: FlightPlan<'_>) {
        let _ = (ctx, flight_plan);
    }

    /// A flight plan's flight-plan data was updated.
    fn on_flight_plan_data_update(&mut self, ctx: &mut Context, flight_plan: FlightPlan<'_>) {
        let _ = (ctx, flight_plan);
    }

    /// A plane's information (livery/type) was updated.
    fn on_plane_information_update(
        &mut self,
        ctx: &mut Context,
        callsign: &str,
        livery: &str,
        plane_type: &str,
    ) {
        let _ = (ctx, callsign, livery, plane_type);
    }

    /// Controller-assigned data changed.
    fn on_controller_assigned_data_update(
        &mut self,
        ctx: &mut Context,
        flight_plan: FlightPlan<'_>,
        data_type: ControllerDataType,
    ) {
        let _ = (ctx, flight_plan, data_type);
    }

    /// A flight strip was pushed to another controller.
    fn on_flight_strip_pushed(
        &mut self,
        ctx: &mut Context,
        flight_plan: FlightPlan<'_>,
        sender: &str,
        target: &str,
    ) {
        let _ = (ctx, flight_plan, sender, target);
    }

    /// A `.command` line was entered that no other handler consumed. Return
    /// `true` if your plugin handled it.
    fn on_compile_command(&mut self, ctx: &mut Context, command_line: &str) -> bool {
        let _ = (ctx, command_line);
        false
    }

    /// A frequency (radio) chat message was received.
    fn on_frequency_chat(
        &mut self,
        ctx: &mut Context,
        sender: &str,
        frequency: f64,
        message: &str,
    ) {
        let _ = (ctx, sender, frequency, message);
    }

    /// A private chat message was received.
    fn on_private_chat(&mut self, ctx: &mut Context, sender: &str, receiver: &str, message: &str) {
        let _ = (ctx, sender, receiver, message);
    }

    /// A flight-plan list registered by this plugin needs its content
    /// refreshed.
    fn on_refresh_fp_list(&mut self, ctx: &mut Context, list: FlightPlanList<'_>) {
        let _ = (ctx, list);
    }

    /// A new METAR was received for a station.
    fn on_new_metar(&mut self, ctx: &mut Context, station: &str, full_metar: &str) {
        let _ = (ctx, station, full_metar);
    }

    /// A registered tag-item function was invoked. `point` is the click
    /// location; `area` is the item's rectangle — pass it to
    /// [`Context::open_popup_list`] / [`Context::open_popup_edit`] to show a
    /// menu or edit box at the click site.
    fn on_function_call(
        &mut self,
        ctx: &mut Context,
        function_id: i32,
        item_string: &str,
        point: Point,
        area: Rect,
    ) {
        let _ = (ctx, function_id, item_string, point, area);
    }

    /// Provide the content of a custom tag item you registered with
    /// [`Context::register_tag_item_type`]. At least one of `flight_plan` /
    /// `radar_target` is valid. `item_code` is your registered code; `tag_data`
    /// indicates what data is available. Fill `item` to render text/colour/font.
    fn on_get_tag_item(
        &mut self,
        ctx: &mut Context,
        flight_plan: FlightPlan<'_>,
        radar_target: RadarTarget<'_>,
        item_code: i32,
        tag_data: TagData,
        item: &mut TagItem,
    ) {
        let _ = (ctx, flight_plan, radar_target, item_code, tag_data, item);
    }

    /// EuroScope is creating a radar screen of a type you registered with
    /// [`Context::register_display_type`]. Return a boxed
    /// [`RadarScreen`](crate::RadarScreen) to drive it, or `None` to decline
    /// (e.g. when `display_name` isn't yours). EuroScope owns and drops the
    /// returned screen.
    fn on_radar_screen_created(
        &mut self,
        ctx: &mut Context,
        display_name: &str,
        need_radar_content: bool,
        geo_referenced: bool,
        can_be_saved: bool,
        can_be_created: bool,
    ) -> Option<Box<dyn RadarScreen>> {
        let _ = (
            ctx,
            display_name,
            need_radar_content,
            geo_referenced,
            can_be_saved,
            can_be_created,
        );
        None
    }

    /// Runway/airport activity (active runways) changed.
    fn on_runway_activity_changed(&mut self, ctx: &mut Context) {
        let _ = ctx;
    }

    /// The user started transmitting on voice (`on_primary`: primary frequency).
    fn on_voice_transmit_started(&mut self, ctx: &mut Context, on_primary: bool) {
        let _ = (ctx, on_primary);
    }

    /// The user stopped transmitting on voice.
    fn on_voice_transmit_ended(&mut self, ctx: &mut Context, on_primary: bool) {
        let _ = (ctx, on_primary);
    }

    /// Voice reception started on a ground-to-air channel.
    fn on_voice_receive_started(&mut self, ctx: &mut Context, channel: GroundToAirChannel<'_>) {
        let _ = (ctx, channel);
    }

    /// Voice reception ended on a ground-to-air channel.
    fn on_voice_receive_ended(&mut self, ctx: &mut Context, channel: GroundToAirChannel<'_>) {
        let _ = (ctx, channel);
    }
}
