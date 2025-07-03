use std::io::*;
use std::net::TcpListener;

fn main() {
    let lis = TcpListener::bind("127.0.0.1:3000").unwrap();
    loop {
        let (mut strm, _) = lis.accept().unwrap();
        strm.write(b"Hello");
    }
}
