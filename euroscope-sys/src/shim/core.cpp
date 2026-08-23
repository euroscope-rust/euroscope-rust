// Plugin lifecycle: the CPlugIn subclass, the exported entry points, and the
// shim -> Rust callback declarations. The per-class `*.cpp` files hold the
// Rust -> EuroScope (`es_*`) wrappers.

#include "common.h"

// ---------------------------------------------------------------------------
// Shim -> Rust callbacks, defined on the Rust side by `register_plugin!`.
// By-value handle params (CController/CFlightPlan/...) are forwarded as
// `void*` pointers to the stack temporary — valid only for the call.
// ---------------------------------------------------------------------------

// Plugin identity, provided by Rust before the CPlugIn base is constructed.
// All pointers must be NUL-terminated and outlive the plugin (Rust supplies
// 'static strings).
struct RustPluginMeta
{
  const char* name;
  const char* version;
  const char* author;
  const char* copyright;
};

extern "C"
{
  void rust_plugin_metadata(RustPluginMeta* out);
  void* rust_plugin_create(void* plugin);
  void rust_plugin_destroy(void* state);

  void rust_on_timer(void* state, int counter);
  void rust_on_controller_position_update(void* state, void* controller);
  void rust_on_controller_disconnect(void* state, void* controller);
  void rust_on_radar_target_position_update(void* state, void* radar_target);
  void rust_on_flightplan_disconnect(void* state, void* flightplan);
  void rust_on_flightplan_data_update(void* state, void* flightplan);
  void rust_on_plane_information_update(void* state,
                                        const char* callsign,
                                        const char* livery,
                                        const char* plane_type);
  void rust_on_controller_assigned_data_update(void* state,
                                               void* flightplan,
                                               int data_type);
  void rust_on_flight_strip_pushed(void* state,
                                   void* flightplan,
                                   const char* sender,
                                   const char* target);
  bool rust_on_compile_command(void* state, const char* command_line);
  void rust_on_frequency_chat(void* state,
                              const char* sender,
                              double frequency,
                              const char* message);
  void rust_on_private_chat(void* state,
                            const char* sender,
                            const char* receiver,
                            const char* message);
  void rust_on_refresh_fp_list(void* state, void* ac_list);
  void rust_on_new_metar(void* state,
                         const char* station,
                         const char* full_metar);
  void rust_on_function_call(void* state,
                             int function_id,
                             const char* item_string,
                             int point_x,
                             int point_y,
                             int area_left,
                             int area_top,
                             int area_right,
                             int area_bottom);
  void rust_on_runway_activity_changed(void* state);
  void rust_on_voice_transmit_started(void* state, bool on_primary);
  void rust_on_voice_transmit_ended(void* state, bool on_primary);
  void rust_on_voice_receive_started(void* state, void* channel);
  void rust_on_voice_receive_ended(void* state, void* channel);
  void rust_on_get_tag_item(void* state,
                            void* flightplan,
                            void* radar_target,
                            int item_code,
                            int tag_data,
                            char* item_string,
                            int* color_code,
                            unsigned long* rgb,
                            double* font_size);
  void* rust_on_radar_screen_created(void* state,
                                     const char* display_name,
                                     bool need_radar_content,
                                     bool geo_referenced,
                                     bool can_be_saved,
                                     bool can_be_created);
}

// ---------------------------------------------------------------------------
// The CPlugIn subclass. Every overridden virtual forwards to Rust; the C++
// compiler generates the correct vtable and destructor slot for us. Handle
// params are passed by address of the stack temporary (valid for the call).
// ---------------------------------------------------------------------------

class RustPlugIn : public CPlugIn
{
  void* m_state;

public:
  explicit RustPlugIn(const RustPluginMeta& meta)
    : CPlugIn(EuroScopePlugIn::COMPATIBILITY_CODE,
              meta.name,
              meta.version,
              meta.author,
              meta.copyright)
    , m_state(nullptr)
  {
    m_state = rust_plugin_create(this);
  }

  // Tear down the Rust state (which stops the plugin's worker threads)
  // exactly once. Idempotent so it is safe whether it runs from
  // EuroScopePlugInExit, from the destructor (if ES deletes the instance), or
  // both.
  void ShutdownRust()
  {
    if (m_state) {
      rust_plugin_destroy(m_state);
      m_state = nullptr;
    }
  }

  virtual ~RustPlugIn() { ShutdownRust(); }

  virtual void OnTimer(int Counter) { rust_on_timer(m_state, Counter); }

  virtual void OnControllerPositionUpdate(CController Controller)
  {
    rust_on_controller_position_update(m_state, &Controller);
  }

  virtual void OnControllerDisconnect(CController Controller)
  {
    rust_on_controller_disconnect(m_state, &Controller);
  }

  virtual void OnRadarTargetPositionUpdate(CRadarTarget RadarTarget)
  {
    rust_on_radar_target_position_update(m_state, &RadarTarget);
  }

  virtual void OnFlightPlanDisconnect(CFlightPlan FlightPlan)
  {
    rust_on_flightplan_disconnect(m_state, &FlightPlan);
  }

  virtual void OnFlightPlanFlightPlanDataUpdate(CFlightPlan FlightPlan)
  {
    rust_on_flightplan_data_update(m_state, &FlightPlan);
  }

  virtual void OnPlaneInformationUpdate(const char* sCallsign,
                                        const char* sLivery,
                                        const char* sPlaneType)
  {
    rust_on_plane_information_update(m_state, sCallsign, sLivery, sPlaneType);
  }

  virtual void OnFlightPlanControllerAssignedDataUpdate(CFlightPlan FlightPlan,
                                                        int DataType)
  {
    rust_on_controller_assigned_data_update(m_state, &FlightPlan, DataType);
  }

