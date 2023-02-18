use std::collections::HashMap;

use actix::prelude::*;
use actix_web_actors::ws::{Message as WebSocketMessage, ProtocolError, WebsocketContext};
use log::error;

pub type AddrServer = Addr<Server>;

pub use actix_web_actors::ws::start;

pub struct Server {
    pending_devices: HashMap<i32, Addr<Connection>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            pending_devices: HashMap::new(),
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<messages::ConnectionRequest> for Server {
    type Result = <messages::ConnectionRequest as Message>::Result;

    fn handle(&mut self, msg: messages::ConnectionRequest, _: &mut Self::Context) -> Self::Result {
        match msg.kind {
            messages::ConnectionRequestKind::Add => {
                if self.pending_devices.insert(msg.pending_device_id, msg.conn).is_some() {
                    error!("A pending device is added twice into connection list");
                }
            }
            messages::ConnectionRequestKind::Remove => {
                if self.pending_devices.remove(&msg.pending_device_id).is_none() {
                    error!("An unknown pending device is removed from connection list");
                }
            }
        }
    }
}

impl Handler<messages::AcceptPendingDevice> for Server {
    type Result = <messages::AcceptPendingDevice as Message>::Result;

    fn handle(&mut self, msg: messages::AcceptPendingDevice, _: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.pending_devices.get(&msg.0) {
            addr.do_send(msg);

            true
        } else {
            false
        }
    }
}

#[derive(MessageResponse)]
pub struct Connection {
    server: Addr<Server>,
    pending_device_id: i32,
}

impl Connection {
    pub fn new(server: Addr<Server>, pending_device_id: i32) -> Self {
        Connection { server, pending_device_id }
    }
}

impl Actor for Connection {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server
            .send(messages::ConnectionRequest {
                conn: ctx.address(),
                pending_device_id: self.pending_device_id,
                kind: messages::ConnectionRequestKind::Add,
            })
            .into_actor(self)
            .then(|_, _, _| fut::ready(()))
            .wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.server.do_send(messages::ConnectionRequest {
            conn: ctx.address(),
            pending_device_id: self.pending_device_id,
            kind: messages::ConnectionRequestKind::Remove,
        });

        Running::Stop
    }
}

impl StreamHandler<Result<WebSocketMessage, ProtocolError>> for Connection {
    fn handle(&mut self, msg: Result<WebSocketMessage, ProtocolError>, ctx: &mut Self::Context) {
        let msg = if let Ok(msg) = msg { msg } else { return };

        match msg {
            WebSocketMessage::Ping(msg) => {
                ctx.pong(&msg);
            }
            WebSocketMessage::Pong(_) => { },
            WebSocketMessage::Text(text) => {
                ctx.text(text);
            }
            WebSocketMessage::Close(_) => ctx.stop(),
            _ => {}
        }
    }
}

impl Handler<messages::AcceptPendingDevice> for Connection {
    type Result = <messages::AcceptPendingDevice as Message>::Result;

    fn handle(&mut self, _: messages::AcceptPendingDevice, ctx: &mut Self::Context) -> Self::Result {
        ctx.text("accepted");

        true
    }
}

pub mod messages {
    use actix::prelude::{Addr, Message};

    use super::Connection;

    pub enum ConnectionRequestKind {
        Add,
        Remove,
    }

    pub struct ConnectionRequest {
        pub conn: Addr<Connection>,
        pub pending_device_id: i32,
        pub kind: ConnectionRequestKind,
    }

    impl Message for ConnectionRequest {
        type Result = ();
    }

    pub struct AcceptPendingDevice(pub i32);

    impl Message for AcceptPendingDevice {
        type Result = bool;
    }
}
