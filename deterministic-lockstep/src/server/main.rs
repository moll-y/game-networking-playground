use std::{
    io::{self, Write},
    net::UdpSocket,
    thread, time,
};

fn main() {
    // The simulation on the right can only simulate frame n
    // when it has the input for that frame
    let socket = UdpSocket::bind("0.0.0.0:7000").expect("couldn't bind address");
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

    let mut sequence = 1;
    loop {
        for (frame_number, frame) in frames.iter().enumerate() {
            print!(
                "\tsequence ({}) frame number ({}) frame [{}]\r",
                sequence, frame_number, frame
            );

            socket
                .send_to(
                    &[sequence, frame_number.try_into().unwrap()],
                    "0.0.0.0:6000",
                )
                .expect("couldn't send message");

            sequence += 1;
            io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}
