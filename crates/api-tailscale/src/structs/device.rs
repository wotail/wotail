pub struct Device {
    pub device_id: String,
    pub local_ip: Vec<String>
}
impl Device  {
    pub fn new(id: String, ip: Vec<String>) -> Self {
        Self { device_id: (id), local_ip: (ip) }
    }
}