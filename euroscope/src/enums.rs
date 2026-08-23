//! Rust enums mirroring the EuroScope SDK's integer constant groups
//! (`CONNECTION_TYPE_*`, and — as the API grows — `TAG_*`, `POPUP_*`, the
//! coordination/handoff states, …). Each has a `from_raw` and an `Other`
//! variant for forward-compatibility.
//!
//! A handful of fields the SDK exposes as a single ANSI character (the ICAO
//! wake/type/engine letters, the communication-type code) are modelled the same
//! way, mapping to/from `c_char` with an `Other(c_char)` escape hatch.

use std::ffi::c_char;

/// How EuroScope is connected to the network. Mirrors `CONNECTION_TYPE_*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// Not connected.
    None,
    /// Direct connection to the network.
    Direct,
    /// Connection via a proxy.
    ViaProxy,
    /// Connected as a simulator server.
    SimulatorServer,
    /// Playback of a recorded session.
    Playback,
    /// Connected as a simulator client.
    SimulatorClient,
    /// Sweatbox (training) server.
    Sweatbox,
    /// An unrecognized connection code (forward-compatibility).
    Unknown(i32),
}

impl ConnectionType {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Direct,
            2 => Self::ViaProxy,
            3 => Self::SimulatorServer,
            4 => Self::Playback,
            5 => Self::SimulatorClient,
            6 => Self::Sweatbox,
            other => Self::Unknown(other),
        }
    }

    /// Whether EuroScope is connected in any way (i.e. not [`None`](Self::None)).
    pub fn is_connected(self) -> bool {
        self != Self::None
    }
}

/// A flight plan's correlation/coordination state, from `CFlightPlan::GetState`.
/// Mirrors the `FLIGHT_PLAN_STATE_*` values used by `GetState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlightPlanState {
    /// Not concerned.
    NonConcerned,
    /// Notified.
    Notified,
    /// Coordinated.
    Coordinated,
    /// Transfer to me initiated.
    TransferToMeInitiated,
    /// Transfer from me initiated.
    TransferFromMeInitiated,
    /// Assumed (tracked by me).
    Assumed,
    /// Redundant.
    Redundant,
    /// An unrecognized state.
    Other(i32),
}

impl FlightPlanState {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::NonConcerned,
            1 => Self::Notified,
            2 => Self::Coordinated,
            3 => Self::TransferToMeInitiated,
            4 => Self::TransferFromMeInitiated,
            5 => Self::Assumed,
            7 => Self::Redundant,
            other => Self::Other(other),
        }
    }
}

/// A flight plan's simulation state, from `CFlightPlan::GetFPState`. Mirrors the
/// `FLIGHT_PLAN_STATE_NOT_STARTED/SIMULATED/TERMINATED` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationState {
    /// Not started.
    NotStarted,
    /// Simulated by EuroScope (out of range).
    Simulated,
    /// Terminated.
    Terminated,
    /// An unrecognized state.
    Other(i32),
}

impl SimulationState {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::NotStarted,
            1 => Self::Simulated,
            2 => Self::Terminated,
            other => Self::Other(other),
        }
    }
}

/// Which controller-assigned datum changed, from the `data_type` of
/// [`Plugin::on_controller_assigned_data_update`](crate::Plugin::on_controller_assigned_data_update).
/// Mirrors `CTR_DATA_TYPE_*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerDataType {
    Squawk,
    FinalAltitude,
    TemporaryAltitude,
    CommunicationType,
    ScratchPadString,
    GroundState,
    ClearanceFlag,
    DepartureSequence,
    Speed,
    Mach,
    Rate,
    Heading,
    DirectTo,
    /// An unrecognized data type.
    Other(i32),
}

impl ControllerDataType {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Squawk,
            2 => Self::FinalAltitude,
            3 => Self::TemporaryAltitude,
            4 => Self::CommunicationType,
            5 => Self::ScratchPadString,
            6 => Self::GroundState,
            7 => Self::ClearanceFlag,
            8 => Self::DepartureSequence,
            9 => Self::Speed,
            10 => Self::Mach,
            11 => Self::Rate,
            12 => Self::Heading,
            13 => Self::DirectTo,
            other => Self::Other(other),
        }
    }
}

