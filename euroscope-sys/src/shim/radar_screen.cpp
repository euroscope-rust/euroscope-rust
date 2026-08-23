// CRadarScreen subclass (a second vtable-subclassing point, like RustPlugIn)
// plus the Rust -> EuroScope wrappers for CRadarScreen methods.
//
// EuroScope owns the screen: it is created in OnRadarScreenCreated and must
// `delete this` from OnAsrContentToBeClosed. We drop the Rust state there too.

#include "common.h"

// Shim -> Rust screen callbacks, defined by register_plugin!. `screen` is the
// Rust screen state; `rs` is the CRadarScreen* (for es_screen_* calls).
extern "C"
{
  void rust_screen_destroy(void* screen);
  void rust_screen_on_asr_loaded(void* screen, void* rs, bool loaded);
  void rust_screen_on_asr_to_be_saved(void* screen, void* rs);
  void rust_screen_on_refresh(void* screen, void* rs, void* hdc, int phase);
  bool rust_screen_on_compile_command(void* screen,
                                      void* rs,
                                      const char* command);
  void rust_screen_on_function_call(void* screen,
                                    void* rs,
                                    int function_id,
                                    const char* item_string,
                                    int x,
                                    int y,
                                    int l,
                                    int t,
                                    int r,
                                    int b);
  void rust_screen_on_object_over(void* screen,
                                  void* rs,
                                  int object_type,
                                  const char* object_id,
                                  int x,
                                  int y,
                                  int l,
                                  int t,
                                  int r,
                                  int b);
  void rust_screen_on_object_button_down(void* screen,
                                         void* rs,
                                         int object_type,
                                         const char* object_id,
                                         int x,
                                         int y,
                                         int l,
                                         int t,
                                         int r,
                                         int b,
                                         int button);
  void rust_screen_on_object_button_up(void* screen,
                                       void* rs,
                                       int object_type,
                                       const char* object_id,
                                       int x,
                                       int y,
                                       int l,
                                       int t,
                                       int r,
                                       int b,
                                       int button);
  void rust_screen_on_object_click(void* screen,
                                   void* rs,
                                   int object_type,
                                   const char* object_id,
                                   int x,
                                   int y,
                                   int l,
                                   int t,
                                   int r,
                                   int b,
                                   int button);
  void rust_screen_on_object_double_click(void* screen,
                                          void* rs,
                                          int object_type,
                                          const char* object_id,
                                          int x,
                                          int y,
                                          int l,
                                          int t,
                                          int r,
                                          int b,
                                          int button);
  void rust_screen_on_object_move(void* screen,
                                  void* rs,
                                  int object_type,
                                  const char* object_id,
                                  int x,
                                  int y,
                                  int l,
                                  int t,
                                  int r,
                                  int b,
                                  bool released);
  void rust_screen_on_controller_position_update(void* screen,
                                                 void* rs,
                                                 void* controller);
  void rust_screen_on_controller_disconnect(void* screen,
                                            void* rs,
                                            void* controller);
  void rust_screen_on_radar_target_position_update(void* screen,
                                                   void* rs,
                                                   void* radar_target);
  void rust_screen_on_flightplan_disconnect(void* screen,
                                            void* rs,
                                            void* flightplan);
  void rust_screen_on_flightplan_data_update(void* screen,
                                             void* rs,
                                             void* flightplan);
  void rust_screen_on_controller_assigned_data_update(void* screen,
                                                      void* rs,
                                                      void* flightplan,
                                                      int data_type);
  void rust_screen_on_flight_strip_pushed(void* screen,
                                          void* rs,
                                          void* flightplan,
                                          const char* sender,
                                          const char* target);
}

class RustRadarScreen : public CRadarScreen
{
  void* m_screen;

public:
  explicit RustRadarScreen(void* screen)
    : m_screen(screen)
  {
  }

  virtual void OnAsrContentLoaded(bool Loaded)
  {
    rust_screen_on_asr_loaded(m_screen, this, Loaded);
  }

  virtual void OnAsrContentToBeSaved(void)
  {
    rust_screen_on_asr_to_be_saved(m_screen, this);
  }

  virtual void OnRefresh(HDC hDC, int Phase)
  {
    rust_screen_on_refresh(m_screen, this, hDC, Phase);
  }

  virtual void OnAsrContentToBeClosed(void)
  {
    rust_screen_destroy(m_screen);
    delete this;
  }

  virtual bool OnCompileCommand(const char* sCommandLine)
  {
    return rust_screen_on_compile_command(m_screen, this, sCommandLine);
  }

