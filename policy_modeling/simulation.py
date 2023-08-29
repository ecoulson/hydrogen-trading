import math
from datetime import datetime
from electrolyzer import Electrolyzer
from energy_source_portfolio import get_energy_source_portfolios
from emission import calculate_emitted_co2
from hydrogen_production import calculate_hydrogen_produced
from transaction import purchase
from power_plant import PowerPlant
from tax_credit import calculate_tax_credit
from tax_credit import TaxCredit45V
from time_range import TimeRange
from transaction import EnergyTransaction


class SimulationResult:
    id: int
    revenue: float
    cost: float

    def __init__(
        self,
        id: int = 0,
        revenue: float = 0,
        cost: float = 0
    ) -> None:
        self.id = id
        self.revenue = revenue
        self.cost = cost


def simulate(
    time_range: TimeRange,
    electrolyzer: Electrolyzer,
    # this will probably become some obj representing the grid
    power_plants: list[PowerPlant]
) -> SimulationResult:
    simulation_id = 0
    transactions = make_optimal_transactions(
        simulation_id,
        time_range.start,
        electrolyzer,
        power_plants
    )
    portfolios = get_energy_source_portfolios(transactions)
    emissions = calculate_emitted_co2(electrolyzer, portfolios)
    hydrogen_productions = calculate_hydrogen_produced(
        electrolyzer, portfolios)

    print("-==- ELECTROLYZER -==-")
    print(electrolyzer.__dict__)
    print("----------------------")

    print("-==- TRANSACTIONS -==-")
    for transaction in transactions:
        print(transaction.__dict__)
    print("----------------------")

    print("-==-  PORTFOLIOS  -==-")
    for portfolio in portfolios:
        print(portfolio.__dict__)
    print("----------------------")

    print("-==-  EMISSIONS   -==-")
    for emission in emissions:
        print(emission.__dict__)
    print("----------------------")

    print("-==-   HYDROGEN   -==-")
    for hydrogen_production in hydrogen_productions:
        print(hydrogen_production.__dict__)
    print("----------------------")

    tax_credit = calculate_tax_credit(emissions, hydrogen_productions)
    print("-==- TAX CREDIT  -==-")
    print(tax_credit.__dict__)
    print("----------------------")

    revenue = calculate_revenue(tax_credit)
    costs = calculate_cost(time_range, electrolyzer, transactions)

    return SimulationResult(
        id=0,
        revenue=revenue,
        cost=costs
    )


def make_optimal_transactions(
    simulation_id: int,
    timestamp: datetime,
    electrolyzer: Electrolyzer,
    power_plants: list[PowerPlant]
) -> list[EnergyTransaction]:
    return list(map(lambda power_plant:
                    purchase(
                        simulation_id, electrolyzer,
                        power_plant, 20000, timestamp), power_plants))


def calculate_revenue(
    tax_credit: TaxCredit45V,
) -> float:
    return tax_credit.total_usd


def calculate_cost(
    simulation_time_range: TimeRange,
    electrolyzer: Electrolyzer,
    transactions: list[EnergyTransaction],
) -> float:
    total_energy_consumed_mwh = sum(
        map(lambda transaction: transaction.amount_mwh, transactions))
    costs = electrolyzer.capital_expenditure

    # calc opex
    difference = simulation_time_range.end - simulation_time_range.start
    hours = difference.total_seconds() / (60 * 60)
    costs += electrolyzer.operational_expenditure * hours \
        * total_energy_consumed_mwh

    # calc repair costs
    repair_cost = electrolyzer.replacement_cost \
        * electrolyzer.capital_expenditure
    years_per_repair = electrolyzer.replacement_threshold / \
        electrolyzer.degradation_rate
    num_repairs = math.floor(difference.total_seconds() /
                             (60 * 60 * 24 * 365) / years_per_repair)
    costs += repair_cost * num_repairs

    for transcation in transactions:
        costs += transcation.price_usd

    return costs
