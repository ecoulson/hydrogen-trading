from datetime import datetime
from enum import Enum

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
        plant_id: int = 0,
        time_consumed: datetime = datetime.now(),
        amount_mmbtu: float = 0
    ):
        self.plant_id = plant_id
        self.time_consumed = time_consumed
        self.amount_mmbtu = amount_mmbtu


class GenerationMetric:
    plant_id: int
    time_generated: datetime
    amount_mwh: float
    sale_price_usd_per_mwh: float

    def __init__(
        self,
        plant_id: int = 0,
        time_generated: datetime = datetime.now(),
        amount_mwh: float = 0,
        sale_price_usd_per_mwh: float = 0
    ):
        self.plant_id = plant_id
        self.time_generated = time_generated
        self.amount_mwh = amount_mwh
        self.sale_price_usd_per_mwh = sale_price_usd_per_mwh


class PowerPlant:
    plant_id: int
    energy_source: EnergySource
    heat_rate: float  # conversion of mmbtu to mwh
    consumption: list[ConsumptionMetric]
    generation: list[GenerationMetric]

    def __init__(
        self,
        plant_id: int = 0,
        heat_rate: float = 0,
        energy_source: EnergySource = EnergySource.ENERGY_SOURCE_COAL,
        consumption: list[ConsumptionMetric] = list(),
        generation: list[GenerationMetric] = list()
    ):
        self.plant_id = plant_id
        self.heat_rate = heat_rate
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
    timestamp: datetime,
    sale_price: float
) -> None:
    power_plant.generation.append(GenerationMetric(
        plant_id=power_plant.plant_id,
        amount_mwh=amount_mwh,
        time_generated=timestamp,
        sale_price_usd_per_mwh=sale_price
    ))


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
