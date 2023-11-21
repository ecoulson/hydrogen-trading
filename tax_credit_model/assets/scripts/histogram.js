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
            .then((histogram) => callback(histogram))
            .catch((error) => console.error("Failed to parse response", error))
    }

    retrieveDataset(parameters.endpoint, (histogram) => {
        new Chart(document.getElementById(parameters.id), {
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
                            text: histogram.labels.x
                        }
                    },
                    y: {
                        title: {
                            display: true,
                            text: histogram.labels.y
                        }
                    }
                }

            }
        });
    });

})();
