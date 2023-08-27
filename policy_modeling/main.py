from datetime import datetime
from model import Electrolyzer, PowerPlant, \
    EnergySource, consume, ConsumptionUnit, \
    generate, get_energy_source_portfolios


def main():
    powerplant = PowerPlant(
        plant_id=50098,
        energy_source=EnergySource.ENERGY_SOURCE_NATURAL_GAS,
    )
    electrolyzer = Electrolyzer()

    current_timestep = datetime(
        year=2023,
        month=7,
        day=1
    )
    consume(powerplant, 15_706.68,
            ConsumptionUnit.CONSUMPTION_UNIT_MMBTU, datetime(
                year=2023,
                month=7,
                day=1
            ))
    consume(powerplant, 15_706.68,
            ConsumptionUnit.CONSUMPTION_UNIT_MMBTU, datetime(
                year=2023,
                month=8,
                day=1
            ))
    generate(powerplant, 1_351.545, current_timestep)

    portfolios = get_energy_source_portfolios([powerplant], current_timestep)

    print(
        portfolios[0].__dict__,
        portfolios[1].__dict__,
        electrolyzer
    )


main()
