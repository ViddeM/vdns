use std::net::{SocketAddr, UdpSocket};

use crate::{common::rr_type::RRType, messages::message::Message};

pub mod common;
pub mod messages;

const DNS_PORT: u16 = 53;
const SEND_FROM_PORT: u16 = 9315;

fn main() {
    let message = lookup("digit.chalmers.it", RRType::A);
    println!("Message: {message}");
}

fn lookup(name: &str, rr_type: RRType) -> Message {
    let message = Message::new_query(name, rr_type);

    let mut buffer = vec![];
    message.serialize(&mut buffer);

    // Send the message
    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], SEND_FROM_PORT)))
        .expect("Failed to bind to port");
    socket.set_read_timeout(None).unwrap();

    socket
        .connect(SocketAddr::from(([192, 168, 1, 1], DNS_PORT)))
        .expect("Failed to connect to remote host");

    socket
        .send(buffer.as_slice())
        .expect("Failed to send DNS message");

    // Listen for a response
    let mut buf = [0u8; 512];
    let size = socket.recv(&mut buf).expect("Failed to listen for a reply");
    let read = &buf[0..size];

    Message::parse(&read).expect("Failed to parse response")
}
