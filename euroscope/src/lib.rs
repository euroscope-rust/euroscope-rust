//! Safe, idiomatic Rust interface for writing [EuroScope] plugins.
//!
//! Implement the [`Plugin`] trait for your type and register it with
//! [`register_plugin!`]. The macro emits the C ABI glue and the two exported
//! entry points EuroScope looks for when it loads your DLL.
//!
//! ```ignore
//! use euroscope::{Context, FlightPlan, Plugin, register_plugin};
//!
//! #[derive(Default)]
//! struct Hello;
//!
//! impl Plugin for Hello {
//!     const NAME: &'static str = "Hello";
//!     const VERSION: &'static str = "0.1.0";
//!     const AUTHOR: &'static str = "me";
//!
//!     fn new(_ctx: &mut Context) -> Self {
//!         Self
//!     }
//!
//!     fn on_flight_plan_data_update(&mut self, _ctx: &mut Context, fp: FlightPlan<'_>) {
//!         let _callsign = fp.callsign();
//!     }
//! }
//!
//! register_plugin!(Hello);
//! ```
//!
//! ## Build
//!
//! EuroScope is 32-bit, so plugins must be built as a `cdylib` for
//! `i686-pc-windows-msvc` (the workspace pins this in `.cargo/config.toml`).
//!
//! ## Threading
//!
//! Every callback runs on EuroScope's main thread. **Never block** inside one —
//! offload slow work (network, IPC) to a worker thread and let callbacks only
//! enqueue to it.
//!
//! [EuroScope]: https://www.euroscope.hu/

pub mod api;
mod context;
mod enums;
mod plugin;
pub mod tag_codes;
#[cfg(feature = "tracing")]
pub mod tracing;
mod utils;

pub use api::{
    Controller, ControllerAssignedData, ControllerHandle, Controllers, ExtractedRoute, FlightPlan,
    FlightPlanData, FlightPlanHandle, FlightPlanList, FlightPlans, GroundToAirChannel,
    GroundToAirChannelHandle, GroundToAirChannels, Hdc, Point, Position, PositionHistory,
    PositionPredictions, RadarScreen, RadarScreenContext, RadarTarget, RadarTargetHandle,
    RadarTargetPosition, RadarTargetPositionHandle, RadarTargets, Rect, RegisteredFlightPlanList,
    SectorElement, SectorElementHandle, SectorElements, TagItem,
};
pub use context::Context;
pub use enums::{
    AircraftCategory, AirwayClass, CommunicationType, ConnectionType, ControllerDataType,
    ControllerRating, CoordinationState, EngineType, Facility, FlightPlanState, MouseButton,
    NavCapability, PopupCheck, RefreshPhase, SectorElementType, SimulationState, TagColor, TagData,
    WakeCategory,
};
pub use plugin::Plugin;
pub use utils::get_plugin_path;

#[doc(hidden)]
pub mod __macro_support;
