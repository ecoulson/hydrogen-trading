use std::str::FromStr;

use super::{errors::Error, simulation_schema::EnergySource, time::Timestamp};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Settlement {
    #[default]
    Final,
    Initial,
}

impl FromStr for Settlement {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "FINAL" => Ok(Settlement::Final),
            "INITIAL" => Ok(Settlement::Initial),
            _ => Err(Error::create_parse_error(value)),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum RepeatedHourFlag {
    #[default]
    Repeated,
    Unrepeated,
}

impl FromStr for RepeatedHourFlag {
    type Err = Error;

    fn from_str(value: &str) -> Result<RepeatedHourFlag, Error> {
        match value {
            "N" => Ok(RepeatedHourFlag::Unrepeated),
            "Y" => Ok(RepeatedHourFlag::Repeated),
            _ => Err(Error::create_parse_error(value)),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum SettlementPointLocation {
    #[default]
    NorthHub,
    SouthHub,
    HustonHub,
    WestHub,
    PanhandleHub,
    HubBusAverage,
    HubAverage,
    AustinEnergyLoadingZone,
    CPSEnergyLoadingZone,
    HustonLoadingZone,
    LowerColoradoRiverAuthorityLoadingZone,
    RayburnElectricCooperativeLoadingZone,
    NorthLoadingZone,
    SouthLoadingZone,
    WestLoadingZone,
}

impl FromStr for SettlementPointLocation {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "HB_BUSAVG" => Ok(SettlementPointLocation::HubBusAverage),
            "HB_HOUSTON" => Ok(SettlementPointLocation::HustonHub),
            "HB_HUBAVG" => Ok(SettlementPointLocation::HubAverage),
            "HB_NORTH" => Ok(SettlementPointLocation::NorthHub),
            "HB_PAN" => Ok(SettlementPointLocation::PanhandleHub),
            "HB_SOUTH" => Ok(SettlementPointLocation::SouthHub),
            "HB_WEST" => Ok(SettlementPointLocation::WestHub),
            "LZ_AEN" => Ok(SettlementPointLocation::AustinEnergyLoadingZone),
            "LZ_CPS" => Ok(SettlementPointLocation::CPSEnergyLoadingZone),
            "LZ_HOUSTON" => Ok(SettlementPointLocation::HustonLoadingZone),
            "LZ_LCRA" => Ok(SettlementPointLocation::LowerColoradoRiverAuthorityLoadingZone),
            "LZ_NORTH" => Ok(SettlementPointLocation::NorthLoadingZone),
            "LZ_RAYBN" => Ok(SettlementPointLocation::RayburnElectricCooperativeLoadingZone),
            "LZ_SOUTH" => Ok(SettlementPointLocation::SouthLoadingZone),
            "LZ_WEST" => Ok(SettlementPointLocation::WestLoadingZone),
            _ => Err(Error::create_parse_error(value)),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum SettlementPointType {
    #[default]
    AH,
    SH,
    HU,
    LZ,
    LZEW,
}

impl FromStr for SettlementPointType {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "AH" => Ok(SettlementPointType::AH),
            "SH" => Ok(SettlementPointType::SH),
            "HU" => Ok(SettlementPointType::HU),
            "LZ" => Ok(SettlementPointType::LZ),
            "LZEW" => Ok(SettlementPointType::LZEW),
            _ => Err(Error::create_parse_error(value)),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ErcotFuelMix {
    pub date: Timestamp,
    pub fuel_source: EnergySource,
    pub settlement: Settlement,
    pub electricity_produced: f64,
}

#[derive(Default, Debug, PartialEq)]
pub struct ErcotRTMPrice {
    pub delivery_timestamp: Timestamp,
    pub repeated_hour_flag: RepeatedHourFlag,
    pub settlement_point_location: SettlementPointLocation,
    pub settlement_point_type: SettlementPointType,
    pub settlement_point_price: f64,
}
