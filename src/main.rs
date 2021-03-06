use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    let s = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{}", s);

    stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
    stream.write_all(b"\r\n")?;
    stream.write_all(b"Hello World!")?;
    stream.flush()?;

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
