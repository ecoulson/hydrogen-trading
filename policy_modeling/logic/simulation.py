from datetime import datetime
from logic.electrolyzer import Electrolyzer
from logic.energy_source_portfolio import get_energy_source_portfolios
from logic.emission import calculate_emitted_co2, EmissionEvent
from logic.hydrogen_production import calculate_hydrogen_produced
from logic.grid import PowerGrid
from logic.hydrogen_production import HydrogenProduction
from logic.transaction import purchase
from logic.power_plant import PowerPlant
from logic.tax_credit import calculate_tax_credit, TaxCredit45V
from logic.time_range import TimeRange
from logic.transaction import EnergyTransaction

HOUR_IN_SECONDS = 60 * 60
YEARS_IN_SECONDS = 60 * 60 * 24 * 365


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
    tax_credit: TaxCredit45V
    emissions: list[EmissionEvent]
    hydrogen_produced: list[HydrogenProduction]

    def __init__(
        self,
        id: int = 0,
        tax_credit: TaxCredit45V = TaxCredit45V(),
        emissions: list[EmissionEvent] = [],
        hydrogen_produced: list[HydrogenProduction] = []
    ) -> None:
        self.id = id
        self.tax_credit = tax_credit
        self.emissions = emissions
        self.hydrogen_produced = hydrogen_produced


def simulate(
    time_range: TimeRange,
    electrolyzer: Electrolyzer,
    grid: PowerGrid
) -> SimulationResult:
    simulation_id = 0
    simulation_state = SimulationState(simulation_id)
    current_timestamp = datetime.fromtimestamp(time_range.start.timestamp())

    while current_timestamp < time_range.end:
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

        current_timestamp = datetime.fromtimestamp(
            current_timestamp.timestamp() + HOUR_IN_SECONDS)

    tax_credit = calculate_tax_credit(
        simulation_state.emissions, simulation_state.hydrogen_outputs)

    return SimulationResult(
        id=simulation_id,
        tax_credit=tax_credit,
        hydrogen_produced=simulation_state.hydrogen_outputs,
        emissions=simulation_state.emissions
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
                        power_plant, 1, timestamp), power_plants))
