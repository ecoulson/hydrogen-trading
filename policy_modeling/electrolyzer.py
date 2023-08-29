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
