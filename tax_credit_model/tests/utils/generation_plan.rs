use chrono::Duration;
use tax_credit_model_server::schema::{
    simulation_schema::{GenerationMetric, PowerPlant},
    time::{TimeRange, Timestamp},
};

pub trait GenerationPlan {
    fn generate(&self, power_plant: &PowerPlant, time_range: &TimeRange) -> Vec<GenerationMetric>;
}

pub struct ConstantGenerationPlan {
    amount_produced_mwh: f32,
    fuel_consumed_mmbtu: f32,
    sale_price_usd_per_mwh: f32,
}

impl GenerationPlan for ConstantGenerationPlan {
    fn generate(&self, power_plant: &PowerPlant, time_range: &TimeRange) -> Vec<GenerationMetric> {
        let mut current_timestamp = time_range.start.to_utc_date_time().unwrap();
        let mut generation_metrics = vec![];

        while current_timestamp <= time_range.end.to_utc_date_time().unwrap() {
            generation_metrics.push(GenerationMetric::new(
                power_plant.plant_id,
                &Timestamp::from(current_timestamp),
                self.amount_produced_mwh,
                self.sale_price_usd_per_mwh,
                self.fuel_consumed_mmbtu,
            ));
            current_timestamp += Duration::hours(1);
        }

        generation_metrics
    }
}
