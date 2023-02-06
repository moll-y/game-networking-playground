//! UDP client

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // The listener port (clients) and the broadcaster port (server) must be the same
    let socket = UdpSocket::bind("0.0.0.0:6000").expect("couldn't bind IP address");

    let mut buf = [5; 5];
    println!("waiting for a message");
    let (number_of_bytes, src_address) = socket.recv_from(&mut buf).expect("didn't receive data");
    let buf = &mut buf[..number_of_bytes];

    let socket_address = socket.local_addr().unwrap();
    println!("{}: {} sent a message: {:?}", socket_address, src_address, buf);

    // Send ten packets to the server
    for x in 1..11 {
        socket
            .send_to(&[x; 5], &src_address)
            .expect("couldn't send message");
    }

    Ok(())
}
