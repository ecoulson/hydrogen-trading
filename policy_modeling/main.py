from datetime import datetime
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
            day=1
        ),
        end=datetime(
            year=2023,
            month=8,
            day=1
        ))
    powerplant = PowerPlant(
        plant_id=50098,
        energy_source=EnergySource.ENERGY_SOURCE_NATURAL_GAS,
        heat_rate=12
    )
    electrolyzer = Electrolyzer(
        id=0,
        replacement_threshold=0.8,
        degredation_rate=0.02,
        capacity_kw=1000,
        production_method=ConstantProduction(conversion_rate=45),
        capital_expenditure=1150000,
        operation_expenditure=17/(365*24),
        replacement_cost=0.5
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

    result = simulate(simulation_time_range, electrolyzer, [powerplant])

    print("Revenue $USD:", result.revenue)
    print("Cost $USD:", result.cost)
    print("Profit $USD:", result.revenue - result.cost)


main()
