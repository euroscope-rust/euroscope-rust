// Rust -> EuroScope wrappers for CGrountToAirChannel.

#include "common.h"

extern "C" bool
es_gtachannel_is_valid(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->IsValid();
}

extern "C" const char*
es_gtachannel_name(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetName();
}

extern "C" double
es_gtachannel_frequency(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetFrequency();
}

extern "C" const char*
es_gtachannel_voice_server(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetVoiceServer();
}

extern "C" const char*
es_gtachannel_voice_channel(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetVoiceChannel();
}

extern "C" bool
es_gtachannel_is_primary(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetIsPrimary();
}

extern "C" bool
es_gtachannel_is_atis(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetIsAtis();
}

extern "C" bool
es_gtachannel_is_text_receive_on(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetIsTextReceiveOn();
}

extern "C" bool
es_gtachannel_is_text_transmit_on(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetIsTextTransmitOn();
}

extern "C" bool
es_gtachannel_is_voice_receive_on(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetIsVoiceReceiveOn();
}

extern "C" bool
es_gtachannel_is_voice_transmit_on(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetIsVoiceTransmitOn();
}

extern "C" bool
es_gtachannel_is_voice_connected(void* h)
{
  return ES_AS(CGrountToAirChannel, h)->GetIsVoiceConnected();
}

extern "C" void
es_gtachannel_toggle_primary(void* h)
{
  ES_AS(CGrountToAirChannel, h)->TogglePrimary();
}

extern "C" void
es_gtachannel_toggle_atis(void* h)
{
  ES_AS(CGrountToAirChannel, h)->ToggleAtis();
}

extern "C" void
es_gtachannel_toggle_text_receive(void* h)
{
  ES_AS(CGrountToAirChannel, h)->ToggleTextReceive();
}

extern "C" void
es_gtachannel_toggle_text_transmit(void* h)
{
  ES_AS(CGrountToAirChannel, h)->ToggleTextTransmit();
}

extern "C" void
es_gtachannel_toggle_voice_receive(void* h)
{
  ES_AS(CGrountToAirChannel, h)->ToggleVoiceReceive();
}

extern "C" void
es_gtachannel_toggle_voice_transmit(void* h)
{
  ES_AS(CGrountToAirChannel, h)->ToggleVoiceTransmit();
}

// --- Owned selectors (CPlugIn -> CGrountToAirChannel) ---------------------

extern "C" void
es_gtachannel_free(void* ch)
{
  delete ES_AS(CGrountToAirChannel, ch);
}

extern "C" void*
es_plugin_gtachannel_select_first(void* plugin)
{
  return own(ES_AS(CPlugIn, plugin)->GroundToArChannelSelectFirst());
}

extern "C" void*
es_plugin_gtachannel_select_next(void* plugin, void* current)
{
  return own(
    ES_AS(CPlugIn, plugin)
      ->GroundToArChannelSelectNext(*ES_AS(CGrountToAirChannel, current)));
}
