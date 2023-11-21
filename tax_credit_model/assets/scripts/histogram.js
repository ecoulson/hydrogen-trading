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

    retrieveDataset(parameters.endpoints.split(","), (histograms) => {
        for (let histogram of histograms) {
            new Chart(document.getElementById(parameters.id), {
                type: 'bar',
                data: {
                    labels: histogram.labels,
                    datasets: histogram.datasets.map((dataset) => {
                        return {
                            label: dataset.label,
                            data: dataset.data_points,
                        }
                    }),
                },
            });
        }
    });

})();
