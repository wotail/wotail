pub struct Device {
    pub device_id: String,
    pub local_ip: String
}
impl Device  {
    pub fn new(id: String, ip: String) -> Self {
        Self { device_id: (id), local_ip: (ip) }
    }
}