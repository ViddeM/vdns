use std::net::{IpAddr, SocketAddr, UdpSocket};

use clap::Parser;
use common::resolvconf::read_nameserver;

use crate::{cli::CLI, common::rr_type::RRType, messages::message::Message};

pub mod cli;
pub mod common;
pub mod messages;

const DNS_PORT: u16 = 53;
const SEND_FROM_PORT: u16 = 9315;

fn main() {
    let args = CLI::parse();

    let nameserver = args.nameserver.unwrap_or(
        *read_nameserver()
            .expect("Failed to read nameservers file!")
            .first()
            .unwrap(),
    );

    let rr_type = args.record_type.unwrap_or(RRType::A);

    let message = lookup(&args.address, rr_type, nameserver, args.recurse);
    println!("Message: {message}");
}

fn lookup(name: &str, rr_type: RRType, nameserver: IpAddr, recurse: bool) -> Message {
    let message = Message::new_query(name, rr_type, recurse);

    let mut buffer = vec![];
    message.serialize(&mut buffer);

    // Send the message
    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], SEND_FROM_PORT)))
        .expect("Failed to bind to port");
    socket.set_read_timeout(None).unwrap();

    socket
        .connect(SocketAddr::from((nameserver, DNS_PORT)))
        .expect("Failed to connect to nameserver");

    socket
        .send(buffer.as_slice())
        .expect("Failed to send DNS message");

    // Listen for a response
    let mut buf = [0u8; 512];
    let size = socket.recv(&mut buf).expect("Failed to listen for a reply");
    let read = &buf[0..size];

    Message::parse(&read).expect("Failed to parse response")
}
