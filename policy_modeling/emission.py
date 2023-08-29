from datetime import datetime
from electrolyzer import Electrolyzer
from energy_source_portfolio import EnergySourcePortfolio

NATURAL_GAS_MMBTU_TO_CO2 = 53.0703


class EmissionEvent:
    simulation_id: int
    electrolyzer_id: int
    emission_timestamp: datetime
    amount_emitted_kg: float

    def __init__(
        self,
        simulation_id: int = 0,
        electrolyzer_id: int = 0,
        emission_timestamp: datetime = datetime.now(),
        amount_emitted_kg: float = 0
    ) -> None:
        self.simulation_id = simulation_id
        self.electrolyzer_id = electrolyzer_id
        self.emission_timestamp = emission_timestamp
        self.amount_emitted_kg = amount_emitted_kg


def calculate_emitted_co2(
    simulation_id: int,
    electrolyzer: Electrolyzer,
    portfolio: EnergySourcePortfolio,
) -> EmissionEvent:
    amount_emitted_kg = 0
    amount_emitted_kg += portfolio.natural_gas_mmbtu * NATURAL_GAS_MMBTU_TO_CO2
    return EmissionEvent(
        simulation_id=simulation_id,
        electrolyzer_id=electrolyzer.id,
        emission_timestamp=portfolio.timestamp,
        amount_emitted_kg=amount_emitted_kg
    )
