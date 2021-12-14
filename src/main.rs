use crate::messages::message::Message;

pub mod common;
pub mod messages;

fn main() {
    let buffer = vec![
        0xa8, 0x20, 0x81, 0x80, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00,
    ];

    let message = Message::parse(&mut buffer.as_slice()).unwrap();
    println!("Message: {}", message);
}
