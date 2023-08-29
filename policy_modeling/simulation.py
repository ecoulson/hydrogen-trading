import math
from datetime import datetime
from electrolyzer import Electrolyzer
from energy_source_portfolio import get_energy_source_portfolios
from emission import calculate_emitted_co2
from hydrogen_production import calculate_hydrogen_produced
from grid import PowerGrid
from emission import EmissionEvent
from hydrogen_production import HydrogenProduction
from transaction import purchase
from power_plant import PowerPlant
from tax_credit import calculate_tax_credit
from tax_credit import TaxCredit45V
from time_range import TimeRange
from transaction import EnergyTransaction


class SimulationState:
    simulation_id: int
    emissions: list[EmissionEvent]
    hydrogen_outputs: list[HydrogenProduction]
    transactions: list[EnergyTransaction]

    def __init__(
        self,
        simulation_id: int,
        emissions: list[EmissionEvent] = [],
        hydrogen_outputs: list[HydrogenProduction] = [],
        transactions: list[EnergyTransaction] = []
    ) -> None:
        self.simulation_id = simulation_id
        self.emissions = emissions
        self.hydrogen_outputs = hydrogen_outputs
        self.transactions = transactions


class SimulationResult:
    id: int
    revenue: float
    cost: float

    def __init__(
        self,
        id: int = 0,
        revenue: float = 0,
        cost: float = 0,
        cost_per_hour: float = 0,
        emissions_per_hour: float = 0,
    ) -> None:
        self.id = id
        self.revenue = revenue
        self.cost = cost


# TODO I think this only works for one time step rn
def simulate(
    time_range: TimeRange,
    electrolyzer: Electrolyzer,
    grid: PowerGrid
) -> SimulationResult:
    simulation_id = 0
    simulation_state = SimulationState(simulation_id)
    hour_in_seconds = 60 * 60
    current_timestamp = datetime.fromtimestamp(time_range.start.timestamp())

    while current_timestamp < time_range.end:
        current_timestamp = datetime.fromtimestamp(
            current_timestamp.timestamp() + hour_in_seconds)

        transactions = make_optimal_transactions(
            simulation_id,
            time_range.start,
            electrolyzer,
            grid.power_plants
        )
        portfolio = get_energy_source_portfolios(
            simulation_id, current_timestamp, transactions)
        emission = calculate_emitted_co2(
            simulation_id, electrolyzer, portfolio)
        hydrogen_output = calculate_hydrogen_produced(
            simulation_id, electrolyzer, portfolio)

        simulation_state.emissions.append(emission)
        simulation_state.hydrogen_outputs.append(hydrogen_output)
        simulation_state.transactions += transactions

    tax_credit = calculate_tax_credit(
        simulation_state.emissions, simulation_state.hydrogen_outputs)
    revenue = calculate_revenue(tax_credit)
    costs = calculate_cost(time_range, electrolyzer,
                           simulation_state.transactions)

    return SimulationResult(
        id=simulation_id,
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
                        power_plant, 0.2, timestamp), power_plants))


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
