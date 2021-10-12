use std::net;
use std::thread;
use std::io::Read;

fn main() {
    let boss_thread = thread::spawn(|| {
        let listener = net::TcpListener::bind("127.0.0.1:2556").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let worker = thread::spawn(move || {
                let mut stream = stream; // move stream into this func

                let mut buf = [0; 2048];

                stream.read(&mut buf);

                for thing in buf {
                    println!("{:08b}", thing);
                }
                // the magic that can decipher a VarInt
                let a = decode_var_int_i32(&[buf[2],buf[3],buf[4],buf[5],buf[6]]).unwrap();
                //
                println!("final {}", a);

                let string = String::from_utf8_lossy(&mut buf);

                println!("{:?}", buf);
            }).join().unwrap();
        }

    }).join().unwrap();
}

fn decode_var_int_i32(buf: &[u8;5]) -> Result<i32, ()> {
    println!("DECODING {:?}", buf);
    let mut value = 0;
    for n in 0..5 {
        let byte = buf[n];
        value |= ((byte & 0b01111111) as i32) << 7 * n;
        if buf[n] & 0b10000000 == 0 {
            break;
        }
        println!("{}",value);
    }
    Ok(value)
}