use std::{error::Error, io::Write, net::TcpStream};

fn main() -> Result<(), Box<dyn Error>> {
    let host = "www.rustinaction.com:80";
    let mut conn = TcpStream::connect(host)?;
    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n\r\n")?;
    conn.write_all(b"Host:www.rustinaction.com")?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;

    Ok(())
}
