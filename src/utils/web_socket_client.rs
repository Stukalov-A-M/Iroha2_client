use crate::utils::non_zero_handler::non_zero_handler;
use iroha_data_model::block::stream::BlockSubscriptionRequest;
use parity_scale_codec::Encode;
use std::error::Error;
use websocket::header::Headers;
use websocket::native_tls::TlsConnector;
use websocket::ws::dataframe::DataFrame;
use websocket::{ClientBuilder, Message, OwnedMessage};

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
    let msg = Message::binary(BlockSubscriptionRequest::new(non_zero_handler(1)).encode());

    let _request = client.send_message(&msg).unwrap();
    let _response = OwnedMessage::from(client.recv_message().unwrap()).take_payload();

    Ok(())
}

