from datetime import datetime


class TimeRange:
    start: datetime
    end: datetime

    def __init__(
        self,
        start: datetime = datetime.now(),
        end: datetime = datetime.now()
    ) -> None:
        self.start = start
        self.end = end
