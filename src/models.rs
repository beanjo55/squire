use serde::{self, Deserialize};

#[derive(Deserialize)]
pub struct OvationAuroraLatest {
    #[serde(rename = "Observation Time")]
    pub observation_time: String,
    #[serde(rename = "Forecast Time")]
    pub forcast_time: String,
    #[serde(rename = "Data Format")]
    pub data_format: String,
    #[serde(rename = "coordinates")]
    pub coordinates: Vec<[i16; 3]>,
    #[serde(rename = "type")]
    pub data_type: String,
}
