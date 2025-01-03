use core::{error, fmt};
use std::{
    fs::File,
    io,
    net::{AddrParseError, Ipv6Addr},
};

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError {}

impl From<io::Error> for UpstreamError {
    fn from(value: io::Error) -> Self {
        UpstreamError::IO(value)
    }
}

impl From<AddrParseError> for UpstreamError {
    fn from(value: AddrParseError) -> Self {
        UpstreamError::Parsing(value)
    }
}

fn main() -> Result<(), UpstreamError> {
    let f = File::open("invisible.txt")?;
    let a = "::1".parse::<Ipv6Addr>()?;

    // let f = File::open("invisible.txt").map_err(UpstreamError::IO)?;
    // let a = "::1".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing)?;

    Ok(())
}
