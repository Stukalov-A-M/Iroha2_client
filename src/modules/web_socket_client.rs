use std::error::Error;
use iroha_data_model::block::stream::{BlockMessage, BlockSubscriptionRequest};
use parity_scale_codec::{DecodeAll, Encode};
use websocket::{ClientBuilder, Message, OwnedMessage};
use websocket::header::{Headers};
use websocket::native_tls::TlsConnector;
use websocket::ws::dataframe::DataFrame;
use crate::utils::non_zero_handler::non_zero_handler;


pub fn socket_init() -> Result<(), Box<dyn Error>> {
    let buf = b"Connection: Upgrade\n\
    Upgrade: websocket\n\
    Sec-WebSocket-Version: 13\n\
    Sec-WebSocket-Extensions: permessage-deflate; client_max_window_bits\n\
    Authorization: Basic VlFhUDJyUVNrbVhxT0Y5Z2RyTGdrdEJvWmI6TGVpV2xqTHpVczR2eFhwNXJQY3JQbTN3RE8=\n";
    let mut headers = [httparse::EMPTY_HEADER; 5];
    httparse::parse_headers(buf, &mut headers[..]).expect("");

    let headers = Headers::from_raw(&headers).unwrap();
    let mut client = ClientBuilder::new("https://s1.tst.iroha2.iroha.tech/block/stream")
        .unwrap()
        .custom_headers(&headers)
        .connect(TlsConnector::new().ok())
        .unwrap();
    let msg = Message::binary(BlockSubscriptionRequest::new(non_zero_handler(1)).encode()) ;

    let _request = client.send_message(&msg).unwrap();
    let mut response = OwnedMessage::from(client.recv_message().unwrap()).take_payload();
    let rsp_msg = BlockMessage::decode_all(&mut response.as_slice()).unwrap();

    println!("{:?}", rsp_msg);

    Ok(())
}

/*
fn config_client () -> Box<[dyn Header<'static>]> {



    Box::from(headers)
}

 */


