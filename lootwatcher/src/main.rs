use futures::prelude::*;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};
use std::thread::sleep;
use std::time::Duration;
use futures::executor::block_on;


#[tokio::main]
pub async fn main() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());
    let mut serialized =
        tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());

    serialized
        .send(json!(
    {
        "action": "update"
    }
    ))
        .await
        .unwrap();

}