//! `extern "C"` declarations for the shim's `es_*` wrappers, one module per
//! EuroScope class. Everything is re-exported flat, so callers use
//! `euroscope_sys::es_flightplan_callsign(..)` regardless of module.

mod controller;
mod controller_assigned_data;
mod extracted_route;
mod flight_plan;
mod flight_plan_data;
mod flight_plan_list;
mod ground_to_air_channel;
mod plugin;
mod position;
mod position_predictions;
mod radar_screen;
mod radar_target;
mod radar_target_position;
mod sector_element;

pub use controller::*;
pub use controller_assigned_data::*;
pub use extracted_route::*;
pub use flight_plan::*;
pub use flight_plan_data::*;
pub use flight_plan_list::*;
pub use ground_to_air_channel::*;
pub use plugin::*;
pub use position::*;
pub use position_predictions::*;
pub use radar_screen::*;
pub use radar_target::*;
pub use radar_target_position::*;
pub use sector_element::*;
