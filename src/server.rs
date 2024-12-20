

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web::web;
use actix_web::web::Bytes;
use actix_web_actors::ws;
use actix_web_actors::ws::ProtocolError;
use crate::AppState;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct MyWebSocket {
    hb: Instant,
    db:  web::Data<AppState>,
}

impl MyWebSocket {
    pub fn new(db: web::Data<AppState>) -> Self {
        Self { hb: Instant::now(), db }
    }

    // This function will run on an interval, every 5 seconds to check
    // that the connection is still alive. If it's been more than
    // 10 seconds since the last ping, we'll close the connection.
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    // Start the heartbeat process for this connection
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}


// The `StreamHandler` trait is used to handle the messages that are sent over the socket.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {

    // The `handle()` function is where we'll determine the response
    // to the client's messages. So, for example, if we ping the client,
    // it should respond with a pong. These two messages are necessary
    // for the `hb()` function to maintain the connection status.
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ping/Pong will be used to make sure the connection is still alive
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                log::info!("Successfully receive heart beat message");
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                log::info!("Successfully receive pong beat message");
                self.hb = Instant::now();
            }
            // Text will echo any text received back to the client (for now)
            Ok(ws::Message::Text(text)) => {
                // 获取 Redis 连接并执行 SET 操作
                match self.db.redis.db.get_connection_with_timeout(Duration::from_secs(5)) {
                    Ok(mut conn) => {
                        let json = String::from_utf8_lossy(text.as_bytes()).to_string();

                        match redis::cmd("SET").arg("my_key").arg(json.clone()).query::<()>(&mut conn) {
                            Ok(_) => {
                                log::info!("Successfully set key in Redis{:?}",json);
                            }
                            Err(e) => {
                                log::error!("Failed to set key in Redis: {:?}", e);
                                ctx.text(format!("Error setting key in Redis: {:?}", e));
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to get Redis connection: {:?}", e);
                        ctx.text(format!("Error getting Redis connection: {:?}", e));
                        return;
                    }
                }

                // Echo the original message back to the client
                ctx.text(text);
            }

            // Close will close the socket
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}


#[cfg(test)]
mod test{

    use actix_codec::*;
    use actix_web::web::BytesMut;
    use actix_web_actors::ws::Message;
    use awc::ws::Codec;

    #[test]
    pub fn  test_encode_massage() {
        let message = "Hello, World!";
        let mut cdk =Codec::new();
    let mut  meg= BytesMut::new();
        cdk.encode(Message::Ping("".into()), &mut meg).unwrap();
      println!("ping {:?}", meg.to_vec());
    }

}
