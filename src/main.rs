use std::net::{SocketAddr, UdpSocket};

use crate::messages::message::Message;

pub mod common;
pub mod messages;

const DNS_PORT: u16 = 53;
const SEND_FROM_PORT: u16 = 9315;

fn main() {
    let input_buffer = vec![
        0x87, 0x2a, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x76, 0x69,
        0x64, 0x61, 0x72, 0x6d, 0x61, 0x67, 0x6e, 0x75, 0x73, 0x73, 0x6f, 0x6e, 0x03, 0x63, 0x6f,
        0x6d, 0x00, 0x00, 0x01, 0x00, 0x01,
    ];

    let message = Message::parse(&mut input_buffer.as_slice()).unwrap();

    println!("Message: {message}");

    let mut buffer = vec![];
    message.serialize(&mut buffer);

    println!("Buf {:?}", buffer.as_slice());

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
    let mut read = &buf[0..size];

    println!("Buf {read:?}");

    let parsed_response = Message::parse(&mut read).unwrap();

    println!("Response {parsed_response}");

    // let  message = DNSResponse::parse(&mut buf).unwrap();
}
