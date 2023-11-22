const CHART_DB = new Map();

function updateChartData(id, datasets) {
    chart = CHART_DB.get(id);

    if (!chart) {
        console.error(`No chart with id ${id}`);
        console.error("^ Should trigger htmx error");
        return;
    }

    chart.data.datasets = datasets.map(convertTimeSeriesDataToChartJsDataset);
    chart.update();
}

function createTimeSeriesChart(id, timeSeries) {
    let chart = new Chart(document.getElementById(id), {
        type: 'line',
        data: {
            datasets: timeSeries.datasets.map(convertTimeSeriesDataToChartJsDataset),
        },
        options: {
            scales: {
                xAxes: {
                    type: 'time',
                    title: {
                        display: true,
                        text: timeSeries.labels.x
                    }
                },
                y: {
                    title: {
                        display: true,
                        text: timeSeries.labels.y
                    }
                }
            }
        }
    });
    CHART_DB.set(id, chart);

    return chart;
}

function createHistogram(id, histogram) {
    console.log(histogram);
    let chart = new Chart(document.getElementById(id), {
        type: 'bar',
        data: {
            labels: histogram.keys,
            datasets: histogram.datasets.map((dataset) => {
                return {
                    label: dataset.label,
                    data: dataset.data_points,
                }
            }),
        },
        options: {
            scales: {
                xAxes: {
                    title: {
                        display: true,
                        text: histogram.label.x
                    }
                },
                y: {
                    title: {
                        display: true,
                        text: histogram.label.y
                    }
                }
            }

        }
    });
    CHART_DB.set(id, chart);

    return chart;
}

function convertTimeSeriesDataToChartJsDataset(dataset) {
    return {
        label: dataset.label,
        segment: {
            borderColor: (context) => context.p0.raw.color
        },
        borderColor: (context) => {
            switch (context.type) {
                case "data":
                    return context.dataset.data[context.dataIndex].color;
                case "dataset":
                    return context.dataset.color;
                default:
                    return "black";
            }
        },
        data: dataset.data_points.map((data_point) => {
            return {
                x: data_point.date,
                y: data_point.value,
                color: data_point.color,
            };
        }),
    }
}
