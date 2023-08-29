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
        production_method=ConstantProduction(conversion_rate=10000),
        capital_expenditure=1150,
        operation_expenditure=17/(365*24),
        replacement_cost=0.5
    )
    power_grid = PowerGrid(
        power_plants=select_power_plants(selected_plant_ids=[50098]))

    result = simulate(simulation_time_range, electrolyzer, power_grid)

    print(
        f"Ran from {simulation_time_range.start} to {simulation_time_range.end}")
    print("Revenue $USD:", result.revenue)
    print("Cost $USD:", result.cost)
    print("Profit $USD:", result.revenue - result.cost)


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
        heat_rate=0.1
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
