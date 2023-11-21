(function() {
    const parameters = parseScriptParameters();

    function retrieveDataset(endpoint, callback) {
        fetch(endpoint, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                }
            })
                .then((response) => response.json())
                .then((timeSeries) => callback(timeSeries))
                .catch((error) => console.error("Failed to parse response", error))
    }

    retrieveDataset(parameters.endpoint, (time_series) => {
        new Chart(document.getElementById(parameters.id), {
            type: 'line',
            data: {
                datasets: time_series.datasets.map((dataset) => {
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

                }),
            },
            options: {
                scales: {
                    xAxes: {
                        type: 'time',
                        title: {
                            display: true,
                            text: time_series.labels.x
                        }
                    },
                    y: {
                        title: {
                            display: true,
                            text: time_series.labels.y
                        }
                    }
                }
            }
        });
    });

})();
