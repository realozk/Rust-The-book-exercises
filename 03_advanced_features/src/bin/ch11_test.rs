#[derive(Debug, PartialEq)]
pub struct NetworkInterface {
    pub ip: String,
    pub port: u16,
    is_active: bool,
}

impl NetworkInterface {
    pub fn new(ip: &str, port: u16) -> NetworkInterface {
        if port < 1 || port > 65535 {
            panic!("Port must be between 1 and 65535");
        }

        NetworkInterface {
            ip: ip.to_string(),
            port,
            is_active: false,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn is_secure(&self) -> bool {
        self.port == 443 || self.port == 22
    }

    fn validate_ip_format(&self) -> bool {
        self.ip.contains('.') && self.ip.split('.').count() == 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_creation() {
        let interface = NetworkInterface::new("192.168.1.1", 8080);
        assert_eq!(interface.ip, "192.168.1.1");
        assert_ne!(interface.port, 80);
    }

    #[test]
    fn test_security_check() {
        let secure_interface = NetworkInterface::new("10.0.0.1", 443);
        let insecure_interface = NetworkInterface::new("10.0.0.1", 80);

        assert!(secure_interface.is_secure());

        // Custom failure message
        assert!(
            !insecure_interface.is_secure(),
            "Expected port 80 to be insecure, but it wasn't"
        );
    }

    #[test]
    fn test_private_validation() {
        let interface = NetworkInterface::new("127.0.0.1", 3000);
        assert!(interface.validate_ip_format());
    }

    #[test]
    #[should_panic(expected = "Port must be between 1 and 65535")]
    fn test_invalid_port_panic() {
        NetworkInterface::new("192.168.1.1", 0);
    }

    #[test]
    #[ignore]
    fn test_expensive_network_call() {
        // Simulating a slow operation
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(true);
    }

    #[test]
    fn test_activation_status() -> Result<(), String> { // New: استخدام Result يسمح باستخدام ؟
        let mut interface = NetworkInterface::new("10.0.0.5", 22);

        if interface.is_active {
            return Err(String::from("Interface should be inactive initially"));
        }

        interface.activate();

        if !interface.is_active {
            return Err(String::from("Interface failed to activate"));
        }

        Ok(())
    }
}

fn main() {}