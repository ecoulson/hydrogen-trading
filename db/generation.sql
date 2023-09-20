CREATE TABLE GenerationMetrics (
    id VARCHAR,
    plant_id INTEGER,
    time_generated JSON,
    sale_price DOUBLE PRECISION,
    portfolio JSON
);

GRANT INSERT ON GenerationMetrics TO hydrogen_trading_dev;
