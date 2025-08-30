use std::net::UdpSocket;

use crate::mac_addr::MacAddr;

pub mod mac_addr;

pub fn wake_on_lan(
    mac: &str
)-> Result<(), Box<dyn std::error::Error>> {
    wake_on_lan_broadcast(mac, "255.255.255.255:9")
}


pub fn wake_on_lan_broadcast(
    mac: &str,
    broadcast_addr: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    let binding = match MacAddr::from_str(mac) {
        Ok(addr) => addr,
        Err(e) => return Err(e.into()),
    };

    let mac_bytes = binding.as_bytes();

    if mac_bytes.len() != 6 { return Err("MAC address must be 6 bytes".into()); }

    // Build magic packet
    let mut packet = vec![0xFF; 6];
    
    for _ in 0..16 {
        packet.extend(mac_bytes);
    }

    // Send packet to broadcast address
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.send_to(&packet, broadcast_addr)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wake_on_lan_with_overlength_mac_str() {
        let mac = "00:11:22:33:44:55:66";
        let result = wake_on_lan(mac);
        assert!(result.is_err());
    }

    #[test]
    fn test_wake_on_lan_with_empty_mac_str() {
        let result = wake_on_lan("");
        assert!(result.is_err());
    }

    #[test]
    fn test_wake_on_lan_with_known_mac() {
        let mac = "00:22:4d:9b:92:32";  // Test PC
        let result = wake_on_lan(mac);
        assert!(result.is_ok());
    }

}
