use std::net::IpAddr;

use clap::Parser;
use vdns_lib::common::rr_type::RRType;

/// Program to perform DNS lookups
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// Override the default nameserver with the provided nameserver, e.g. 8.8.8.8
    #[arg(long, short)]
    pub nameserver: Option<IpAddr>,

    /// Which type of resource record to query for, defaults to A records
    #[arg(long, short = 't')]
    pub record_type: Option<RRType>,

    /// Sets the recurse desired flag to true for the query
    #[arg(long, short = 'r')]
    pub recurse: bool,

    /// The address to lookup
    #[arg()]
    pub address: String,
}
