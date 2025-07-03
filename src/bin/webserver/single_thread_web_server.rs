use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut strm: TcpStream) -> Result<()> {
    let res = concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Length: 12\n",
        "Connection: close\r\n\r\n",
        "Hello world!"
    );
    let mut written = 0;

    loop {
        let num_bytes = strm.write(res[written..].as_bytes())?;
        if num_bytes == 0 {
            println!("client disconnected unexpectedly");
            return Ok(());
        }
        written += num_bytes;
        if written == res.len() {
            break;
        }
    }

    strm.flush()
}

fn main() {
    let lis = TcpListener::bind("127.0.0.1:3000").unwrap();
    loop {
        let (strm, _) = lis.accept().unwrap();
        if let Err(e) = handle_connection(strm) {
            println!("Error {e}");
        }
    }
}
