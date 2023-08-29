from datetime import datetime
from power_plant import EnergySource
from transaction import EnergyTransaction


class EnergySourcePortfolio:
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
    transactions: list[EnergyTransaction],
) -> list[EnergySourcePortfolio]:
    portfolios_over_time: dict[datetime, EnergySourcePortfolio] = {}
    portfolios: list[EnergySourcePortfolio] = []

    for transaction in transactions:
        transaction_portfolio = portfolio_from_transaction(transaction)
        if transaction.timestamp not in portfolios_over_time:
            portfolios_over_time[transaction.timestamp] = transaction_portfolio
        else:
            portfolios_over_time[transaction.timestamp] = merge_portfolio(
                transaction_portfolio,
                portfolios_over_time[transaction.timestamp]
            )

    for timestamp in portfolios_over_time.keys():
        portfolios.append(portfolios_over_time[timestamp])

    return portfolios


def portfolio_from_transaction(
    transaction: EnergyTransaction
) -> EnergySourcePortfolio:
    portfolio = EnergySourcePortfolio(
        timestamp=transaction.timestamp,
        total_electricity_mwh=transaction.amount_mwh
    )

    match transaction.energy_source:
        case EnergySource.ENERGY_SOURCE_NATURAL_GAS:
            portfolio.natural_gas_mmbtu += transaction.fuel_consumed_mmbtu

    return portfolio


def merge_portfolio(
    portfolio_a: EnergySourcePortfolio,
    portfolio_b: EnergySourcePortfolio
) -> EnergySourcePortfolio:
    if portfolio_a.timestamp != portfolio_b.timestamp:
        raise Exception("Timestamps must match")

    merged_portfolio = EnergySourcePortfolio()
    merged_portfolio.total_electricity_mwh = \
        portfolio_b.total_electricity_mwh + portfolio_a.total_electricity_mwh
    merged_portfolio.natural_gas_mmbtu = portfolio_b.natural_gas_mmbtu + \
        portfolio_a.natural_gas_mmbtu

    return merged_portfolio
