use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;
use std::{fs, io};

#[derive(Debug, thiserror::Error)]
pub enum ResolvConfErr {
    #[error("IO error")]
    IOError(#[from] io::Error),
}

const DEFAULT_FILE_PATH: &str = "/etc/resolv.conf";

pub fn read_nameserver() -> Result<Vec<IpAddr>, ResolvConfErr> {
    let path = Path::new(DEFAULT_FILE_PATH);
    if path.exists() && path.is_file() {}
    let nameservers = fs::read_to_string(path)?;
    let nameservers = nameservers
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .filter(|(first, _)| *first == "nameserver")
        .map(|(_, addr)| {
            IpAddr::from_str(addr).expect("Failed to parse nameserver address as IPv4 adress")
        })
        .collect();

    Ok(nameservers)
}