/// A point/altitude coordination state, from the `Get*CoordinationState`
/// getters. Mirrors `COORDINATION_STATE_*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinationState {
    None,
    RequestedByMe,
    RequestedByOther,
    Accepted,
    Refused,
    ManualAccepted,
    /// An unrecognized state.
    Other(i32),
}

impl CoordinationState {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::None,
            2 => Self::RequestedByMe,
            3 => Self::RequestedByOther,
            4 => Self::Accepted,
            5 => Self::Refused,
            6 => Self::ManualAccepted,
            other => Self::Other(other),
        }
    }
}

/// What data is available for a tag item, from the `tag_data` of
/// [`Plugin::on_get_tag_item`](crate::Plugin::on_get_tag_item). Mirrors
/// `TAG_DATA_*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagData {
    /// Only uncorrelated radar data.
    UncorrelatedRadar,
    /// Flight-plan track data.
    FlightPlanTrack,
    /// Correlated radar + flight plan.
    Correlated,
    /// An unrecognized value.
    Other(i32),
}

impl TagData {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::UncorrelatedRadar,
            2 => Self::FlightPlanTrack,
            3 => Self::Correlated,
            other => Self::Other(other),
        }
    }
}

/// A built-in tag colour, set via [`TagItem::set_color`](crate::TagItem::set_color).
///
/// Mirrors `TAG_COLOR_*`. Use [`Rgb`](Self::Rgb)/[`Default`](Self::Default) with
/// [`TagItem::set_rgb`](crate::TagItem::set_rgb) for a custom colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagColor {
    /// Default colour.
    Default,
    /// Use the RGB set via `set_rgb`.
    Rgb,
    NonConcerned,
    Notified,
    Assumed,
    TransferToMeInitiated,
    Redundant,
    Information,
    OngoingRequestFromMe,
    OngoingRequestToMe,
    OngoingRequestAccepted,
    OngoingRequestRefused,
    Emergency,
    /// A raw colour code not covered above.
    Other(i32),
}

impl TagColor {
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Default => 0,
            Self::Rgb => 1,
            Self::NonConcerned => 2,
            Self::Notified => 3,
            Self::Assumed => 4,
            Self::TransferToMeInitiated => 5,
            Self::Redundant => 6,
            Self::Information => 7,
            Self::OngoingRequestFromMe => 8,
            Self::OngoingRequestToMe => 9,
            Self::OngoingRequestAccepted => 10,
            Self::OngoingRequestRefused => 11,
            Self::Emergency => 12,
            Self::Other(v) => v,
        }
    }
}

/// A mouse button, from the screen-object `RadarScreen` callbacks. Mirrors
/// `BUTTON_*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    /// An unrecognized button code.
    Other(i32),
}

impl MouseButton {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Left,
            2 => Self::Middle,
            3 => Self::Right,
            other => Self::Other(other),
        }
    }
}

/// Classification of an extracted-route airway segment. Mirrors `AIRWAY_CLASS_*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AirwayClass {
    Valid,
    DirectionError,
    Unconnected,
    NoDataDirect,
    /// An unrecognized class.
    Other(i32),
}

impl AirwayClass {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Valid,
            1 => Self::DirectionError,
            2 => Self::Unconnected,
            3 => Self::NoDataDirect,
            other => Self::Other(other),
        }
    }
}

/// The drawing phase passed to
/// [`RadarScreen::on_refresh`](crate::RadarScreen::on_refresh). Mirrors
/// `REFRESH_PHASE_*`. `OnRefresh` is called once per phase each repaint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshPhase {
    /// Background bitmap (map content).
    BackBitmap,
    /// Before TAGs are drawn.
    BeforeTags,
    /// After TAGs, before lists.
    AfterTags,
    /// After everything (before the chat area).
    AfterLists,
    /// An unrecognized phase (forward-compatibility).
    Other(i32),
}

impl RefreshPhase {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::BackBitmap,
            1 => Self::BeforeTags,
            2 => Self::AfterTags,
            3 => Self::AfterLists,
            other => Self::Other(other),
        }
    }
}

