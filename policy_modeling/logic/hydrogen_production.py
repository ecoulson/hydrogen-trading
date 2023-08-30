from datetime import datetime
from logic.electrolyzer import Electrolyzer
from logic.energy_source_portfolio import EnergySourcePortfolio


class HydrogenProduction:
    simulation_id: int
    electrolyzer_id: int
    production_timestamp: datetime
    kg_hydrogen: float

    def __init__(
        self,
        simulation_id: int = 0,
        electrolyzer_id: int = 0,
        production_timestamp: datetime = datetime.now(),
        kg_hydrogen: float = 0
    ) -> None:
        self.simulation_id = simulation_id
        self.electrolyzer_id = electrolyzer_id
        self.production_timestamp = production_timestamp
        self.kg_hydrogen = kg_hydrogen


def calculate_hydrogen_produced(
    simulation_id: int,
    electrolyzer: Electrolyzer,
    portfolio: EnergySourcePortfolio,
) -> HydrogenProduction:
    production = HydrogenProduction(simulation_id, electrolyzer.id)
    production.production_timestamp = portfolio.timestamp
    production.kg_hydrogen = min(
        portfolio.total_electricity_mwh, electrolyzer.capacity_mw) * \
        electrolyzer.production_method.calculate_production(
            portfolio.total_electricity_mwh)
    return production
