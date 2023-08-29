from datetime import datetime
from electrolyzer import Electrolyzer
from power_plant import EnergySource, PowerPlant


class EnergyTransaction:
    simulation_id: int
    electrolyzer_id: int
    plant_id: int
    timestamp: datetime
    amount_mwh: float
    fuel_consumed_mmbtu: float
    price_usd: float
    energy_source: EnergySource

    def __init__(
        self,
        simulation_id: int = 0,
        electrolyzer_id: int = 0,
        plant_id: int = 0,
        timestamp: datetime = datetime.now(),
        amount_mwh: float = 0,
        fuel_consumed_mmbtu: float = 0,
        price_usd: float = 0,
        energy_source: EnergySource = EnergySource.ENERGY_SOURCE_COAL
    ) -> None:
        self.simulation_id = simulation_id
        self.electrolyzer_id = electrolyzer_id
        self.plant_id = plant_id
        self.timestamp = timestamp
        self.amount_mwh = amount_mwh
        self.price_usd = price_usd
        self.energy_source = energy_source
        self.fuel_consumed_mmbtu = fuel_consumed_mmbtu


def purchase(
    simulation_id: int,
    electrolyzer: Electrolyzer,
    power_plant: PowerPlant,
    amount_mwh: float,
    timestamp: datetime,
) -> EnergyTransaction:
    for generation in power_plant.generation:
        if generation.time_generated.year == timestamp.year and \
                generation.time_generated.month == timestamp.month:
            return EnergyTransaction(
                simulation_id=simulation_id,
                electrolyzer_id=electrolyzer.id,
                plant_id=power_plant.plant_id,
                timestamp=timestamp,
                amount_mwh=amount_mwh,
                fuel_consumed_mmbtu=amount_mwh * power_plant.heat_rate,
                price_usd=generation.sale_price_usd_per_mwh * amount_mwh,
                energy_source=power_plant.energy_source
            )
    raise Exception("No generation from powerplant at the current timestamp")
