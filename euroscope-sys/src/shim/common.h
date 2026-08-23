// Shared prelude for every shim translation unit.
//
// EuroScope is 32-bit and its header does not include <windows.h> itself, yet
// it uses RECT / HDC / POINT / COLORREF — so we must pull Windows in first.
//
// This is the only place the C++ ABI is touched (thiscall, name mangling,
// vtable layout, by-value handle passing). Rust sees only the flat `extern
// "C"` wrappers each .cpp defines.

#pragma once

#include <windows.h>

#include "EuroScopePlugIn.h"

using namespace EuroScopePlugIn;

// Handle-cast helpers. EuroScope's data classes are thin handles (one
// pointer), passed to Rust as opaque `void*`. `AS(T, p)` recovers a `T*`; the
// wrappers then call the by-value/by-ref SDK methods on it.
#define ES_AS(Type, ptr) (static_cast<Type*>(ptr))

// Heap-own a by-value handle so a pointer to it survives the FFI boundary.
// EuroScope's Select*/GetCorrelated*/Register* methods return handle objects
// BY VALUE, whose internal index we cannot read across the flat C ABI. So we
// copy the returned handle onto the heap (`new T(value)`) and hand Rust the
// pointer, or nullptr when the handle is invalid. Rust owns the copy and frees
// it with the matching `es_*_free`. Methods on the copy work because the
// handle is just an index into live EuroScope data.
template<typename T>
inline void*
own(const T& value)
{
  return value.IsValid() ? new T(value) : nullptr;
}

// Defined in radar_screen.cpp: wraps a Rust radar-screen state in a
// CRadarScreen subclass that EuroScope owns (freed via
// OnAsrContentToBeClosed).
CRadarScreen*
rust_make_radar_screen(void* rust_screen);
