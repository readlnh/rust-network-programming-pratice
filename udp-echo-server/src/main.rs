use std::thread;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("Could not bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Could not clone socket");
        match sock.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Handing connection from {}", src);
                    //sock.send_to(b"Hi ", src).expect("Failed to send a response");
                    sock.send_to(&buf, src).expect("Failed to send a response");
                });
            }
            Err(e) => {
                eprintln!("Could not receieve a datagram: {}", e);
            }
        }
    }    
}
