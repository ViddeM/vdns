use std::net::IpAddr;
use std::path::Path;
use std::{fs, io};

#[derive(Debug, thiserror::Error)]
pub enum ResolvConfErr {
    #[error("IO error")]
    IOError(#[from] io::Error),
}

const DEFAULT_FILE_PATH: &str = "/etc/resolv.conf";

pub fn read_nameserver() -> Result<Option<IpAddr>, ResolvConfErr> {
    let path = Path::new(DEFAULT_FILE_PATH);
    if path.exists() && path.is_file() {}
    let nameservers = fs::read_to_string(path)?;
    println!("Nameservers:\n{nameservers}");

    Ok(None)
}
