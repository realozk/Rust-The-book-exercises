#[derive(Debug, Clone )]
struct Color(i32, i32, i32);

#[derive(Debug, Clone )]
struct Server {
    name: String,
    ip: String,
    port: u16,
    active: bool,
}

impl Server {
    fn new(name: String, ip: String) -> Server {
        Server {
            name,
            ip,
            port: 8080,
            active: true,
        }
    }

    fn get_status(&self) -> String {
        format!("Server {} is running on port {}", self.name, self.port)
    }

    fn set_offline(&mut self) {
        self.active = false;
    }
}

fn main() {
    let status_led = Color(0, 255, 0);

    let name = String::from("Auth-Server");
    let ip = String::from("10.0.0.1");

    let mut server1 = Server::new(name, ip);

    server1.set_offline();

    let server2 = Server {
        name: String::from("Data-Server"),
        ..server1.clone()
    };

    println!("LED Status: {:?}", status_led);
    println!("Server 1: {:#?}", server1);
    println!("Status Msg: {}", server1.get_status());
    println!("Server 2 (Copied): {:?}", server2);
}