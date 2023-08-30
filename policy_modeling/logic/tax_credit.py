from logic.emission import EmissionEvent
from logic.hydrogen_production import HydrogenProduction

TAX_CREDIT_45V_MAX_VALUE_USD = 3


class TaxCredit45V:
    amount_usd_per_kg: float
    total_usd: float

    def __init__(
        self,
        amount_usd_per_kg: float = 0,
        total_usd: float = 0
    ) -> None:
        self.amount_usd_per_kg = amount_usd_per_kg
        self.total_usd = total_usd


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
        credit.amount_usd_per_kg = TAX_CREDIT_45V_MAX_VALUE_USD * 0.2
    elif 1.5 <= co2_per_h2 and co2_per_h2 < 2.5:
        credit.amount_usd_per_kg = TAX_CREDIT_45V_MAX_VALUE_USD * 0.25
    elif 0.45 <= co2_per_h2 and co2_per_h2 < 1.5:
        credit.amount_usd_per_kg = TAX_CREDIT_45V_MAX_VALUE_USD * 0.334
    elif co2_per_h2 < 0.45:
        credit.amount_usd_per_kg = TAX_CREDIT_45V_MAX_VALUE_USD

    credit.total_usd = credit.amount_usd_per_kg * total_h2_produced

    return credit
