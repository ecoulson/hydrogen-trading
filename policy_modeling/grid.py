from power_plant import PowerPlant


class PowerGrid:
    power_plants: list[PowerPlant]

    def __init__(
        self,
        power_plants: list[PowerPlant] = []
    ) -> None:
        self.power_plants = power_plants
