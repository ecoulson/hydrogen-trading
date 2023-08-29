from datetime import datetime
from power_plant import EnergySource
from transaction import EnergyTransaction


class EnergySourcePortfolio:
    simulation_id: int
    timestamp: datetime
    total_electricity_mwh: float
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

    def __init__(
        self,
        simulation_id: int = 0,
        timestamp: datetime = datetime.now(),
        total_electricity_mwh: float = 0,
        petroleum_mmbtu: float = 0,
        hydrocarbons_mmbtu: float = 0,
        natural_gas_mmbtu: float = 0,
        coal_mmbtu: float = 0,
        nuclear_mmbtu: float = 0,
        solar_mmbtu: float = 0,
        geothermal_mmbtu: float = 0,
        wind_mmbtu: float = 0,
        biomass_mmbtu: float = 0,
        hydropower_mmbtu: float = 0
    ):
        self.simulation_id = simulation_id
        self.timestamp = timestamp
        self.total_electricity_mwh = total_electricity_mwh
        self.petroleum_mmbtu = petroleum_mmbtu
        self.hydrocarbons_mmbtu = hydrocarbons_mmbtu
        self.natural_gas_mmbtu = natural_gas_mmbtu
        self.coal_mmbtu = coal_mmbtu
        self.nuclear_mmbtu = nuclear_mmbtu
        self.solar_mmbtu = solar_mmbtu
        self.geothermal_mmbtu = geothermal_mmbtu
        self.wind_mmbtu = wind_mmbtu
        self.biomass_mmbtu = biomass_mmbtu
        self.hydropower_mmbtu = hydropower_mmbtu


# Check that power purchased doesn't exeed the amount generated at a given time
def get_energy_source_portfolios(
    simulation_id: int,
    timestamp: datetime,
    transactions: list[EnergyTransaction],
) -> EnergySourcePortfolio:
    portfolio = EnergySourcePortfolio(simulation_id, timestamp)

    for transaction in transactions:
        portfolio = add_transaction(portfolio, transaction)

    return portfolio


def add_transaction(
    portfolio: EnergySourcePortfolio,
    transaction: EnergyTransaction
) -> EnergySourcePortfolio:
    portfolio.total_electricity_mwh += transaction.amount_mwh
    portfolio.natural_gas_mmbtu += transaction.fuel_consumed_mmbtu

    return portfolio
