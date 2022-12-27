use std::net::{IpAddr, SocketAddr, UdpSocket};

use vdns_lib::{
    lookup,
    messages::{header::flags::RCode, message::Message},
    DNS_PORT,
};

const DNS_MAX_PACKAGE_SIZE: usize = 512;

pub fn main() {
    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], DNS_PORT)))
        .expect("Failed to bind to UDP port ");
    let router_address = IpAddr::from([192, 168, 1, 1]);
    println!("VDNS server started and listening on port {DNS_PORT}");

    loop {
        let mut receive_buffer = [0; DNS_MAX_PACKAGE_SIZE];
        let (bytes_received, remote_addr) = socket
            .recv_from(&mut receive_buffer)
            .expect("Failed to receive UDP message");

        println!("Request received from {remote_addr}");

        let message = Message::parse(&receive_buffer[0..bytes_received])
            .expect("Failed to parse DNS message");

        println!("\n\nMessage parsed as: \n======\n{message}\n======\n");

        if message.is_query() {
            let response = if message.do_recursion() {
                let responses = message
                    .question_names()
                    .into_iter()
                    .map(|(name, rr_type)| lookup(&name.to_string(), rr_type, router_address, true))
                    .collect::<Vec<Message>>();

                println!("Got {} responses", responses.len());

                Message::new_response(
                    &message,
                    responses.into_iter().map(|r| {
                        if r.header.flags.r_code == RCode::NoError {
                            r.answer
                        } else {
                            println!("Got error response from remote DNS server: \n======\n{r}\n======\n");
                            vec![]
                        }
                    }).flatten().collect(),
                )
            } else {
                // Local cache not implemented yet :(
                todo!("Only recursion is available at the moment");
            };

            println!("Responding with \n======\n{response}\n======\n");

            // Send the response
            let serialized = response.serialize();

            // Try to parse it to ensure that it looks alright
            // Message::parse(&serialized).expect("Failed to parse message to send!!!");

            socket
                .send_to(&serialized, remote_addr)
                .expect("Failed to send response!");
        } else {
            println!("Received non-query request? \n======\n{message}\n======\n");
        }
    }
}
