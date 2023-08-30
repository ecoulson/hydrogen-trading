from flask import Blueprint, request
import math
import json
from datetime import datetime
from logic.grid import PowerGrid
from logic.simulation import simulate
from logic.power_plant import PowerPlant, ConsumptionUnit, \
    consume, generate, EnergySource
from models.simulation_service import ExecuteSimulationResponse, \
    ExecuteSimulationRequest

register = Blueprint('register', __name__)


@register.post("/ExecuteSimulation")
def route() -> str:
    response_model = handle(
        ExecuteSimulationRequest.from_json(json.loads(request.data)))
    # This sends back too much data think the response should just hold the state of the
    # simulation and this data can be queried from a data source
    return json.dumps(response_model.to_json())


def handle(request: ExecuteSimulationRequest) -> ExecuteSimulationResponse:
    electrolyzer = request.electrolyzer
    simulation_time_range = request.simulation_time_range
    power_grid = PowerGrid(
        power_plants=load_power_plants_in_memory())

    result = simulate(simulation_time_range,
                      electrolyzer, power_grid)
    diff = simulation_time_range.end - simulation_time_range.start
    hours = math.ceil(diff.total_seconds() / (60 * 60))
    average_emissions = sum(
        map(lambda emission: emission.amount_emitted_kg, result.emissions)) \
        / hours
    average_kg_hydrogen = sum(
        map(lambda hydrogen: hydrogen.kg_hydrogen, result.hydrogen_produced)) \
        / hours

    discount_rate = 0.0575
    opex_sum = 0
    hydrogen_sum = 0
    for i in range(0, hours):
        opex_sum += (electrolyzer.operational_expenditure /
                     (1 + math.pow(discount_rate, i)))
        hydrogen_sum += (result.hydrogen_produced[i].kg_hydrogen /
                         (1 + math.pow(discount_rate, i)))

    lcoh = (electrolyzer.capital_expenditure + opex_sum) / hydrogen_sum

    print(f"From {simulation_time_range.start} to {simulation_time_range.end}")
    print(f"Average emission (kg/hr): {average_emissions}")
    print(f"Average hydrogen produced (kg/hr): {average_kg_hydrogen}")
    print(f"Tax credit ($): {result.tax_credit.total_usd}")
    print(f"LCOH $/(kg * hr): {lcoh}")

    return ExecuteSimulationResponse(result)


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
