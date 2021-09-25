use std::net;
use std::thread;
fn main() {
    let boss_thread = thread::spawn(|| {
        let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let worker = thread::spawn(move || {
                let stream = stream; // move stream into this func
            }).join().unwrap();
        }

    }).join().unwrap();
}