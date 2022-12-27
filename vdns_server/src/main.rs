use std::net::{IpAddr, SocketAddr, UdpSocket};

use mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use vdns_lib::{
    lookup,
    messages::{
        header::flags::RCode, message::Message, resource_record::resource_record::ResourceRecord,
    },
    DNS_PORT,
};

pub mod cache;

const DNS_MAX_PACKAGE_SIZE: usize = 512;

#[tokio::main]
pub async fn main() {
    // Setup Redis cache
    let redis_client =
        redis::Client::open("redis://localhost:6379").expect("Failed to connect to redis");
    let redis_manager = RedisConnectionManager::new(redis_client);
    let redis_pool = Pool::builder().build(redis_manager);

    let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], DNS_PORT)))
        .expect("Failed to bind to UDP port ");
    let router_address = IpAddr::from([192, 168, 1, 1]);
    println!("VDNS server started and listening on port {DNS_PORT}");

    loop {
        let mut receive_buffer = [0; DNS_MAX_PACKAGE_SIZE];
        let (bytes_received, remote_addr) = socket
            .recv_from(&mut receive_buffer)
            .expect("Failed to receive UDP message");

        let message = Message::parse(&receive_buffer[0..bytes_received])
            .expect("Failed to parse DNS message");

        println!("Request received for {}", message.to_short_string());

        if message.is_query() {
            let response = if message.do_recursion() {
                let responses = get_answers(&redis_pool, &message, &router_address).await;

                Message::new_response(&message, responses)
            } else {
                // Get it from the cache
                todo!("Only recursion is available at the moment");
            };

            println!("\t- Responding with {} answers", response.answer.len());
            cache::cache_response(&redis_pool, &response).await;

            // Send the response
            let serialized = response.serialize();

            // Try to parse it to ensure that it looks alright
            Message::parse(&serialized).expect("Failed to parse message to send!!!");

            socket
                .send_to(&serialized, remote_addr)
                .expect("Failed to send response!");
        } else {
            println!("Received non-query request? \n======\n{message}\n======\n");
        }
    }
}

async fn get_answers(
    redis_pool: &Pool<RedisConnectionManager>,
    message: &Message,
    router_address: &IpAddr,
) -> Vec<ResourceRecord> {
    let mut records = vec![];
    for (name, rr_type) in message.question_names().iter() {
        if let Some(record) = cache::lookup_cached(&redis_pool, &name, &rr_type).await {
            println!("\tUsing cached value for {name} {rr_type}");
            records.push(record);
        } else {
            let resp = lookup(
                &name.to_string(),
                rr_type.clone(),
                router_address.clone(),
                true,
            );
            if resp.header.flags.r_code == RCode::NoError {
                for ans in resp.answer.into_iter() {
                    records.push(ans);
                }
            } else {
                println!(
                    "Got error response from remote DNS server: \n======\n{message}\n======\n"
                );
            }
        }
    }

    records
}
