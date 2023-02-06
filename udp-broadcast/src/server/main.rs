//! UDP server

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // I want a dynamic port from the stack, that's why the port :0 is used
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind IP address");
    // When enable, this socket is allow to send packets to a broadcast address
    socket
        .set_broadcast(true)
        .expect("set_broadcast call failed");

    let mut buf = [0; 5];
    // This is the `Broadcast Address`, any port number can be used here, but the udp clients must
    // match this port number if they want to receive the packets sent by us.
    socket
        .send_to(&buf, "255.255.255.255:6000")
        .expect("couldn't send message");

    let socket_address = socket.local_addr().unwrap();

    println!("waiting for messages");
    while let Ok((number_of_bytes, src_address)) = socket.recv_from(&mut buf) {
        let buf = &mut buf[..number_of_bytes];
        println!("{}: {} sent a message: {:?}", socket_address, src_address, buf);
    }

    Ok(())
}
