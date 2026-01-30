
//scenario: im building a tool to map a network path. we have both ipv4 and ipv6 and both diffrent size so why using BOX?
//the problem in the ENUM has a NextHop and the NetworkRoute has one and like that until infinty
// the box is the help here to just hold a pointer in the head not a huge place

#[derive(Debug)]
enum NetworkRoute{
    Destination,
    Hop(String, Box<NetworkRoute>),
}
fn main() {
    let path = NetworkRoute::Hop(String::from("192.168.702.1"),
        Box::new(NetworkRoute::Hop(
            String::from("10.0.0.5"),
            Box::new(NetworkRoute::Destination),
        )),
    );
    println!("The Path is: {:?}", path);
}
