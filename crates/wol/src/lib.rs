use tokio::net::UdpSocket;

use wotail_lib::mac_addr::MacAddr;

// /// Sends a wake on lan packet to a specified mac address (broadcasting to 255.255.255.255:9)
// pub async fn wake_on_lan(
//     mac: &str
// )-> Result<(), Box<dyn std::error::Error>> {
//     wake_on_lan_broadcast(mac, "255.255.255.255:9").await // TODO: could be more efficient
// }

/// Sends a wake on lan packet to a specified mac address from a specific broadcast address
pub async fn wake_on_lan(
    mac: &str,
    broadcast: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {

    let broadcast_addr = match broadcast {
        Some(addr) => addr,
        None => "255.255.255.255:9",
    };

    let binding = match MacAddr::from_str(mac) {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    let mac_bytes: &[u8] = binding.as_bytes();

    // Build magic packet
    let mut packet: Vec<u8> = vec![0xFF; 6];
    for _ in 0..16 {
        packet.extend(mac_bytes);
    }

    // Send packet to broadcast address
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;
    socket.send_to(&packet, broadcast_addr).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[tokio::test]
    async fn test_wake_on_lan_with_overlength_mac_str() {
        let mac: &'static str = "00:11:22:33:44:55:66";
        let result: Result<(), Box<dyn Error>> = wake_on_lan(mac, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wake_on_lan_with_empty_mac_str() {
        let result: Result<(), Box<dyn Error>> = wake_on_lan("", None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wake_on_lan_with_known_mac() {
        let mac: &'static str = "00:22:4d:9b:92:32";  // Test PC
        let result: Result<(), Box<dyn Error>> = wake_on_lan(mac, None).await;
        assert!(result.is_ok());
    }

}
