use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Deserialize, Debug)]
pub struct Login {
    pub auth_token: String,
    pub token_expires_at: u64,
    pub user_id: String,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct SolarbankInfo {
    #[serde_as(as = "DisplayFromStr")]
    pub total_battery_power: f32,
    #[serde_as(as = "DisplayFromStr")]
    pub total_charging_power: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub total_output_power: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub total_photovoltaic_power: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub solar_power_1: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub solar_power_2: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub solar_power_3: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub solar_power_4: u32,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Statistic {
    #[serde_as(as = "DisplayFromStr")]
    pub total: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub r#type: u32,
    pub unit: String,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct GridInfo {
    #[serde_as(as = "DisplayFromStr")]
    pub grid_to_home_power: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub photovoltaic_to_grid_power: u32,
}

#[derive(Deserialize, Debug)]
pub struct ScenInfo {
    pub grid_info: GridInfo,
    pub solarbank_info: SolarbankInfo,
    pub statistics: Vec<Statistic>,
}
