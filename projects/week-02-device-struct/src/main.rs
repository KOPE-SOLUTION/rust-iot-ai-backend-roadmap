#[derive(Debug)]
struct Device{
    device_id: String,
    site_id: String,
    name: String,
    firmware_version: String,
    online: bool,
}

impl Device{
    fn new(
        device_id: &str,
        site_id: &str,
        name: &str,
        firmware_version: &str,
        online: bool,
    ) -> Self{
        Self {
            device_id: device_id.to_string(),
            site_id: site_id.to_string(),
            name: name.to_string(),
            firmware_version: firmware_version.to_string(),
            online,
        }
    }

    fn status(&self) -> &str{
        if self.online {
            "ONLINE"
        } else {
            "OFFLINE"
        }
    }

    fn print_summary(&self){
        println!("=== Device Summary ===");
        println!("Device ID         : {}", self.device_id);
        println!("Site ID           : {}", self.site_id);
        println!("Name              : {}", self.name);
        println!("Firmware Version  : {}", self.firmware_version);
        println!("Status            : {}", self.status());
    }
}

fn main() {
    let device = Device::new(
        "node-01",
        "farm-1",
        "Greenhouse Temperature Node",
        "v1.0.0",
        true,
    );

    println!("{:#?}", device);
    println!();

    device.print_summary();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_device() {
        let device = Device::new(
            "node-01",
            "farm-1",
            "Temp Node",
            "v1.0.0",
            true,
        );

        assert_eq!(device.device_id, "node-01");
        assert_eq!(device.site_id, "farm-1");
        assert_eq!(device.name, "Temp Node");
        assert_eq!(device.firmware_version, "v1.0.0");
        assert_eq!(device.online, true);
    }

    #[test]
    fn test_status_online() {
        let device = Device::new(
            "node-02",
            "farm-1",
            "EC Node",
            "v1.0.1",
            true,
        );

        assert_eq!(device.status(), "ONLINE");
    }

    #[test]
    fn test_status_offline() {
        let device = Device::new(
            "node-03",
            "farm-2",
            "pH Node",
            "v1.0.2",
            false,
        );

        assert_eq!(device.status(), "OFFLINE");
    }
}