  virtual void OnFlightPlanFlightStripPushed(CFlightPlan FlightPlan,
                                             const char* sSenderController,
                                             const char* sTargetController)
  {
    rust_on_flight_strip_pushed(
      m_state, &FlightPlan, sSenderController, sTargetController);
  }

  virtual bool OnCompileCommand(const char* sCommandLine)
  {
    return rust_on_compile_command(m_state, sCommandLine);
  }

  virtual void OnCompileFrequencyChat(const char* sSenderCallsign,
                                      double Frequency,
                                      const char* sChatMessage)
  {
    rust_on_frequency_chat(m_state, sSenderCallsign, Frequency, sChatMessage);
  }

  virtual void OnCompilePrivateChat(const char* sSenderCallsign,
                                    const char* sReceiverCallsign,
                                    const char* sChatMessage)
  {
    rust_on_private_chat(
      m_state, sSenderCallsign, sReceiverCallsign, sChatMessage);
  }

  virtual void OnRefreshFpListContent(CFlightPlanList AcList)
  {
    rust_on_refresh_fp_list(m_state, &AcList);
  }

  virtual void OnNewMetarReceived(const char* sStation, const char* sFullMetar)
  {
    rust_on_new_metar(m_state, sStation, sFullMetar);
  }

  virtual void OnFunctionCall(int FunctionId,
                              const char* sItemString,
                              POINT Pt,
                              RECT Area)
  {
    rust_on_function_call(m_state,
                          FunctionId,
                          sItemString,
                          Pt.x,
                          Pt.y,
                          Area.left,
                          Area.top,
                          Area.right,
                          Area.bottom);
  }

  virtual void OnAirportRunwayActivityChanged(void)
  {
    rust_on_runway_activity_changed(m_state);
  }

  virtual void OnVoiceTransmitStarted(bool OnPrimary)
  {
    rust_on_voice_transmit_started(m_state, OnPrimary);
  }

  virtual void OnVoiceTransmitEnded(bool OnPrimary)
  {
    rust_on_voice_transmit_ended(m_state, OnPrimary);
  }

  virtual void OnVoiceReceiveStarted(CGrountToAirChannel Channel)
  {
    rust_on_voice_receive_started(m_state, &Channel);
  }

  virtual void OnVoiceReceiveEnded(CGrountToAirChannel Channel)
  {
    rust_on_voice_receive_ended(m_state, &Channel);
  }

  virtual void OnGetTagItem(CFlightPlan FlightPlan,
                            CRadarTarget RadarTarget,
                            int ItemCode,
                            int TagData,
                            char sItemString[16],
                            int* pColorCode,
                            COLORREF* pRGB,
                            double* pFontSize)
  {
    rust_on_get_tag_item(m_state,
                         &FlightPlan,
                         &RadarTarget,
                         ItemCode,
                         TagData,
                         sItemString,
                         pColorCode,
                         pRGB,
                         pFontSize);
  }

  virtual CRadarScreen* OnRadarScreenCreated(const char* sDisplayName,
                                             bool NeedRadarContent,
                                             bool GeoReferenced,
                                             bool CanBeSaved,
                                             bool CanBeCreated)
  {
    void* screen = rust_on_radar_screen_created(m_state,
                                                sDisplayName,
                                                NeedRadarContent,
                                                GeoReferenced,
                                                CanBeSaved,
                                                CanBeCreated);
    return screen ? rust_make_radar_screen(screen) : NULL;
  }
};

// ---------------------------------------------------------------------------
// Exported entry points. Declared exactly as in the header (NOT extern "C"),
// so MSVC mangles them identically to what EuroScope.exe looks up.
// ---------------------------------------------------------------------------

static RustPlugIn* g_plugin = nullptr;

void __declspec(dllexport)
EuroScopePlugInInit(EuroScopePlugIn::CPlugIn** ppPlugInInstance)
{
  RustPluginMeta meta = {};
  rust_plugin_metadata(&meta);
  g_plugin = new RustPlugIn(meta);
  *ppPlugInInstance = g_plugin;
}

void __declspec(dllexport)
EuroScopePlugInExit(void)
{
  // Whether ES frees the CPlugIn instance itself is ambiguous: the SDK docs
  // say a COMPATIBILITY_CODE plugin is deleted by ES (so deleting it here too
  // would double-free), yet the reference example still `delete`s it and
  // works in practice. So we stay correct under BOTH behaviours:
  //
  //  * Tear the Rust state down here explicitly (ShutdownRust is idempotent),
  //    so the rust resources are always removed even if ES never deletes us
  //    and thus never runs ~RustPlugIn().
  //  * Do NOT `delete g_plugin`: if ES owns the instance, deleting is a
  //    double-free (UB, may silently corrupt the heap rather than crash). The
  //    cost of not deleting is at most one leaked small C++ object per unload
  //    if ES does not own it — negligible.
  if (g_plugin) {
    g_plugin->ShutdownRust();
  }
  g_plugin = nullptr;
}

// Link seed. Nothing in Rust calls the two exports above -- EuroScope looks
// them up by name once the DLL is loaded -- so without a reference into this
// translation unit the linker would leave it in the static library and the
// finished plugin would export nothing.
//
// `register_plugin!` references this symbol, which drags this object (and with
// it the exports, and `radar_screen.cpp` via `rust_make_radar_screen`) into the
// link. Anything that links the shim *without* registering a plugin -- the
// crates' own test harnesses, say -- leaves it behind, along with this file's
// calls to the `rust_*` callbacks that only `register_plugin!` defines.
extern "C" void
es_shim_anchor(void)
{}
