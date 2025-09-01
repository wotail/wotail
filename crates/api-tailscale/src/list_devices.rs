

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
            let id = f.get("id").expect("No ID").as_str().expect("Cannot parse ID").to_string();

            let client_connectivity = f.get("clientConnectivity").unwrap();
            //println!("{client_connectivity}");

            let ip = client_connectivity.get("endpoints").expect("No endpoints").as_array().expect("Cannot make into array");
            let ip: Vec<String> = ip.iter().map(|c| c.clone().to_string()).collect();

            let new_device = device::Device::new(id, ip);
            device_filtered.push(new_device);
        });
    }
    device_filtered.iter().for_each(|d| {
        println!("ID: {:}", d.device_id);
         d.local_ip.iter().for_each(|i|{
             println!("Potential Local IP: {i}");
         });
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