/// A sector-file element category, used to filter
/// [`Context::sector_file_elements`](crate::Context::sector_file_elements).
/// Mirrors `SECTOR_ELEMENT_*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectorElementType {
    Info,
    Vor,
    Ndb,
    Airport,
    Runway,
    Fix,
    Star,
    Sid,
    LowAirway,
    HighAirway,
    HighArtc,
    Artc,
    LowArtc,
    Geo,
    FreeText,
    Airspace,
    Position,
    SidsStars,
    Radars,
    Regions,
    /// All element types (`SECTOR_ELEMENT_ALL`).
    All,
    /// An unrecognized element type.
    Other(i32),
}

impl SectorElementType {
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Info => 0,
            Self::Vor => 1,
            Self::Ndb => 2,
            Self::Airport => 3,
            Self::Runway => 4,
            Self::Fix => 5,
            Self::Star => 6,
            Self::Sid => 7,
            Self::LowAirway => 8,
            Self::HighAirway => 9,
            Self::HighArtc => 10,
            Self::Artc => 11,
            Self::LowArtc => 12,
            Self::Geo => 13,
            Self::FreeText => 14,
            Self::Airspace => 15,
            Self::Position => 16,
            Self::SidsStars => 17,
            Self::Radars => 18,
            Self::Regions => 19,
            Self::All => -1,
            Self::Other(v) => v,
        }
    }
}

/// Checkbox state of a popup-list element. Mirrors `POPUP_ELEMENT_*`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupCheck {
    /// Show an unchecked checkbox.
    Unchecked,
    /// Show a checked checkbox.
    Checked,
    /// No checkbox at all.
    NoCheckbox,
}

impl PopupCheck {
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Unchecked => 0,
            Self::Checked => 1,
            Self::NoCheckbox => 2,
        }
    }
}

/// A controller's network (VATSIM) rating, from `CController::GetRating`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerRating {
    /// No/unknown rating.
    Unknown,
    /// Observer.
    Observer,
    /// Student 1 (Tower Trainee).
    S1,
    /// Student 2 (Tower Controller).
    S2,
    /// Senior Student (Approach Controller).
    S3,
    /// Enroute Controller.
    C1,
    /// C2 (reserved / unused on VATSIM).
    C2,
    /// Senior Controller.
    C3,
    /// Instructor.
    I1,
    /// I2 (reserved / unused on VATSIM).
    I2,
    /// Senior Instructor.
    I3,
    /// Supervisor.
    Supervisor,
    /// Administrator.
    Administrator,
    /// An unrecognized rating id (forward-compatibility).
    Other(i32),
}

impl ControllerRating {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Observer,
            2 => Self::S1,
            3 => Self::S2,
            4 => Self::S3,
            5 => Self::C1,
            6 => Self::C2,
            7 => Self::C3,
            8 => Self::I1,
            9 => Self::I2,
            10 => Self::I3,
            11 => Self::Supervisor,
            12 => Self::Administrator,
            other => Self::Other(other),
        }
    }

    /// Short label, e.g. `"S3"`. Empty for [`Unknown`](Self::Unknown) and
    /// unrecognized ratings.
    pub fn label(self) -> &'static str {
        match self {
            Self::Unknown | Self::Other(_) => "",
            Self::Observer => "OBS",
            Self::S1 => "S1",
            Self::S2 => "S2",
            Self::S3 => "S3",
            Self::C1 => "C1",
            Self::C2 => "C2",
            Self::C3 => "C3",
            Self::I1 => "I1",
            Self::I2 => "I2",
            Self::I3 => "I3",
            Self::Supervisor => "SUP",
            Self::Administrator => "ADM",
        }
    }
}

/// The kind of position a controller occupies, from `CController::GetFacility`.
///
/// Reflects *how you are connected*, independent of [`ControllerRating`]: an
/// `S3` connected as an observer has facility [`Observer`](Self::Observer).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facility {
    /// Observer — not controlling a position.
    Observer,
    /// Flight Service Station.
    FlightService,
    /// Clearance Delivery.
    Delivery,
    /// Ground.
    Ground,
    /// Tower.
    Tower,
    /// Approach / Departure.
    Approach,
    /// Center / Enroute.
    Center,
    /// An unrecognized facility id (forward-compatibility).
    Other(i32),
}

