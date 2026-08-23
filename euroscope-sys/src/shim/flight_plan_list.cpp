// Rust -> EuroScope wrappers for CFlightPlanList.

#include "common.h"

extern "C" bool
es_fplist_is_valid(void* h)
{
  return ES_AS(CFlightPlanList, h)->IsValid();
}

extern "C" int
es_fplist_column_number(void* h)
{
  return ES_AS(CFlightPlanList, h)->GetColumnNumber();
}

extern "C" void
es_fplist_delete_all_columns(void* h)
{
  ES_AS(CFlightPlanList, h)->DeleteAllColumns();
}

extern "C" void
es_fplist_add_column_definition(void* h,
                                const char* column_title,
                                int width,
                                bool centered,
                                const char* item_provider,
                                int item_code,
                                const char* left_button_function_provider,
                                int left_button_function,
                                const char* right_button_function_provider,
                                int right_button_function)
{
  ES_AS(CFlightPlanList, h)
    ->AddColumnDefinition(column_title,
                          width,
                          centered,
                          item_provider,
                          item_code,
                          left_button_function_provider,
                          left_button_function,
                          right_button_function_provider,
                          right_button_function);
}

extern "C" void
es_fplist_add_fp_to_the_list(void* h, void* fp)
{
  ES_AS(CFlightPlanList, h)->AddFpToTheList(*ES_AS(CFlightPlan, fp));
}

extern "C" void
es_fplist_remove_fp_from_the_list(void* h, void* fp)
{
  ES_AS(CFlightPlanList, h)->RemoveFpFromTheList(*ES_AS(CFlightPlan, fp));
}

extern "C" void
es_fplist_show_fp_list(void* h, bool show)
{
  ES_AS(CFlightPlanList, h)->ShowFpList(show);
}

// --- Registration (CPlugIn -> CFlightPlanList) ----------------------------

extern "C" void
es_fplist_free(void* list)
{
  delete ES_AS(CFlightPlanList, list);
}

extern "C" void*
es_plugin_register_fp_list(void* plugin, const char* name)
{
  return own(ES_AS(CPlugIn, plugin)->RegisterFpList(name));
}
