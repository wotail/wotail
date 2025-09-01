

use reqwest::{self, Client};
use crate::structs::tailscale_response::TailscaleResponse;
use std::{error::Error};




pub async fn get_devices() -> Result<TailscaleResponse, Box<dyn Error>>{

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
    Ok(response)
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_devices(){
        assert!(get_devices().await.is_ok());
    }
}