impl Facility {
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Observer,
            1 => Self::FlightService,
            2 => Self::Delivery,
            3 => Self::Ground,
            4 => Self::Tower,
            5 => Self::Approach,
            6 => Self::Center,
            other => Self::Other(other),
        }
    }

    /// Whether this is an observer position (not controlling).
    pub fn is_observer(self) -> bool {
        matches!(self, Self::Observer)
    }
}

/// The voice/text communication capability of a flight plan.
///
/// A controller can assign it, and a flight plan carries it. Mirrors the single
/// character used by `GetCommunicationType`/`SetCommunicationType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommunicationType {
    /// Unassigned (`'\0'`).
    Unassigned,
    /// Full voice (`'V'`).
    Voice,
    /// Receive voice only, transmit by text (`'R'`).
    ReceiveOnly,
    /// Text only (`'T'`).
    TextOnly,
    /// An unrecognized code (forward-compatibility).
    Other(c_char),
}

impl CommunicationType {
    pub(crate) fn from_raw(value: c_char) -> Self {
        match value.cast_unsigned() {
            0 => Self::Unassigned,
            b'V' => Self::Voice,
            b'R' => Self::ReceiveOnly,
            b'T' => Self::TextOnly,
            other => Self::Other(other.cast_signed()),
        }
    }

    pub(crate) fn to_raw(self) -> c_char {
        let byte: u8 = match self {
            Self::Unassigned => 0,
            Self::Voice => b'V',
            Self::ReceiveOnly => b'R',
            Self::TextOnly => b'T',
            Self::Other(c) => c.cast_unsigned(),
        };
        byte.cast_signed()
    }
}

/// The ICAO wake turbulence category of an aircraft, from
/// `CFlightPlanData::GetAircraftWtc`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeCategory {
    /// Unknown (`'?'` or unset).
    Unknown,
    /// Light (`'L'`).
    Light,
    /// Medium (`'M'`).
    Medium,
    /// Heavy (`'H'`).
    Heavy,
    /// Super heavy (`'J'`).
    Super,
    /// An unrecognized code (forward-compatibility).
    Other(c_char),
}

impl WakeCategory {
    pub(crate) fn from_raw(value: c_char) -> Self {
        match value.cast_unsigned() {
            0 | b'?' => Self::Unknown,
            b'L' => Self::Light,
            b'M' => Self::Medium,
            b'H' => Self::Heavy,
            b'J' => Self::Super,
            other => Self::Other(other.cast_signed()),
        }
    }
}

/// The airframe category of an aircraft, from
/// `CFlightPlanData::GetAircraftType` (the ICAO single-letter type).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftCategory {
    /// Unknown (`'?'` or unset).
    Unknown,
    /// Landplane (`'L'`).
    Landplane,
    /// Seaplane (`'S'`).
    Seaplane,
    /// Amphibian (`'A'`).
    Amphibian,
    /// Helicopter (`'H'`).
    Helicopter,
    /// Gyrocopter (`'G'`).
    Gyrocopter,
    /// Tilt-wing (`'T'`).
    TiltWing,
    /// An unrecognized code (forward-compatibility).
    Other(c_char),
}

impl AircraftCategory {
    pub(crate) fn from_raw(value: c_char) -> Self {
        match value.cast_unsigned() {
            0 | b'?' => Self::Unknown,
            b'L' => Self::Landplane,
            b'S' => Self::Seaplane,
            b'A' => Self::Amphibian,
            b'H' => Self::Helicopter,
            b'G' => Self::Gyrocopter,
            b'T' => Self::TiltWing,
            other => Self::Other(other.cast_signed()),
        }
    }
}

