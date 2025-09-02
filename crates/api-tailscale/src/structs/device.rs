use wotail_commons::ip_addr_v4::IPAddrV4;
use wotail_commons::mac_addr::MacAddr;

pub struct Device {
    pub device_id: String,
    pub device_name: String,
    pub local_ip: Vec<IPAddrV4>,
    pub mac_address: Option<MacAddr>
}
impl Device  {
    pub fn new(id: String, name: String, ip: Vec<IPAddrV4>) -> Self {
        Self { 
            device_id: id,
            device_name: name, 
            local_ip: ip, 
            mac_address: None 
        }
    }
    pub fn with_mac(id: String, name: String, ip: Vec<IPAddrV4>, mac: MacAddr) -> Self{
        Self { 
            device_id: id, 
            device_name: name, 
            local_ip: ip, 
            mac_address: Some(mac) 
        }
    }
}