use std::{str, io};
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Could not bind socket");
    socket.connect("127.0.0.1:8080").expect("Could not connect to server");
    
    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];
        io::stdin().read_line(&mut input).expect("Could not read from stdin");
        socket.send(input.as_bytes()).expect("Failed to write to server");
        socket.recv(&mut buffer).expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}
