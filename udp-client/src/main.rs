use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // Listening address, the listening port should be equal to the broadcast port.
    let socket = UdpSocket::bind("0.0.0.0:6000").expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");

    let mut buf = [5; 10];
    let (number_of_bytes, src_address) = socket.recv_from(&mut buf).expect("couldn't receive message");
    let buf = &mut buf[..number_of_bytes];
    print!("{:#?}", buf);

    for _ in 0..20 {
        socket.send_to(&[0; 10], &src_address).expect("couldn't send message");
    }
    Ok(())
}
