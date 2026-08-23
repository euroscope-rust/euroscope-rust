// Rust -> EuroScope wrappers for CPlugIn-level API.

#include "common.h"

extern "C" void
es_plugin_display_user_message(void* plugin,
                               const char* handler,
                               const char* sender,
                               const char* message,
                               bool show_handler,
                               bool show_unread,
                               bool show_unread_even_if_busy,
                               bool start_flashing,
                               bool need_confirmation)
{
  ES_AS(CPlugIn, plugin)
    ->DisplayUserMessage(handler,
                         sender,
                         message,
                         show_handler,
                         show_unread,
                         show_unread_even_if_busy,
                         start_flashing,
                         need_confirmation);
}

// Connection state. The "myself" controller is reached via
// es_plugin_controller_myself (controller.cpp) as a proper CController handle.

extern "C" int
es_plugin_connection_type(void* plugin)
{
  return ES_AS(CPlugIn, plugin)->GetConnectionType();
}

// Traffic counts. Iteration happens here (one FFI call, no per-aircraft
// round-trips) on EuroScope's main thread, where OnTimer invokes these.

extern "C" int
es_plugin_count_tracked_by_me(void* plugin)
{
  CPlugIn* p = ES_AS(CPlugIn, plugin);
  int count = 0;
  for (CFlightPlan fp = p->FlightPlanSelectFirst(); fp.IsValid();
       fp = p->FlightPlanSelectNext(fp)) {
    if (fp.GetTrackingControllerIsMe()) {
      ++count;
    }
  }
  return count;
}

extern "C" int
es_plugin_count_radar_targets(void* plugin)
{
  CPlugIn* p = ES_AS(CPlugIn, plugin);
  int count = 0;
  for (CRadarTarget rt = p->RadarTargetSelectFirst(); rt.IsValid();
       rt = p->RadarTargetSelectNext(rt)) {
    ++count;
  }
  return count;
}

// Registration.

extern "C" void
es_plugin_register_tag_item_type(void* plugin, const char* name, int code)
{
  ES_AS(CPlugIn, plugin)->RegisterTagItemType(name, code);
}

extern "C" void
es_plugin_register_tag_item_function(void* plugin, const char* name, int code)
{
  ES_AS(CPlugIn, plugin)->RegisterTagItemFunction(name, code);
}

extern "C" void
es_plugin_add_alias(void* plugin, const char* name, const char* value)
{
  ES_AS(CPlugIn, plugin)->AddAlias(name, value);
}

extern "C" const char*
es_plugin_name(void* plugin)
{
  return ES_AS(CPlugIn, plugin)->GetPlugInName();
}

extern "C" void
es_plugin_register_toolbar_item(void* plugin, int item_id, const char* name)
{
  ES_AS(CPlugIn, plugin)->RegisterToolbarItem(item_id, name);
}

extern "C" void
es_plugin_refresh_toolbar(void* plugin, bool resize_too)
{
  ES_AS(CPlugIn, plugin)->RefreshToolbar(resize_too);
}

extern "C" void
es_plugin_select_active_sectorfile(void* plugin)
{
  ES_AS(CPlugIn, plugin)->SelectActiveSectorfile();
}

extern "C" void
es_plugin_select_screen_sectorfile(void* plugin, void* screen)
{
  ES_AS(CPlugIn, plugin)->SelectScreenSectorfile(ES_AS(CRadarScreen, screen));
}

extern "C" void
es_plugin_register_display_type(void* plugin,
                                const char* name,
                                bool need_radar_content,
                                bool geo_referenced,
                                bool can_be_saved,
                                bool can_be_created)
{
  ES_AS(CPlugIn, plugin)
    ->RegisterDisplayType(
      name, need_radar_content, geo_referenced, can_be_saved, can_be_created);
}

// Popups. `Area` is typically the rectangle delivered to OnFunctionCall so the
// popup appears at the click site.

extern "C" void
es_plugin_open_popup_list(void* plugin,
                          int left,
                          int top,
                          int right,
                          int bottom,
                          const char* title,
                          int column_number)
{
  RECT area = { left, top, right, bottom };
  ES_AS(CPlugIn, plugin)->OpenPopupList(area, title, column_number);
}

extern "C" void
es_plugin_open_popup_edit(void* plugin,
                          int left,
                          int top,
                          int right,
                          int bottom,
                          int function_id,
                          const char* initial_value)
{
  RECT area = { left, top, right, bottom };
  ES_AS(CPlugIn, plugin)->OpenPopupEdit(area, function_id, initial_value);
}

extern "C" void
es_plugin_add_popup_list_element(void* plugin,
                                 const char* string1,
                                 const char* string2,
                                 int function_id,
                                 bool selected,
                                 int checked,
                                 bool disabled,
                                 bool fixed)
{
  ES_AS(CPlugIn, plugin)
    ->AddPopupListElement(
      string1, string2, function_id, selected, checked, disabled, fixed);
}

// Misc CPlugIn scalars & settings.

extern "C" int
es_plugin_transition_altitude(void* plugin)
{
  return ES_AS(CPlugIn, plugin)->GetTransitionAltitude();
}

extern "C" const char*
es_plugin_get_data_from_settings(void* plugin, const char* key)
{
  return ES_AS(CPlugIn, plugin)->GetDataFromSettings(key);
}

extern "C" void
es_plugin_save_data_to_settings(void* plugin,
                                const char* key,
                                const char* description,
                                const char* value)
{
  ES_AS(CPlugIn, plugin)->SaveDataToSettings(key, description, value);
}
