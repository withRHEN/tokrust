use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use protobuf::Message;
use tokio::runtime::Runtime;
use tokio_tungstenite::tungstenite::connect;
use tokio_tungstenite::tungstenite::handshake::client::Request;

use crate::core::live_client::TikTokLiveClient;
use crate::core::live_client_mapper::TikTokLiveMessageMapper;
use crate::data::live_common::ConnectionState::CONNECTED;
use crate::generated::events::{TikTokConnectedEvent, TikTokLiveEvent};
use crate::http::http_data::LiveConnectionDataResponse;
use crate::generated::messages::webcast::{WebcastPushFrame, WebcastResponse};


pub struct TikTokLiveWebsocketClient
{
    pub(crate) message_mapper: TikTokLiveMessageMapper,
    pub(crate) running: Arc<AtomicBool>,
}

impl TikTokLiveWebsocketClient
{
    pub fn new(message_mapper: TikTokLiveMessageMapper) -> Self {
        TikTokLiveWebsocketClient {
            message_mapper,
            running: Arc::new(AtomicBool::new(false)),
        }
    }


    pub async fn start(&self, response: LiveConnectionDataResponse,  client: Arc<TikTokLiveClient>)
    {
        let host = response.web_socket_url.host_str().expect("Invalid host in WebSocket URL");

        let request = Request::builder()
            .method("GET")
            .uri(response.web_socket_url.to_string())
            .header("Host", host)
            .header("Upgrade", "websocket")
            .header("Connection", "upgrade")
            .header("Sec-Websocket-Key", "asd")
            .header("Cookie", response.web_socket_cookies)
            .header("Sec-Websocket-Version", "13")
            .body(())
            .unwrap();

        let (mut socket, _) = connect(request).expect("Failed to connect");

        client.set_connection_state(CONNECTED);
        client.publish_event(TikTokLiveEvent::OnConnected(TikTokConnectedEvent {}));


        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        let message_mapper = self.message_mapper.clone();

        thread::spawn(move || {
            while running.load(Ordering::SeqCst)
            {
                let optional_message = socket.read_message();

                if optional_message.is_err()
                {
                    continue;
                }
                let message = optional_message.unwrap();

                let buffer = message.into_data();

                let mut push_frame = WebcastPushFrame::parse_from_bytes(buffer.as_slice()).expect("Unable to read push frame");
                let webcast_response = WebcastResponse::parse_from_bytes(push_frame.Payload.as_mut_slice()).expect("Unable to read webcast response");

                if webcast_response.needsAck
                {
                    let mut push_frame_ack = WebcastPushFrame::new();
                    push_frame_ack.PayloadType = "ack".to_string();
                    push_frame_ack.LogId = push_frame.LogId;
                    push_frame_ack.Payload = webcast_response.internalExt.clone().into_bytes();

                    let binary = push_frame_ack.write_to_bytes().unwrap();
                    let message = tungstenite::protocol::Message::binary(binary);
                    socket.write_message(message).expect("Unable to send ack packet");
                }

                message_mapper.handle_webcast_response(webcast_response, client.as_ref());
            }
        });
    }

    pub fn stop(&self)
    {
        self.running.store(false, Ordering::SeqCst);
    }
}