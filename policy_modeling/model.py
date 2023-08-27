from datetime import datetime
from enum import Enum

TAX_CREDIT_45V_MAX_VALUE_USD = 3
NATURAL_GAS_MMBTU_TO_CO2 = 53.0703
MCF_TO_MMBTU = 1.036


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
    amount_mmbtu: float

    def __init__(
        self,
        plant_id: int,
        time_consumed: datetime,
        amount_mmbtu: float
    ):
        self.plant_id = plant_id
        self.time_consumed = time_consumed
        self.amount_mmbtu = amount_mmbtu


class GenerationMetric:
    plant_id: int
    time_generated: datetime
    amount_mwh: float

    def __init__(
        self,
        plant_id: int,
        time_generated: datetime,
        amount_mwh: float
    ):
        self.plant_id = plant_id
        self.time_generated = time_generated
        self.amount_mwh = amount_mwh


class PowerPlant:
    plant_id: int
    energy_source: EnergySource
    consumption: list[ConsumptionMetric]
    generation: list[GenerationMetric]

    def __init__(
        self,
        plant_id: int,
        energy_source: EnergySource,
        consumption: list[ConsumptionMetric] = list(),
        generation: list[GenerationMetric] = list()
    ):
        self.plant_id = plant_id
        self.energy_source = energy_source
        self.consumption = consumption
        self.generation = generation


def convert_to_mmbtu(amount: float, unit: ConsumptionUnit):
    match unit:
        case ConsumptionUnit.CONSUMPTION_UNIT_MMBTU:
            return amount
        case ConsumptionUnit.CONSUMPTION_UNIT_MCF:
            # Can use data from EIA to get exact ratio
            return amount * MCF_TO_MMBTU


# TODO: Check for unique timestamp
def generate(
    power_plant: PowerPlant,
    amount_mwh: float,
    timestamp: datetime
) -> None:
    generation = GenerationMetric(
        plant_id=power_plant.plant_id,
        amount_mwh=amount_mwh,
        time_generated=timestamp
    )
    power_plant.generation.append(generation)


# TODO: Check for unique timestamp
def consume(
    power_plant: PowerPlant,
    amount: float,
    unit: ConsumptionUnit,
    timestamp: datetime
) -> None:
    amount_mmbtu = convert_to_mmbtu(amount, unit)

    consumption = ConsumptionMetric(
        plant_id=power_plant.plant_id, amount_mmbtu=amount_mmbtu,
        time_consumed=timestamp)
    power_plant.consumption.append(consumption)


class EmissionEvent:
    plant_id: int
    emission_timestamp: datetime
    amount_emitted_kg: float


class HydrogenProduction:
    plant_id: int
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
    capital_expenditure: float
    operational_expenditure: float
    replacement_cost: float


class TaxCredit45V:
    amount_usd: float


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

    def __init__(self, timestamp: datetime):
        self.timestamp = timestamp
        self.petroleum_mmbtu = 0
        self.hydrocarbons_mmbtu = 0
        self.natural_gas_mmbtu = 0
        self.coal_mmbtu = 0
        self.nuclear_mmbtu = 0
        self.solar_mmbtu = 0
        self.geothermal_mmbtu = 0
        self.wind_mmbtu = 0
        self.biomass_mmbtu = 0
        self.hydropower_mmbtu = 0


def get_energy_source_portfolios(
    power_plants: list[PowerPlant],
    start_time: datetime
) -> list[EnergySourcePortfolio]:
    portfolios = []
    current_indexes: dict[int, int] = {}
    last_time = start_time

    for power_plant in power_plants:
        for (i, consumption) in enumerate(power_plant.consumption):
            last_time = max(start_time, consumption.time_consumed)
            if consumption.time_consumed.year == start_time.year \
                    and consumption.time_consumed.month == start_time.month:
                current_indexes[power_plant.plant_id] = i

    current_timestamp = start_time
    while current_timestamp <= last_time:
        portfolio = EnergySourcePortfolio(timestamp=current_timestamp)
        for power_plant in power_plants:
            consumption_index = current_indexes[power_plant.plant_id]
            consumption = power_plant.consumption[consumption_index]
            portfolio.natural_gas_mmbtu += consumption.amount_mmbtu
            current_indexes[power_plant.plant_id] += 1
        portfolios.append(portfolio)
        current_timestamp = datetime(
            year=current_timestamp.year,
            month=current_timestamp.month + 1,
            day=current_timestamp.day
        )

    return portfolios


def calculate_emitted_co2(
    portfolio: EnergySourcePortfolio,
    timestamp: datetime
) -> EmissionEvent:
    emission = EmissionEvent()
    emission.emission_timestamp = timestamp
    emission.amount_emitted_kg += calculate_emitted_co2_from_natural_gas(
        portfolio.natural_gas_mmbtu)
    return emission


def calculate_emitted_co2_from_natural_gas(natural_gas_mmbtu: float) -> float:
    return natural_gas_mmbtu * NATURAL_GAS_MMBTU_TO_CO2


def calculate_hydrogen_produced(
    electrolyzer: Electrolyzer,
    generations: list[GenerationMetric], timestamp: datetime
) -> HydrogenProduction:
    total_generated_electricity_mwh = sum(
        map(lambda generation: generation.amount_mwh, generations))

    production = HydrogenProduction()
    production.production_timestamp = timestamp
    production.kg_hydrogen = total_generated_electricity_mwh * \
        electrolyzer.production_rate.calculate_production_rate(
            total_generated_electricity_mwh)

    return production


def calculate_tax_credit(
    emissions: list[EmissionEvent],
    hydrogen_produced: list[HydrogenProduction]
) -> TaxCredit45V:
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
