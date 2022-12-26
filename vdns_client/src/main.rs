use clap::Parser;
use vdns_lib::{
    common::{resolvconf::read_nameserver, rr_type::RRType},
    lookup,
};

use crate::cli::CLI;

pub mod cli;

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
