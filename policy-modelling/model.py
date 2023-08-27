from datetime import datetime
from enum import Enum

TAX_CREDIT_45V_MAX_VALUE_USD = 3
NATURAL_GAS_MMBTU_TO_CO2 = 53.0703


class EnergySourcePortfolio:
    timestamp: datetime
    petroleum_mmbtu: float
    hydrocarbons_mmbtu: float
    natural_gas_mmbtu: float
    coal_mmbtu: float
    nuclear_mmbtu: float
    solar_mmbtu: float
    geothermal_mmbtu: float
    wind_mmbtu: float
    biomass_mmbtu: float
    hydropower_mmbtu: float


class EnergySource(Enum):
    ENERGY_SOURCE_PETROLEUM = 1
    ENERGY_SOURCE_HYDROCARBONS = 2
    ENERGY_SOURCE_NATURAL_GAS = 3
    ENERGY_SOURCE_COAL = 4
    ENERGY_SOURCE_NUCLEAR = 5
    ENERGY_SOURCE_SOLAR = 6
    ENERGY_SOURCE_GEOTHERMAL = 7
    ENERGY_SOURCE_WIND = 8
    ENERGY_SOURCE_BIOMASS = 9
    ENERGY_SOURCE_HYDROPOWER = 10


class ConsumptionUnit(Enum):
    CONSUMPTION_UNIT_MCF = 1
    CONSUMPTION_UNIT_MMBTU = 2


class ConsumptionMetric:
    plant_id: int
    time_consumed: datetime
    unit: ConsumptionUnit
    amount: float


class GenerationMetric:
    plant_id: int
    time_generated: datetime
    amount_mwh: float


class PowerPlant:
    plant_id: int
    energy_source: EnergySource


class EmissionEvent:
    emission_timestamp: datetime
    amount_emitted_kg: float


class HydrogenProduction:
    production_timestamp: datetime
    kg_hydrogen: float


class ProductionRate:
    # takes in the input kw to determine the efficiency rate
    def calculate_production_rate(self, input_kwh: float) -> float:
        return 0


class ConstantProductionRate(ProductionRate):
    conversion_rate: float

    # takes in the input kw to determine the efficiency rate
    def calculate_production_rate(self, input_mwh: float) -> float:
        return 1 / self.conversion_rate


class Electrolyzer:
    replacement_threshold: float
    degradation_rate: float
    capacity_kw: float
    production_rate: ProductionRate


class TaxCredit45V:
    amount_usd: float


def calculate_emitted_co2(portfolio: EnergySourcePortfolio, timestamp: datetime) -> EmissionEvent:
    emission = EmissionEvent()
    emission.emission_timestamp = timestamp
    emission.amount_emitted_kg += calculate_emitted_co2_from_natural_gas(
        portfolio.natural_gas_mmbtu)
    return emission


def calculate_emitted_co2_from_natural_gas(natural_gas_mmbtu: float) -> float:
    return natural_gas_mmbtu * NATURAL_GAS_MMBTU_TO_CO2


def calculate_hydrogen_produced(electrolyzer: Electrolyzer, generations: list[GenerationMetric], timestamp: datetime) -> HydrogenProduction:
    total_generated_electricity_mwh = sum(
        map(lambda generation: generation.amount_mwh, generations))

    production = HydrogenProduction()
    production.production_timestamp = timestamp
    production.kg_hydrogen = total_generated_electricity_mwh * \
        electrolyzer.production_rate.calculate_production_rate(
            total_generated_electricity_mwh)
    return production


def calculate_tax_credit(emissions: list[EmissionEvent], hydrogen_produced: list[HydrogenProduction]) -> TaxCredit45V:
    credit = TaxCredit45V()
    total_co2_emitted = sum(
        map(lambda emission: emission.amount_emitted_kg, emissions))
    total_h2_produced = sum(
        map(lambda production: production.kg_hydrogen, hydrogen_produced))
    co2_per_h2 = total_co2_emitted / total_h2_produced

    if 2.5 <= co2_per_h2 and co2_per_h2 < 4:
        credit.amount_usd = TAX_CREDIT_45V_MAX_VALUE_USD * 0.2
    elif 1.5 <= co2_per_h2 and co2_per_h2 < 2.5:
        credit.amount_usd = TAX_CREDIT_45V_MAX_VALUE_USD * 0.2
    elif 0.45 <= co2_per_h2 and co2_per_h2 < 1.5:
        credit.amount_usd = TAX_CREDIT_45V_MAX_VALUE_USD * 0.334
    elif co2_per_h2 < 0.45:
        credit.amount_usd = TAX_CREDIT_45V_MAX_VALUE_USD * 0.334

    return credit
