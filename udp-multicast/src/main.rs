use std::{env, str};
use std::net::{UdpSocket, Ipv4Addr};

fn main() {
    let mcast_group: Ipv4Addr = "239.0.0.1".parse().unwrap();
    let port: u16 = 6000;
    let any: Ipv4Addr = "0.0.0.0".parse().unwrap();
    let mut buffer = [0u8; 1600];
    if env::args().count() > 1 {
        // client case
        let socket = UdpSocket::bind((any, port)).expect("Could not bind client socket");
        socket.join_multicast_v4(&mcast_group, &any).expect("Could not join multicast group");
        socket.recv(&mut buffer).expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("Could not write as string"));
    } else {
        // server case
        let socket = UdpSocket::bind((any, 0)).expect("Could not bind socket");
        socket.send_to("Hello World!\n".as_bytes(), &(mcast_group, port)).expect("Failed to write data");
    }   
}
