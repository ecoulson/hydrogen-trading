from datetime import datetime
from model import Electrolyzer, PowerPlant, \
    EnergySource, consume, ConsumptionUnit, \
    generate, get_energy_source_portfolios, calculate_emitted_co2, \
    ConstantProduction, purchase, calculate_hydrogen_produced, \
    calculate_tax_credit


def main():
    simulation_id = 0
    powerplant = PowerPlant(
        plant_id=50098,
        energy_source=EnergySource.ENERGY_SOURCE_NATURAL_GAS,
    )
    electrolyzer = Electrolyzer(
        id=0,
        replacement_threshold=0.8,
        degredation_rate=0.02,
        capacity_kw=1000,
        production_method=ConstantProduction(conversion_rate=45),
        capital_expenditure=1150,
        operation_expenditure=17,
        replacement_cost=0.5
    )

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
    generate(powerplant, 1_351.545, datetime(
        year=2023,
        month=7,
        day=1
    ), sale_price=2)
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
    energy_transactions = [
        purchase(simulation_id, electrolyzer,
                 powerplant, 20000, current_timestep)
    ]
    portfolios = get_energy_source_portfolios(energy_transactions)
    emissions = calculate_emitted_co2(electrolyzer, portfolios)
    hydrogen_productions = calculate_hydrogen_produced(
        electrolyzer, portfolios)

    print("-==- ELECTROLYZER -==-")
    print(electrolyzer.__dict__)
    print("----------------------")

    print("-==- TRANSACTIONS -==-")
    for transaction in energy_transactions:
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


main()
