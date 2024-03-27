use std::net::{Shutdown, TcpListener, TcpStream};

// use tokio::stream;

fn main() {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("Connected");
        drop(stream);
    } else {
        println!("Failed to connect ")
    }
}

fn _shutdown_stream(stream: TcpStream) {
    stream.shutdown(Shutdown::Both).expect("Shutdown failed");
    drop(stream);
}
