mod ch11_test;

#[derive(Debug, Clone)]
struct Packet {
    id: u32,
    size_bytes: u32,
    is_encrypted: bool,
}

// act like stream of network traffic.
struct PacketStream {
    current_id: u32,
    max_packets: u32,
}

impl PacketStream {
    fn new(count: u32) -> PacketStream {
        PacketStream {
            current_id: 0,
            max_packets: count,
        }
    }
}


impl Iterator for PacketStream {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id < self.max_packets {
            self.current_id += 1;

            // Mocking different packet characteristics based on ID.
            Some(Packet {
                id: self.current_id,
                size_bytes: self.current_id * 100,
                is_encrypted: self.current_id % 2 == 0,
            })
        } else {
            None
        }
    }
}

// F: The specific closure type.
struct Firewall<F>
where
    F: Fn(&Packet) -> bool,
{
    filter_rule: F,
}

impl<F> Firewall<F>
where
    F: Fn(&Packet) -> bool,
{
    fn check_packet(&self, packet: &Packet) -> bool {
        (self.filter_rule)(packet)
    }
}

fn main() {
    // modfi variables (Simulation environment).
    let max_allowed_size = 400;
    let critical_keyword = String::from("SUSPICIOUS");

    let complex_filter = move |p: &Packet| {
        p.size_bytes > max_allowed_size || (!p.is_encrypted && critical_keyword == "SUSPICIOUS")
    };

    let firewall = Firewall {
        filter_rule: complex_filter,
    };

    // Initialize the traffic source.
    let stream = PacketStream::new(10);

    println!("--- Analyzing Traffic Stream ---");
//the opretion being done
    // 1. Consume stream.
    // 2. Filter based on firewall rules.
    // 3. Log and extract ID.
    let flagged_packets: Vec<u32> = stream
        .into_iter()
        .enumerate() // Adds index tracking.
        .filter(|(_, packet)| firewall.check_packet(packet))
        .map(|(idx, packet)| {
            println!("Alert! Packet #{} [ID: {}] flagged.", idx, packet.id);
            packet.id
        })
        .collect();

    println!("\nTotal Flagged Packets: {:?}", flagged_packets);

    // Correlate packet IDs with priority levels using zip.
    let priorities = vec![1, 5, 2];

    let report: Vec<_> = flagged_packets.iter().zip(priorities.iter()).collect();

    println!("Priority Report (ID, Priority): {:?}", report);
}