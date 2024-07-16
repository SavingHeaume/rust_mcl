use serde::Deserialize;
use crate::{library::Library, asset::AssetIndex};

pub type Libraries = Vec<Library>;

#[derive(Deserialize)]
pub struct Version {
    pub asset_index: AssetIndex,
    pub downloads: Download,
    pub id: String,
    pub libraries: Libraries,
    pub main_class: String,
    pub release_time: String,
    pub time: String,
    #[serde(alias = "type")]
    pub type_: String,
}


#[derive(Deserialize)]
pub struct Download {
    pub client: Client,
}

#[derive(Deserialize)]
pub struct Client {
    pub sha1: String,
    pub size: u32,
    pub url: String,
}
