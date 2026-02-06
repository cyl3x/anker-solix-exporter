use std::sync::atomic::{AtomicU32, AtomicU64};

use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;

use crate::solix::data;

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    site_id: String,
    unit: String,
}

impl Labels {
    pub fn new(site_id: &str, unit: &str) -> Self {
        Self {
            site_id: site_id.into(),
            unit: unit.into(),
        }
    }
}

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct SolarbankLabels {
    site_id: String,
    unit: String,
    device_sn: String,
}

impl SolarbankLabels {
    pub fn new(site_id: &str, unit: &str, device_sn: &str) -> Self {
        Self {
            site_id: site_id.into(),
            unit: unit.into(),
            device_sn: device_sn.into(),
        }
    }
}

type GaugeU32<T = Labels> = Family<T, Gauge<u32, AtomicU32>>;
type GaugeF64<T = Labels> = Family<T, Gauge<f64, AtomicU64>>;

#[derive(Default)]
pub struct Metrics {
    pub registry: Registry,

    pub home_load_power: GaugeU32,
    pub other_load_power: GaugeU32,

    pub grid_to_home_power: GaugeU32,
    pub photovoltaic_to_grid_power: GaugeU32,

    pub home_charging_power: GaugeF64,

    pub statistics_total_power: GaugeF64,
    pub statistics_total_co2: GaugeF64,
    pub statistics_total_money: GaugeF64,

    pub solar_power_1: GaugeU32,
    pub solar_power_2: GaugeU32,
    pub solar_power_3: GaugeU32,
    pub solar_power_4: GaugeU32,

    pub solarbank_battery_power: GaugeU32<SolarbankLabels>,
    pub solarbank_charging_power: GaugeU32<SolarbankLabels>,
    pub solarbank_output_power: GaugeU32<SolarbankLabels>,
    pub solarbank_photovoltaic_power: GaugeU32<SolarbankLabels>,

    pub solarbank_total_battery_power: GaugeF64,
    pub solarbank_total_charging_power: GaugeU32,
    pub solarbank_total_output_power: GaugeF64,
    pub solarbank_total_photovoltaic_power: GaugeU32,
}

impl Metrics {
    pub fn new() -> Self {
        let mut metrics = Self::default();

        metrics.registry.register(
            "anker_solix_home_load_power",
            "Home load power",
            metrics.home_load_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_other_load_power",
            "Other load power",
            metrics.other_load_power.clone(),
        );

        metrics.registry.register(
            "anker_solix_grid_to_home_power",
            "Grid to home power",
            metrics.grid_to_home_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_photovoltaic_to_grid_power",
            "Photovoltaic to grid power",
            metrics.photovoltaic_to_grid_power.clone(),
        );

        metrics.registry.register(
            "anker_solix_home_charging_power",
            "Home charging power",
            metrics.home_charging_power.clone(),
        );

        metrics.registry.register(
            "anker_solix_statistics_total_power",
            "Statistics total power",
            metrics.statistics_total_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_statistics_total_co2",
            "Statistics total CO2",
            metrics.statistics_total_co2.clone(),
        );
        metrics.registry.register(
            "anker_solix_statistics_total_money",
            "Statistics total money",
            metrics.statistics_total_money.clone(),
        );

        metrics.registry.register(
            "anker_solix_solar_power_1",
            "Solar power 1",
            metrics.solar_power_1.clone(),
        );
        metrics.registry.register(
            "anker_solix_solar_power_2",
            "Solar power 2",
            metrics.solar_power_2.clone(),
        );
        metrics.registry.register(
            "anker_solix_solar_power_3",
            "Solar power 3",
            metrics.solar_power_3.clone(),
        );
        metrics.registry.register(
            "anker_solix_solar_power_4",
            "Solar power 4",
            metrics.solar_power_4.clone(),
        );

        metrics.registry.register(
            "anker_solix_solarbank_battery_power",
            "Solarbank power percent",
            metrics.solarbank_battery_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_solarbank_charging_power",
            "Solarbank charging power",
            metrics.solarbank_charging_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_solarbank_output_power",
            "Solarbank output power",
            metrics.solarbank_output_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_solarbank_photovoltaic_power",
            "Solarbank photovoltaic power",
            metrics.solarbank_photovoltaic_power.clone(),
        );

        metrics.registry.register(
            "anker_solix_solarbank_total_charging_power",
            "Solarbank total charging power",
            metrics.solarbank_total_charging_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_solarbank_total_output_power",
            "Solarbank total output power",
            metrics.solarbank_total_output_power.clone(),
        );
        metrics.registry.register(
            "anker_solix_solarbank_total_photovoltaic_power",
            "Solarbank total photovoltaic power",
            metrics.solarbank_total_photovoltaic_power.clone(),
        );

        metrics
    }

