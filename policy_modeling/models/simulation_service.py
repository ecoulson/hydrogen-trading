from datetime import datetime
from logic.time_range import TimeRange
from logic.electrolyzer import Electrolyzer
from logic.simulation import SimulationResult
from logic.electrolyzer import ConstantProduction


class ExecuteSimulationRequest:
    simulation_time_range: TimeRange
    electrolyzer: Electrolyzer

    def __init__(
        self,
        simulation_time_range: TimeRange = TimeRange(),
        electrolyzer: Electrolyzer = Electrolyzer()
    ) -> None:
        self.simulation_time_range = simulation_time_range
        self.electrolyzer = electrolyzer

    # Need something so much fucking better than this this is ridiculous
    # why can't python just have decent libraries ffs
    @staticmethod
    def from_json(json):
        request = ExecuteSimulationRequest()
        request.simulation_time_range = TimeRange(start=datetime(
            year=json['simulation_time_range']['start']['year'],
            month=json['simulation_time_range']['start']['month'],
            day=json['simulation_time_range']['start']['day'],
            hour=json['simulation_time_range']['start']['hour'],
        ), end=datetime(
            year=json['simulation_time_range']['end']['year'],
            month=json['simulation_time_range']['end']['month'],
            day=json['simulation_time_range']['end']['day'],
            hour=json['simulation_time_range']['end']['hour'],
        ))
        request.electrolyzer = Electrolyzer(
            id=json['electrolyzer']['id'],
            replacement_threshold=json['electrolyzer']['replacement_threshold'],
            degredation_rate=json['electrolyzer']['degredation_rate'],
            capacity_mw=json['electrolyzer']['capacity_mw'],
            production_method=ConstantProduction(
                conversion_rate=json['electrolyzer']['production_method']['conversion_rate']),
            capital_expenditure=json['electrolyzer']['capital_expenditure'],
            operation_expenditure=json['electrolyzer']['operation_expenditure'],
            replacement_cost=json['electrolyzer']['replacement_cost']
        )
        return request


class ExecuteSimulationResponse:
    result: SimulationResult

    def __init__(
        self,
        result: SimulationResult = SimulationResult()
    ) -> None:
        self.result = result

    def to_json(self):
        return {
            'result': {
                'id': self.result.id,
                'tax_credit': {
                    'total_usd': self.result.tax_credit.total_usd,
                    'amount_usd_per_kg': self.result.tax_credit.amount_usd_per_kg
                },
                'emissions': list(map(lambda emission:
                                      {
                                          'simulation_id': emission.simulation_id,
                                          'electrolyzer_id': emission.electrolyzer_id,
                                          'emission_timestamp': emission.emission_timestamp.timestamp(),
                                          'amount_emitted_kg': emission.amount_emitted_kg
                                      }, self.result.emissions)),
                'hydrogen_produced': list(map(lambda hydrogen:
                                              {
                                                  'simulation_id': hydrogen.simulation_id,
                                                  'electrolyzer_id': hydrogen.electrolyzer_id,
                                                  'production_timestamp': hydrogen.production_timestamp.timestamp(),
                                                  'kg_hydrogen': hydrogen.kg_hydrogen,
                                              }, self.result.hydrogen_produced))
            }
        }
