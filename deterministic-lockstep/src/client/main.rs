use std::{
    io::{self, Write},
    net::UdpSocket,
    thread, time,
};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:6000").expect("couldn't bind address");
    let frames = [
        "X         ",
        " X        ",
        "  X       ",
        "   X      ",
        "    X     ",
        "     X    ",
        "      X   ",
        "       X  ",
        "        X ",
        "         X",
    ];

    loop {
        let mut buf = [0; 2];
        let number_of_bytes = match socket.recv_from(&mut buf) {
            Ok((number_of_bytes, _)) => number_of_bytes,
            Err(_) => continue,
        };
        let buf = &buf[..number_of_bytes];
        let server_sequence = buf[0];
        let frame_number = buf[1];

        print!(
            "\tsequence ({}) frame number ({}) frame [{}]\r",
            server_sequence, frame_number, frames[frame_number as usize]
        );
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}
