use std::{collections::HashMap, time::Duration};

use actix::prelude::*;
use actix_web_actors::ws::{Message as WebSocketMessage, ProtocolError, WebsocketContext};
use log::debug;

pub type AddrServer = Addr<Server>;

pub use actix_web_actors::ws::start;

pub struct Server {
    pending_devices: HashMap<(i32, i32), Addr<Connection>>,
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

impl Handler<messages::AddConnection> for Server {
    type Result = <messages::AddConnection as Message>::Result;

    fn handle(&mut self, msg: messages::AddConnection, _: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.pending_devices.insert((msg.user_id, msg.device_id), msg.conn) {
            debug!("A pending device is added twice into connection list");
            addr.do_send(messages::StopConnection);
        }
    }
}

impl Handler<messages::RemoveConnection> for Server {
    type Result = <messages::RemoveConnection as Message>::Result;

    fn handle(&mut self, msg: messages::RemoveConnection, _: &mut Self::Context) -> Self::Result {
        if self.pending_devices.remove(&(msg.user_id, msg.device_id)).is_none() {
            debug!("An unknown pending device is removed from connection list");
        }
    }
}

impl Handler<messages::AcceptPendingDevice> for Server {
    type Result = <messages::AcceptPendingDevice as Message>::Result;

    fn handle(&mut self, msg: messages::AcceptPendingDevice, _: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.pending_devices.get(&(msg.user_id, msg.device_id)) {
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
    user_id: i32,
    device_id: i32,
}

impl Connection {
    pub fn new(server: Addr<Server>, user_id: i32, device_id: i32) -> Self {
        Connection { server, user_id, device_id }
    }
}

impl Actor for Connection {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.server
            .send(messages::AddConnection {
                conn: ctx.address(),
                user_id: self.user_id,
                device_id: self.device_id,
            })
            .into_actor(self)
            .then(|_, _, _| fut::ready(()))
            .wait(ctx);

        // Close connection after 5 minutes.
        ctx.run_later(Duration::from_secs(60 * 5), |_, ctx| {
            ctx.text("timeout");
            ctx.stop()
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server.do_send(messages::RemoveConnection {
            user_id: self.user_id,
            device_id: self.device_id,
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

impl Handler<messages::StopConnection> for Connection {
    type Result = <messages::StopConnection as Message>::Result;

    fn handle(&mut self, _: messages::StopConnection, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

pub mod messages {
    use actix::prelude::{Addr, Message};

    use super::Connection;

    pub struct StopConnection;

    impl Message for StopConnection {
        type Result = ();
    }

    pub struct RemoveConnection {
        pub user_id: i32,
        pub device_id: i32,
    }

    impl Message for RemoveConnection {
        type Result = ();
    }

    pub struct AddConnection {
        pub conn: Addr<Connection>,
        pub user_id: i32,
        pub device_id: i32,
    }

    impl Message for AddConnection {
        type Result = ();
    }

    pub struct AcceptPendingDevice {
        pub user_id: i32,
        pub device_id: i32,
    }

    impl Message for AcceptPendingDevice {
        type Result = bool;
    }
}
