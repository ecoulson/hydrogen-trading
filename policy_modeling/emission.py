from datetime import datetime
from electrolyzer import Electrolyzer
from energy_source_portfolio import EnergySourcePortfolio

NATURAL_GAS_MMBTU_TO_CO2 = 53.0703


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
