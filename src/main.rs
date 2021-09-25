use std::net;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080");
}