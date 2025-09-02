use serde::Deserialize;

#[derive(Deserialize)]
pub struct TailscaleResponse {
    pub devices: Vec<DeviceEntry>
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct DeviceEntry {
    pub id: String,
    pub name: String,
    pub clientConnectivity: Connectivity
}
#[derive(Deserialize)]
pub struct  Connectivity {
    pub endpoints: Vec<String>
}