use serde::{self, Deserialize};
use serde_tuple::Deserialize_tuple;

#[derive(Deserialize_tuple, Debug)]
pub struct OvationAuroraLatestCoordinates {
    pub longitude: i16,
    pub latitude: i16,
    pub value: i16,
}

#[derive(Deserialize, Debug)]

pub struct OvationAuroraLatest {
    #[serde(rename = "Observation Time")]
    pub observation_time: String,
    #[serde(rename = "Forecast Time")]
    pub forcast_time: String,
    #[serde(rename = "Data Format")]
    pub data_format: String,
    #[serde(rename = "coordinates")]
    pub coordinates: Vec<OvationAuroraLatestCoordinates>,
    #[serde(rename = "type")]
    pub data_type: String,
}
