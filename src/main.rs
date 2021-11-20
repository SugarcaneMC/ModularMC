mod var_int;

use std::env::var;
use bytes::{Buf, BytesMut};
use futures::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Decoder;
use crate::var_int::{get_var_int, VarIntError};

struct PacketDecoder;

impl Decoder for PacketDecoder {
    type Item = i32;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("{}", src.len());
        let mut iter = src.iter();

        let (var_int_length, length_of_length) = match get_var_int::<i32, _>(&mut iter) {
            Ok((v, var_int_length)) => {
                (v, var_int_length)
            }, Err(e) => {
                return match e {
                    VarIntError::MissingExpectedByte => { Ok(None) },
                    VarIntError::TooManyBytes { length: var_int_length } =>
                        {
                            Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                format!("VarInt of length {} is too large.", var_int_length.max_length())
                            ))
                        },
                }
            }
        };


        todo!("get packet type and then return data array");

        println!("{}", packet_type);

        Ok(Some(var_int_length)) // retunr len for now
    }
}



async fn process(stream: TcpStream) -> Result<i32, std::io::Error> {
    let value = tokio_util::codec::Framed::new(stream, PacketDecoder).next().await.unwrap().unwrap();
    Ok(value)
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        println!("Has connection");
        tokio::spawn(async move {
            let v = process(stream)
                .await
                .unwrap();

            println!("{:?}", v)
        }).await.unwrap();
    }
}


