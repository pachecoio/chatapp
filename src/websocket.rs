use actix::{Actor, StreamHandler};
use actix_web::http::Error;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

struct WebSocket;

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let resp = ws::start(WebSocket {}, &req, stream);
    println!("{:?}", resp);
    resp
}