/// The engine type of an aircraft, from `CFlightPlanData::GetEngineType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineType {
    /// Unknown (`'?'` or unset).
    Unknown,
    /// Piston (`'P'`).
    Piston,
    /// Turboprop / turboshaft (`'T'`).
    Turboprop,
    /// Jet (`'J'`).
    Jet,
    /// Electric (`'E'`).
    Electric,
    /// An unrecognized code (forward-compatibility).
    Other(c_char),
}

impl EngineType {
    pub(crate) fn from_raw(value: c_char) -> Self {
        match value.cast_unsigned() {
            0 | b'?' => Self::Unknown,
            b'P' => Self::Piston,
            b'T' => Self::Turboprop,
            b'J' => Self::Jet,
            b'E' => Self::Electric,
            other => Self::Other(other.cast_signed()),
        }
    }
}

/// The navigation/equipment capability of an aircraft, from
/// `CFlightPlanData::GetCapibilities`.
///
/// This is the classic single-letter equipment suffix; the variants spell out
/// the DME / transponder / RNAV combination each letter encodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavCapability {
    /// Unknown (`'?'` or unset).
    Unknown,
    /// `'T'` — no DME, transponder without mode A+C.
    NoDmeTransponderNoModeC,
    /// `'X'` — no DME, no transponder.
    NoDmeNoTransponder,
    /// `'U'` — no DME, transponder with mode A+C.
    NoDmeTransponderModeC,
    /// `'D'` — DME, no transponder.
    DmeNoTransponder,
    /// `'B'` — DME, transponder without mode A+C.
    DmeTransponderNoModeC,
    /// `'A'` — DME, transponder with mode A+C.
    DmeTransponderModeC,
    /// `'M'` — TACAN only, no transponder.
    TacanNoTransponder,
    /// `'N'` — TACAN only, transponder without mode A+C.
    TacanTransponderNoModeC,
    /// `'P'` — TACAN only, transponder with mode A+C.
    TacanTransponderModeC,
    /// `'Y'` — simple RNAV, no transponder.
    SimpleRnavNoTransponder,
    /// `'C'` — simple RNAV, transponder without mode A+C.
    SimpleRnavTransponderNoModeC,
    /// `'I'` — simple RNAV, transponder with mode A+C.
    SimpleRnavTransponderModeC,
    /// `'E'` — advanced RNAV with dual FMS.
    AdvancedRnavDualFms,
    /// `'F'` — advanced RNAV with single FMS.
    AdvancedRnavSingleFms,
    /// `'G'` — advanced RNAV with GPS or GNSS.
    AdvancedRnavGnss,
    /// `'R'` — advanced RNAV with RNP capability.
    AdvancedRnavRnp,
    /// `'W'` — advanced RNAV with RVSM capability.
    AdvancedRnavRvsm,
    /// `'Q'` — advanced RNAV with RNP and RVSM.
    AdvancedRnavRnpRvsm,
    /// An unrecognized code (forward-compatibility).
    Other(c_char),
}

impl NavCapability {
    pub(crate) fn from_raw(value: c_char) -> Self {
        match value.cast_unsigned() {
            0 | b'?' => Self::Unknown,
            b'T' => Self::NoDmeTransponderNoModeC,
            b'X' => Self::NoDmeNoTransponder,
            b'U' => Self::NoDmeTransponderModeC,
            b'D' => Self::DmeNoTransponder,
            b'B' => Self::DmeTransponderNoModeC,
            b'A' => Self::DmeTransponderModeC,
            b'M' => Self::TacanNoTransponder,
            b'N' => Self::TacanTransponderNoModeC,
            b'P' => Self::TacanTransponderModeC,
            b'Y' => Self::SimpleRnavNoTransponder,
            b'C' => Self::SimpleRnavTransponderNoModeC,
            b'I' => Self::SimpleRnavTransponderModeC,
            b'E' => Self::AdvancedRnavDualFms,
            b'F' => Self::AdvancedRnavSingleFms,
            b'G' => Self::AdvancedRnavGnss,
            b'R' => Self::AdvancedRnavRnp,
            b'W' => Self::AdvancedRnavRvsm,
            b'Q' => Self::AdvancedRnavRnpRvsm,
            other => Self::Other(other.cast_signed()),
        }
    }
}
