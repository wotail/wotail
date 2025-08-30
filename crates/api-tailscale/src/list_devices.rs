

use reqwest::{self, Client};
use crate::structs::device;

pub async fn get_devices(){

    let token = env::var_os("TAILSCALE_TOKEN")
        .unwrap()
        .into_string()
        .expect("Token Environment variable not set");
    let tailnet = env::var_os("TAILNET")
        .unwrap()
        .into_string()
        .expect("Token Environment variable not set");

    let url = "https://api.tailscale.com/api/v2/tailnet/".to_string() + &tailnet + "/devices?fields=all";

    let response = Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .expect("No response").json::<serde_json::Value>()
        .await
        .expect("Response is not a json");
    // let Devices: Vec<device::Device>;
    let devices = response.get("devices");
    let mut device_filtered: Vec<device::Device> = Vec::new();
    if devices.is_some() {
        let test = devices.unwrap().as_array().expect("test");
        test.iter().for_each(|f| {
            let id = f.get("id").expect("test").as_str().expect("test").to_string();
            device_filtered.push(device::Device::new(id, "test".to_string()));
        });
    }
    device_filtered.iter().for_each(|d| {
        println!("ID: {:}", d.device_id);
        println!("IP: {:}", d.local_ip);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_devices(){
        get_devices().await;
    }
}