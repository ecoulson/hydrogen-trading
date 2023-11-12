(function() {
    const parameters = parseScriptParameters();

    function retrieveDataset(endpoints, callback) {
        const requests = [];

        for (let endpoint of endpoints) {
            requests.push(fetch(endpoint, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                }
            })
                .then((response) => response.json())
                .then((dataset) => dataset)
                .catch((error) => console.error("Failed to parse response", error))
            )
        }

        Promise
            .all(requests)
            .then((datasets) => callback(datasets))
            .catch((error) => console.log("Failed to retrieve data set", error));
    }

    retrieveDataset(parameters.endpoints.split(","), (datasets) => {
        new Chart(document.getElementById(parameters.id), {
            type: 'line',
            data: {
                datasets: datasets.map((dataset) => {
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
                        type: 'time'
                    }
                }
            }
        });
    });

})();
