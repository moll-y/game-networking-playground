use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // Listening address
    let socket = UdpSocket::bind("0.0.0.0:7000").expect("couldn't bind to listening address: address zero");
    socket.set_broadcast(true).expect("set_broadcast call failed");

    let mut buf = [0; 10];
    // Broadcast address, any port can be used
    socket.send_to(&buf, "255.255.255.255:6000").expect("couldn't send message");

    println!("Awaiting responses...");   // self.recv_buff is a [u8; 8092]
    while let Ok((n, addr)) = socket.recv_from(&mut buf) {
        println!("{} bytes response from {:?}", n, addr);
    }
    Ok(())
}