  virtual void OnFunctionCall(int FunctionId,
                              const char* sItemString,
                              POINT Pt,
                              RECT Area)
  {
    rust_screen_on_function_call(m_screen,
                                 this,
                                 FunctionId,
                                 sItemString,
                                 Pt.x,
                                 Pt.y,
                                 Area.left,
                                 Area.top,
                                 Area.right,
                                 Area.bottom);
  }

  virtual void OnOverScreenObject(int ObjectType,
                                  const char* sObjectId,
                                  POINT Pt,
                                  RECT Area)
  {
    rust_screen_on_object_over(m_screen,
                               this,
                               ObjectType,
                               sObjectId,
                               Pt.x,
                               Pt.y,
                               Area.left,
                               Area.top,
                               Area.right,
                               Area.bottom);
  }

  virtual void OnButtonDownScreenObject(int ObjectType,
                                        const char* sObjectId,
                                        POINT Pt,
                                        RECT Area,
                                        int Button)
  {
    rust_screen_on_object_button_down(m_screen,
                                      this,
                                      ObjectType,
                                      sObjectId,
                                      Pt.x,
                                      Pt.y,
                                      Area.left,
                                      Area.top,
                                      Area.right,
                                      Area.bottom,
                                      Button);
  }

  virtual void OnButtonUpScreenObject(int ObjectType,
                                      const char* sObjectId,
                                      POINT Pt,
                                      RECT Area,
                                      int Button)
  {
    rust_screen_on_object_button_up(m_screen,
                                    this,
                                    ObjectType,
                                    sObjectId,
                                    Pt.x,
                                    Pt.y,
                                    Area.left,
                                    Area.top,
                                    Area.right,
                                    Area.bottom,
                                    Button);
  }

  virtual void OnClickScreenObject(int ObjectType,
                                   const char* sObjectId,
                                   POINT Pt,
                                   RECT Area,
                                   int Button)
  {
    rust_screen_on_object_click(m_screen,
                                this,
                                ObjectType,
                                sObjectId,
                                Pt.x,
                                Pt.y,
                                Area.left,
                                Area.top,
                                Area.right,
                                Area.bottom,
                                Button);
  }

  virtual void OnDoubleClickScreenObject(int ObjectType,
                                         const char* sObjectId,
                                         POINT Pt,
                                         RECT Area,
                                         int Button)
  {
    rust_screen_on_object_double_click(m_screen,
                                       this,
                                       ObjectType,
                                       sObjectId,
                                       Pt.x,
                                       Pt.y,
                                       Area.left,
                                       Area.top,
                                       Area.right,
                                       Area.bottom,
                                       Button);
  }

  virtual void OnMoveScreenObject(int ObjectType,
                                  const char* sObjectId,
                                  POINT Pt,
                                  RECT Area,
                                  bool Released)
  {
    rust_screen_on_object_move(m_screen,
                               this,
                               ObjectType,
                               sObjectId,
                               Pt.x,
                               Pt.y,
                               Area.left,
                               Area.top,
                               Area.right,
                               Area.bottom,
                               Released);
  }

  virtual void OnControllerPositionUpdate(CController Controller)
  {
    rust_screen_on_controller_position_update(m_screen, this, &Controller);
  }

  virtual void OnControllerDisconnect(CController Controller)
  {
    rust_screen_on_controller_disconnect(m_screen, this, &Controller);
  }

  virtual void OnRadarTargetPositionUpdate(CRadarTarget RadarTarget)
  {
    rust_screen_on_radar_target_position_update(m_screen, this, &RadarTarget);
  }

  virtual void OnFlightPlanDisconnect(CFlightPlan FlightPlan)
  {
    rust_screen_on_flightplan_disconnect(m_screen, this, &FlightPlan);
  }

  virtual void OnFlightPlanFlightPlanDataUpdate(CFlightPlan FlightPlan)
  {
    rust_screen_on_flightplan_data_update(m_screen, this, &FlightPlan);
  }

  virtual void OnFlightPlanControllerAssignedDataUpdate(CFlightPlan FlightPlan,
                                                        int DataType)
  {
    rust_screen_on_controller_assigned_data_update(
      m_screen, this, &FlightPlan, DataType);
  }

  virtual void OnFlightPlanFlightStripPushed(CFlightPlan FlightPlan,
                                             const char* sSender,
                                             const char* sTarget)
  {
    rust_screen_on_flight_strip_pushed(
      m_screen, this, &FlightPlan, sSender, sTarget);
  }
};

CRadarScreen*
rust_make_radar_screen(void* rust_screen)
{
  return new RustRadarScreen(rust_screen);
}

// --- Rust -> EuroScope: CRadarScreen methods -----------------------------

