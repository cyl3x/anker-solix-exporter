use std::sync::atomic::{AtomicU32, AtomicU64};

use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;

use crate::solix::data;

type GaugeU32 = Gauge<u32, AtomicU32>;
type GaugeF64 = Gauge<f64, AtomicU64>;

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {}

#[derive(Default)]
pub struct Metrics {
    pub registry: Registry,
    pub solarbank_power_percent: Family<Labels, GaugeU32>,
    pub solarbank_charging_power: Family<Labels, GaugeU32>,
    pub solarbank_output_power: Family<Labels, GaugeF64>,
    pub photovoltaic_power: Family<Labels, GaugeU32>,
    pub photovoltaic_power_1: Family<Labels, GaugeU32>,
    pub photovoltaic_power_2: Family<Labels, GaugeU32>,
    pub photovoltaic_power_3: Family<Labels, GaugeU32>,
    pub photovoltaic_power_4: Family<Labels, GaugeU32>,
    pub statistic_total_power: Family<Labels, GaugeF64>,
    pub grid_to_home_power: Family<Labels, GaugeU32>,
    pub photovoltaic_to_grid_power: Family<Labels, GaugeU32>,
}

impl Metrics {
    pub fn new() -> Self {
        let mut metrics = Self::default();

        metrics.registry.register(
            "anker_solix_solarbank_power_percent",
            "Total power of bank in percent (percent)",
            metrics.solarbank_power_percent.clone(),
        );
        metrics.registry.register(
            "anker_solix_solarbank_charging_power",
            "Charging power of bank (watt)",
            metrics.solarbank_charging_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_solarbank_output_power",
            "Output power of bank (watt)",
            metrics.solarbank_output_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_photovoltaic_power",
            "Photovoltaic power (watt)",
            metrics.photovoltaic_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_photovoltaic_power_1",
            "Power of photovoltaic 1 (watt)",
            metrics.photovoltaic_power_1.clone(),
        );
        metrics.registry.register(
            "anker_solix_photovoltaic_power_2",
            "Power of photovoltaic 2 (watt)",
            metrics.photovoltaic_power_2.clone(),
        );
        metrics.registry.register(
            "anker_solix_photovoltaic_power_3",
            "Power of photovoltaic 3 (watt)",
            metrics.photovoltaic_power_3.clone(),
        );
        metrics.registry.register(
            "anker_solix_photovoltaic_power_4",
            "Power of photovoltaic 4 (watt)",
            metrics.photovoltaic_power_4.clone(),
        );
        metrics.registry.register(
            "anker_solix_statistic_total_power",
            "Total power generated statistic (kilo-watt)",
            metrics.statistic_total_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_grid_to_home_power",
            "Total power that flows from the grid into home (watt)",
            metrics.grid_to_home_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_photovoltaic_to_grid_power",
            "Total power that flows from the home to the grid (watt)",
            metrics.photovoltaic_to_grid_power.clone(),
        );

        metrics
    }

    pub fn gather(&self) -> String {
        let mut buffer = String::new();
        encode(&mut buffer, &self.registry).unwrap();

        buffer
    }

    pub fn update(&self, scene_data: data::ScenInfo) {
        let labels = Labels::default();

        self.solarbank_power_percent
            .get_or_create(&labels)
            .set((scene_data.solarbank_info.total_battery_power * 100.0) as u32);
        self.solarbank_charging_power
            .get_or_create(&labels)
            .set(scene_data.solarbank_info.total_charging_power);
        self.solarbank_output_power
            .get_or_create(&labels)
            .set(scene_data.solarbank_info.total_output_power);
        self.photovoltaic_power
            .get_or_create(&labels)
            .set(scene_data.solarbank_info.total_photovoltaic_power);
        self.photovoltaic_power_1
            .get_or_create(&labels)
            .set(scene_data.solarbank_info.solar_power_1);
        self.photovoltaic_power_2
            .get_or_create(&labels)
            .set(scene_data.solarbank_info.solar_power_2);
        self.photovoltaic_power_3
            .get_or_create(&labels)
            .set(scene_data.solarbank_info.solar_power_3);
        self.photovoltaic_power_4
            .get_or_create(&labels)
            .set(scene_data.solarbank_info.solar_power_4);
        self.grid_to_home_power
            .get_or_create(&labels)
            .set(scene_data.grid_info.grid_to_home_power);
        self.photovoltaic_to_grid_power
            .get_or_create(&labels)
            .set(scene_data.grid_info.photovoltaic_to_grid_power);

        if let Some(m) = scene_data
            .statistics
            .iter()
            .find(|s| s.r#type == 1)
            .map(|s| s.total)
        {
            self.statistic_total_power.get_or_create(&labels).set(m);
        }
    }
}
