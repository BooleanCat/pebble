use serde::Deserialize;

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct ConfigRoot {
    pub path: String,
    pub read_only: Option<bool>,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub oci_version: String,
    pub root: ConfigRoot,
}