static void
put_rect(RECT a, int* l, int* t, int* r, int* b)
{
  *l = a.left;
  *t = a.top;
  *r = a.right;
  *b = a.bottom;
}

extern "C" void*
es_screen_get_plugin(void* screen)
{
  return ES_AS(CRadarScreen, screen)->GetPlugIn();
}

extern "C" void
es_screen_radar_area(void* screen, int* l, int* t, int* r, int* b)
{
  put_rect(ES_AS(CRadarScreen, screen)->GetRadarArea(), l, t, r, b);
}

extern "C" void
es_screen_toolbar_area(void* screen, int* l, int* t, int* r, int* b)
{
  put_rect(ES_AS(CRadarScreen, screen)->GetToolbarArea(), l, t, r, b);
}

extern "C" void
es_screen_chat_area(void* screen, int* l, int* t, int* r, int* b)
{
  put_rect(ES_AS(CRadarScreen, screen)->GetChatArea(), l, t, r, b);
}

extern "C" void
es_screen_position_to_pixel(void* screen,
                            double lat,
                            double lon,
                            int* x,
                            int* y)
{
  CPosition p;
  p.m_Latitude = lat;
  p.m_Longitude = lon;
  POINT pt = ES_AS(CRadarScreen, screen)->ConvertCoordFromPositionToPixel(p);
  *x = pt.x;
  *y = pt.y;
}

extern "C" void
es_screen_pixel_to_position(void* screen,
                            int x,
                            int y,
                            double* lat,
                            double* lon)
{
  POINT pt = { x, y };
  CPosition p =
    ES_AS(CRadarScreen, screen)->ConvertCoordFromPixelToPosition(pt);
  *lat = p.m_Latitude;
  *lon = p.m_Longitude;
}

extern "C" void
es_screen_add_screen_object(void* screen,
                            int object_type,
                            const char* object_id,
                            int l,
                            int t,
                            int r,
                            int b,
                            bool moveable,
                            const char* message)
{
  RECT area = { l, t, r, b };
  ES_AS(CRadarScreen, screen)
    ->AddScreenObject(object_type, object_id, area, moveable, message);
}

extern "C" void
es_screen_request_refresh(void* screen)
{
  ES_AS(CRadarScreen, screen)->RequestRefresh();
}

extern "C" void
es_screen_refresh_map_content(void* screen)
{
  ES_AS(CRadarScreen, screen)->RefreshMapContent();
}

extern "C" const char*
es_screen_get_data_from_asr(void* screen, const char* name)
{
  return ES_AS(CRadarScreen, screen)->GetDataFromAsr(name);
}

extern "C" void
es_screen_save_data_to_asr(void* screen,
                           const char* name,
                           const char* description,
                           const char* value)
{
  ES_AS(CRadarScreen, screen)->SaveDataToAsr(name, description, value);
}

extern "C" void
es_screen_get_display_area(void* screen,
                           double* ld_lat,
                           double* ld_lon,
                           double* ru_lat,
                           double* ru_lon)
{
  CPosition ld, ru;
  ES_AS(CRadarScreen, screen)->GetDisplayArea(&ld, &ru);
  *ld_lat = ld.m_Latitude;
  *ld_lon = ld.m_Longitude;
  *ru_lat = ru.m_Latitude;
  *ru_lon = ru.m_Longitude;
}

extern "C" void
es_screen_set_display_area(void* screen,
                           double ld_lat,
                           double ld_lon,
                           double ru_lat,
                           double ru_lon)
{
  CPosition ld, ru;
  ld.m_Latitude = ld_lat;
  ld.m_Longitude = ld_lon;
  ru.m_Latitude = ru_lat;
  ru.m_Longitude = ru_lon;
  ES_AS(CRadarScreen, screen)->SetDisplayArea(ld, ru);
}

extern "C" void
es_screen_show_sector_element(void* screen,
                              void* element,
                              const char* component,
                              bool show)
{
  ES_AS(CRadarScreen, screen)
    ->ShowSectorFileElement(*ES_AS(CSectorElement, element), component, show);
}

extern "C" void
es_screen_start_tag_function(void* screen,
                             const char* callsign,
                             const char* item_plugin,
                             int item_code,
                             const char* item_string,
                             const char* function_plugin,
                             int function_id,
                             int x,
                             int y,
                             int l,
                             int t,
                             int r,
                             int b)
{
  POINT pt = { x, y };
  RECT area = { l, t, r, b };
  ES_AS(CRadarScreen, screen)
    ->StartTagFunction(callsign,
                       item_plugin,
                       item_code,
                       item_string,
                       function_plugin,
                       function_id,
                       pt,
                       area);
}
