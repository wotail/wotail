use wotail_commons::ip_addr_v4::IPAddrV4;

pub struct Device {
    pub device_id: String,
    pub device_name: String,
    pub local_ip: Vec<IPAddrV4>
}
impl Device  {
    pub fn new(id: String, name: String, ip: Vec<IPAddrV4>) -> Self {
        Self { device_id: (id), device_name: (name), local_ip: (ip) }
    }
}