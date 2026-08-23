//! Safe handle wrappers for EuroScope's data classes, one module per class.
//!
//! Every wrapper is a thin `Copy` handle over an opaque EuroScope pointer,
//! carrying a `'cb` lifetime tying it to the callback that produced it — the
//! SDK documents these references as valid only "inside the block you are
//! querying", so storing one past the callback is a use-after-free. The
//! lifetime makes that a borrow-check error in safe code.
//!
//! Types are re-exported flat from the crate root.

pub mod controller;
pub mod controller_assigned_data;
pub mod extracted_route;
pub mod flight_plan;
pub mod flight_plan_data;
pub mod flight_plan_list;
pub mod geometry;
pub mod ground_to_air_channel;
pub(crate) mod handle;
pub mod position;
pub mod position_predictions;
pub mod radar_screen;
pub mod radar_target;
pub mod radar_target_position;
pub mod sector_element;
pub mod tag;

pub use controller::{Controller, ControllerHandle, Controllers};
pub use controller_assigned_data::ControllerAssignedData;
pub use extracted_route::ExtractedRoute;
pub use flight_plan::{FlightPlan, FlightPlanHandle, FlightPlans};
pub use flight_plan_data::FlightPlanData;
pub use flight_plan_list::{FlightPlanList, RegisteredFlightPlanList};
pub use geometry::{Point, Rect};
pub use ground_to_air_channel::{
    GroundToAirChannel, GroundToAirChannelHandle, GroundToAirChannels,
};
pub use position::Position;
pub use position_predictions::PositionPredictions;
pub use radar_screen::{Hdc, RadarScreen, RadarScreenContext};
pub use radar_target::{RadarTarget, RadarTargetHandle, RadarTargets};
pub use radar_target_position::{PositionHistory, RadarTargetPosition, RadarTargetPositionHandle};
pub use sector_element::{SectorElement, SectorElementHandle, SectorElements};
pub use tag::TagItem;
