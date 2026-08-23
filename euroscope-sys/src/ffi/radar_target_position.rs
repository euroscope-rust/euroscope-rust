//! `CRadarTargetPositionData` FFI declarations.
//!
//! The handle is an owned `CRadarTargetPositionData*` (created by a position
//! selector, freed with `es_rtposdata_free`); each accessor reads it directly.

use std::ffi::{c_char, c_int};

use crate::EsHandle;

unsafe extern "C" {
    /// Free an owned position snapshot.
    pub fn es_rtposdata_free(h: EsHandle);

    /// `CRadarTargetPositionData::IsValid` — whether the position reference is valid.
    pub fn es_rtpos_is_valid(h: EsHandle) -> bool;

    /// `CRadarTargetPositionData::IsFPTrackPosition` — whether the position is a
    /// reference to the flight-plan track.
    pub fn es_rtpos_is_fp_track_position(h: EsHandle) -> bool;

    /// `CRadarTargetPositionData::GetReceivedTime` — seconds since the position
    /// data was received.
    pub fn es_rtpos_received_time(h: EsHandle) -> c_int;

    /// `CRadarTargetPositionData::GetPosition` — the lat/lon of the plane, via
    /// out-params.
    pub fn es_rtpos_position(h: EsHandle, lat: *mut f64, lon: *mut f64);

    /// `CRadarTargetPositionData::GetSquawk` — squawk sent by the pilot.
    /// Borrowed, NUL-terminated, ANSI.
    pub fn es_rtpos_squawk(h: EsHandle) -> *const c_char;

    /// `CRadarTargetPositionData::GetTransponderC` — whether the transponder is
    /// in C mode.
    pub fn es_rtpos_transponder_c(h: EsHandle) -> bool;

    /// `CRadarTargetPositionData::GetTransponderI` — whether the transponder is
    /// in IDENT mode.
    pub fn es_rtpos_transponder_i(h: EsHandle) -> bool;

    /// `CRadarTargetPositionData::GetPressureAltitude` — true altitude in feet.
    pub fn es_rtpos_pressure_altitude(h: EsHandle) -> c_int;

    /// `CRadarTargetPositionData::GetFlightLevel` — altitude on standard
    /// pressure, in feet.
    pub fn es_rtpos_flight_level(h: EsHandle) -> c_int;

    /// `CRadarTargetPositionData::GetReportedGS` — ground speed reported by the
    /// pilot client.
    pub fn es_rtpos_reported_gs(h: EsHandle) -> c_int;

    /// `CRadarTargetPositionData::GetReportedHeading` — heading reported by the
    /// pilot client.
    pub fn es_rtpos_reported_heading(h: EsHandle) -> c_int;

    /// `CRadarTargetPositionData::GetReportedHeadingTrueNorth` — reported
    /// heading referenced to true north.
    pub fn es_rtpos_reported_heading_true_north(h: EsHandle) -> c_int;

    /// `CRadarTargetPositionData::GetReportedPitch` — pitch reported by the
    /// pilot client (-180..+180, negative is above horizon).
    pub fn es_rtpos_reported_pitch(h: EsHandle) -> c_int;

    /// `CRadarTargetPositionData::GetReportedBank` — bank reported by the pilot
    /// client (-180..+180, negative is right bank).
    pub fn es_rtpos_reported_bank(h: EsHandle) -> c_int;

    /// `CRadarTargetPositionData::GetRadarFlags` — OR of the `RADAR_POSITION_*`
    /// flags for this position.
    pub fn es_rtpos_radar_flags(h: EsHandle) -> c_int;
}
