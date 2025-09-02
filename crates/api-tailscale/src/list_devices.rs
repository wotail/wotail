

use reqwest::{self, Client};
use wotail_commons::ip_addr_v4::IPAddrV4;
use crate::structs::{device::Device, tailscale_response::TailscaleResponse};
use std::{error::Error};




pub async fn get_devices() -> Result<Vec<Device>, Box<dyn Error>>{

    let token = env::var("TAILSCALE_TOKEN")?;
    let tailnet = env::var("TAILNET")?;
    let url = "https://api.tailscale.com/api/v2/tailnet/".to_string() + &tailnet + "/devices?fields=all";

    let response =  Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?
        .json::<TailscaleResponse>()
        .await?;

    let mut devices: Vec<Device> = Vec::new();

    for device in response.devices {
        let name = device.name;
        let id = device.id;
        let mut ip: Vec<IPAddrV4> = Vec::new();

        for endpoint in device.clientConnectivity.endpoints {
            let endpoint = &endpoint[..endpoint.find(':').unwrap_or(endpoint.len())];
            let device_ip = IPAddrV4::from_str(&endpoint)?;
            if device_ip.is_local() {
                ip.push(device_ip);
            }
        }


        devices.push(Device::new(id, name, ip));
    }
    Ok(devices)
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_devices(){
        assert!(get_devices().await.is_ok())
    }
}