from datetime import datetime
from enum import Enum

TAX_CREDIT_45V_MAX_VALUE_USD = 3
NATURAL_GAS_MMBTU_TO_CO2 = 53.0703
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


class PPAAgreement:
    plant_id: int
    electrolyzer_id: int


class PolicyType(Enum):
    POLICY_TYPE_PPA_AGREEMENT = 1


class Policy:
    policy_type: PolicyType
    ppa_agreement: PPAAgreement


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
    consumption: list[ConsumptionMetric]
    generation: list[GenerationMetric]

    def __init__(
        self,
        plant_id: int = 0,
        energy_source: EnergySource = EnergySource.ENERGY_SOURCE_COAL,
        consumption: list[ConsumptionMetric] = list(),
        generation: list[GenerationMetric] = list()
    ):
        self.plant_id = plant_id
        self.energy_source = energy_source
        self.consumption = consumption
        self.generation = generation


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


class EmissionEvent:
    electrolyzer_id: int
    emission_timestamp: datetime
    amount_emitted_kg: float

    def __init__(
        self,
        electrolyzer_id: int = 0,
        emission_timestamp: datetime = datetime.now(),
        amount_emitted_kg: float = 0
    ) -> None:
        self.electrolyzer_id = electrolyzer_id
        self.emission_timestamp = emission_timestamp
        self.amount_emitted_kg = amount_emitted_kg


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


class Production:
    # takes in the input kw to determine the efficiency rate
    def calculate_production(self, _: float) -> float:
        raise Exception("Use impl of production")


class ConstantProduction(Production):
    conversion_rate: float  # kWh / kg

    def calculate_production(self, input_kwh: float) -> float:
        return input_kwh / self.conversion_rate

    def __init__(self, conversion_rate: float = 0) -> None:
        super().__init__()
        self.conversion_rate = conversion_rate


class Electrolyzer:
    id: int
    replacement_threshold: float
    degradation_rate: float
    capacity_kw: float
    production_method: Production
    capital_expenditure: float
    operational_expenditure: float
    replacement_cost: float

    def __init__(
        self,
        id: int = 0,
        replacement_threshold: float = 0,
        degredation_rate: float = 0,
        capacity_kw: float = 0,
        production_method: Production = Production(),
        capital_expenditure: float = 0,
        operation_expenditure: float = 0,
        replacement_cost: float = 0
    ) -> None:
        self.id = id
        self.replacement_threshold = replacement_threshold
        self.degradation_rate = degredation_rate
        self.capacity_kw = capacity_kw
        self.production_method = production_method
        self.capital_expenditure = capital_expenditure
        self.operational_expenditure = operation_expenditure
        self.replacement_cost = replacement_cost


class TaxCredit45V:
    amount_usd: float

    def __init__(
        self,
        amount_usd: float = 0
    ) -> None:
        self.amount_usd = amount_usd


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


def purchase(
    simulation_id: int,
    electrolyzer: Electrolyzer,
    power_plant: PowerPlant,
    amount_mwh: float,
    timestamp: datetime,
) -> EnergyTransaction:
    for (i, generation) in enumerate(power_plant.generation):
        if generation.time_generated.year == timestamp.year and \
                generation.time_generated.month == timestamp.month:
            consumption = power_plant.consumption[i]
            return EnergyTransaction(
                simulation_id=simulation_id,
                electrolyzer_id=electrolyzer.id,
                plant_id=power_plant.plant_id,
                timestamp=timestamp,
                amount_mwh=amount_mwh,
                # This should only be some amount of the total mmbtu
                # (ratio of purchased to total generated)
                fuel_consumed_mmbtu=consumption.amount_mmbtu,
                price_usd=generation.sale_price_usd_per_mwh * amount_mwh,
                energy_source=power_plant.energy_source
            )
    raise Exception("No generation from powerplant at the current timestamp")


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


def calculate_emitted_co2(
    electrolyzer: Electrolyzer,
    portfolios: list[EnergySourcePortfolio],
) -> list[EmissionEvent]:
    emissions = []
    for portfolio in portfolios:
        amount_emitted_kg = 0
        amount_emitted_kg += calculate_emitted_co2_from_natural_gas(
            portfolio.natural_gas_mmbtu)
        emissions.append(EmissionEvent(
            electrolyzer_id=electrolyzer.id,
            emission_timestamp=portfolio.timestamp,
            amount_emitted_kg=amount_emitted_kg
        ))
    return emissions


def calculate_emitted_co2_from_natural_gas(natural_gas_mmbtu: float) -> float:
    return natural_gas_mmbtu * NATURAL_GAS_MMBTU_TO_CO2


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


def calculate_tax_credit(
    emissions: list[EmissionEvent],
    hydrogen_produced: list[HydrogenProduction]
) -> TaxCredit45V:
    credit = TaxCredit45V()
    total_co2_emitted = sum(
        map(lambda emission: emission.amount_emitted_kg, emissions))
    total_h2_produced = sum(
        map(lambda production: production.kg_hydrogen, hydrogen_produced))
    co2_per_h2 = total_co2_emitted / total_h2_produced

    if 2.5 <= co2_per_h2 and co2_per_h2 < 4:
        credit.amount_usd = TAX_CREDIT_45V_MAX_VALUE_USD * 0.2
    elif 1.5 <= co2_per_h2 and co2_per_h2 < 2.5:
        credit.amount_usd = TAX_CREDIT_45V_MAX_VALUE_USD * 0.2
    elif 0.45 <= co2_per_h2 and co2_per_h2 < 1.5:
        credit.amount_usd = TAX_CREDIT_45V_MAX_VALUE_USD * 0.334
    elif co2_per_h2 < 0.45:
        credit.amount_usd = TAX_CREDIT_45V_MAX_VALUE_USD

    return credit
