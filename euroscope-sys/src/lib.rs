//! Raw FFI surface for the EuroScope plugin SDK.
//!
//! This crate owns the C++ glue shim (`src/shim/`, compiled by `build.rs`) and
//! exposes its flat `extern "C"` entry points to Rust. It is deliberately
//! `unsafe` and untyped — the safe, idiomatic API lives in the `euroscope`
//! crate, which is what application plugins should depend on.
//!
//! ## The two halves of the boundary
//!
//! * **`es_*` (module [`ffi`]):** wrappers *the shim provides*, i.e. Rust -> ES calls. One module
//!   per EuroScope class, re-exported flat here.
//! * **`rust_*` (see [`callbacks`]):** callbacks *the shim expects Rust to provide*, i.e. ES ->
//!   Rust. `euroscope::register_plugin!` emits `#[no_mangle]` definitions for these; the signatures
//!   are documented there.

use std::ffi::{c_char, c_void};

pub mod ffi;
pub use ffi::*;

/// An opaque pointer to an EuroScope handle object (`CFlightPlan`, `CPlugIn`,
/// `CController`, …). Only valid for as long as EuroScope says — usually the
/// callback that produced it. The safe `euroscope` crate encodes those rules.
pub type EsHandle = *mut c_void;

/// Opaque pointer to a live `CFlightPlan`.
pub type FlightPlanPtr = EsHandle;
/// Opaque pointer to the shim's `CPlugIn` instance.
pub type PluginPtr = EsHandle;

/// Plugin identity handed to the shim before the `CPlugIn` base is
/// constructed. Layout must match `RustPluginMeta` in `shim/core.cpp`.
#[repr(C)]
pub struct RustPluginMeta {
    pub name: *const c_char,
    pub version: *const c_char,
    pub author: *const c_char,
    pub copyright: *const c_char,
}
