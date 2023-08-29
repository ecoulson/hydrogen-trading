import math
from datetime import datetime
from grid import PowerGrid
from simulation import simulate
from time_range import TimeRange
from electrolyzer import Electrolyzer, ConstantProduction
from power_plant import PowerPlant, ConsumptionUnit, consume, generate, \
    EnergySource


def main():
    simulation_time_range = TimeRange(
        start=datetime(
            year=2023,
            month=7,
            day=1,
            hour=1
        ),
        end=datetime(
            year=2024,
            month=7,
            day=1,
            hour=1
        ))
    electrolyzer = Electrolyzer(
        id=0,
        replacement_threshold=0.8,
        degredation_rate=0.02,
        capacity_mw=1,
        production_method=ConstantProduction(conversion_rate=20),
        capital_expenditure=1150,
        operation_expenditure=17/(365*24),
        replacement_cost=0.5
    )
    power_grid = PowerGrid(
        power_plants=select_power_plants(selected_plant_ids=[50098]))

    result = simulate(simulation_time_range, electrolyzer, power_grid)

    hours = math.ceil((simulation_time_range.end -
                       simulation_time_range.start).total_seconds() / (60 * 60))
    average_emissions = sum(
        map(lambda emission: emission.amount_emitted_kg, result.emissions)) / hours
    average_kg_hydrogen = sum(
        map(lambda hydrogen: hydrogen.kg_hydrogen, result.hydrogen_produced)) / hours

    discount_rate = 0.0575
    opex_sum = 0
    hydrogen_sum = 0
    for i in range(0, hours):
        opex_sum += (electrolyzer.operational_expenditure /
                     (1 + math.pow(discount_rate, i)))
        hydrogen_sum += (result.hydrogen_produced[i].kg_hydrogen /
                         (1 + math.pow(discount_rate, i)))

    lcoh = (electrolyzer.capital_expenditure + opex_sum) / hydrogen_sum

    print(
        f"Ran from {simulation_time_range.start} to {simulation_time_range.end}")
    print(f"Average emission (kg/hr): {average_emissions}")
    print(f"Average hydrogen produced (kg/hr): {average_kg_hydrogen}")
    print(f"Tax credit ($): {result.tax_credit.total_usd}")
    print(f"LCOH $/(kg * hr): {lcoh}")


def select_power_plants(selected_plant_ids: list[int]) -> list[PowerPlant]:
    # TODO: Will be done in pipeline and this should be a db query
    all_power_plants = load_power_plants_in_memory()
    return list(filter(
        lambda power_plant:
        power_plant.plant_id in selected_plant_ids, all_power_plants))


def load_power_plants_in_memory() -> list[PowerPlant]:
    powerplant = PowerPlant(
        plant_id=50098,
        energy_source=EnergySource.ENERGY_SOURCE_NATURAL_GAS,
        heat_rate=0.2
    )

    consume(powerplant, 15_706.68,
            ConsumptionUnit.CONSUMPTION_UNIT_MMBTU, datetime(
                year=2023,
                month=7,
                day=1
            ))
    generate(powerplant, 1_351.545, datetime(
        year=2023,
        month=7,
        day=1
    ), sale_price=0.02)
    consume(powerplant, 15_706.68,
            ConsumptionUnit.CONSUMPTION_UNIT_MMBTU, datetime(
                year=2023,
                month=8,
                day=1
            ))
    generate(powerplant, 1_351.545, datetime(
        year=2023,
        month=8,
        day=1
    ), sale_price=2)

    return [powerplant]


main()
