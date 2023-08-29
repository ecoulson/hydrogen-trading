from datetime import datetime
from electrolyzer import Electrolyzer
from energy_source_portfolio import EnergySourcePortfolio


class HydrogenProduction:
    plant_id: int
    production_timestamp: datetime
    kg_hydrogen: float

    def __init__(
        self,
        plant_id: int = 0,
        production_timestamp: datetime = datetime.now(),
        kg_hydrogen: float = 0
    ) -> None:
        self.plant_id = plant_id
        self.production_timestamp = production_timestamp
        self.kg_hydrogen = kg_hydrogen


def calculate_hydrogen_produced(
    electrolyzer: Electrolyzer,
    portfolios: list[EnergySourcePortfolio],
) -> list[HydrogenProduction]:
    productions: list[HydrogenProduction] = []

    for portfolio in portfolios:
        production = HydrogenProduction()
        production.production_timestamp = portfolio.timestamp
        production.kg_hydrogen = portfolio.total_electricity_mwh * \
            electrolyzer.production_method.calculate_production(
                portfolio.total_electricity_mwh)
        productions.append(production)

    return productions
