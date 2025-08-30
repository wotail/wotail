use reqwest::{self, Client};

pub async fn get_devices(){

    let token = env::var_os("TAILSCALE_TOKEN").unwrap().into_string().expect("Token Environment variable not set");
    let tailnet = env::var_os("TAILNET").unwrap().into_string().expect("Token Environment variable not set");

    let url = "https://api.tailscale.com/api/v2/tailnet/".to_string() + &tailnet + "/devices?fields=all";

    let response = Client::new()
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .send().await
        .expect("womp womp").text().await.expect("womp womp womp");
    println!("{response}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_devices(){
        get_devices().await;
    }
}