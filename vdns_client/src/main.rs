use clap::Parser;
use vdns_lib::{
    common::{resolvconf::read_nameserver, rr_type::RRType},
    lookup,
};

use crate::cli::CLI;

pub mod cli;

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