    pub fn gather(&self) -> String {
        let mut buffer = String::new();
        encode(&mut buffer, &self.registry).unwrap();

        buffer
    }

    pub fn update(&self, site_id: &str, scene_data: data::ScenInfo) {
        let grid_labels = Labels::new(site_id, "W");

        self.home_load_power
            .get_or_create(&grid_labels)
            .set(scene_data.home_load_power);
        self.other_load_power
            .get_or_create(&grid_labels)
            .set(scene_data.other_loads_power);

        self.grid_to_home_power
            .get_or_create(&grid_labels)
            .set(scene_data.grid_info.grid_to_home_power);
        self.photovoltaic_to_grid_power
            .get_or_create(&grid_labels)
            .set(scene_data.grid_info.photovoltaic_to_grid_power);

        self.home_charging_power
            .get_or_create(&Labels::new(site_id, &scene_data.home_info.power_unit))
            .set(scene_data.home_info.charging_power);

        self.statistics_total_power
            .get_or_create(&Labels::new(site_id, &scene_data.statistics[0].unit))
            .set(scene_data.statistics[0].total);
        self.statistics_total_co2
            .get_or_create(&Labels::new(site_id, &scene_data.statistics[1].unit))
            .set(scene_data.statistics[1].total);
        self.statistics_total_money
            .get_or_create(&Labels::new(site_id, &scene_data.statistics[2].unit))
            .set(scene_data.statistics[2].total);

        let solar_power_labels = Labels::new(site_id, &scene_data.solarbank_info.power_unit);

        self.solar_power_1
            .get_or_create(&solar_power_labels)
            .set(scene_data.solarbank_info.solar_power_1);
        self.solar_power_2
            .get_or_create(&solar_power_labels)
            .set(scene_data.solarbank_info.solar_power_2);
        self.solar_power_3
            .get_or_create(&solar_power_labels)
            .set(scene_data.solarbank_info.solar_power_3);
        self.solar_power_4
            .get_or_create(&solar_power_labels)
            .set(scene_data.solarbank_info.solar_power_4);

        for solarbank in &scene_data.solarbank_info.solarbank_list {
            let solarbank_labels =
                SolarbankLabels::new(site_id, &solarbank.power_unit, &solarbank.device_sn);

            self.solarbank_battery_power
                .get_or_create(&solarbank_labels)
                .set(solarbank.battery_power);
            self.solarbank_charging_power
                .get_or_create(&solarbank_labels)
                .set(solarbank.charging_power);
            self.solarbank_output_power
                .get_or_create(&solarbank_labels)
                .set(solarbank.output_power);
            self.solarbank_photovoltaic_power
                .get_or_create(&solarbank_labels)
                .set(solarbank.photovoltaic_power);
        }

        let solarbank_total_labels = Labels::new(site_id, &scene_data.solarbank_info.power_unit);

        self.solarbank_total_battery_power
            .get_or_create(&solarbank_total_labels)
            .set(scene_data.solarbank_info.total_battery_power);
        self.solarbank_total_charging_power
            .get_or_create(&solarbank_total_labels)
            .set(scene_data.solarbank_info.total_charging_power);
        self.solarbank_total_output_power
            .get_or_create(&solarbank_total_labels)
            .set(scene_data.solarbank_info.total_output_power);
        self.solarbank_total_photovoltaic_power
            .get_or_create(&solarbank_total_labels)
            .set(scene_data.solarbank_info.total_photovoltaic_power);

        log::info!("Updated metrics for site {site_id}");
    }
}
