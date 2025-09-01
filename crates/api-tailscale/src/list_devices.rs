

use reqwest::{self, Client};
use crate::structs::device::{self, Device};
use std::error::Error;


pub async fn get_devices() -> Result<Vec<Device>, Box<dyn Error>>{

    let token = env::var("TAILSCALE_TOKEN")?;
    let tailnet = env::var("TAILNET")?;
    let url = "https://api.tailscale.com/api/v2/tailnet/".to_string() + &tailnet + "/devices?fields=all";

    let response =  Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    let mut device_filtered: Vec<device::Device> = Vec::new();
    let devices = response.get("devices")
        .ok_or("API Response does not contain devices")?
        .as_array()
        .ok_or("Devices is not an array")?;
    devices.iter().for_each(|f| {
        let id = f.get("id")
        .ok_or("Device does not contain a ID")
        .as_str()
        .ok_or(err)
        .expect("Cannot parse ID").to_string();

        let client_connectivity = f.get("clientConnectivity").unwrap();


        let ip = client_connectivity.get("endpoints").expect("No endpoints").as_array().expect("Cannot make into array");
        let ip: Vec<String> = ip.iter().map(|c| c.clone().to_string()).collect();

        let new_device = device::Device::new(id, ip);
        device_filtered.push(new_device);
    });
    


    Ok(device_filtered)
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_devices(){
        let _ = get_devices().await.expect("Error");
    }
}