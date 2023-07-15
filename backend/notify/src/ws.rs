use std::{collections::HashMap, time::Duration};

use actix::prelude::*;
use actix_web_actors::ws::{Message as WebSocketMessage, ProtocolError, WebsocketContext};
use log::debug;

pub type AddrServer = Addr<Server>;

pub use actix_web_actors::ws::start;

pub struct Server {
    users: HashMap<i32, HashMap<i32, Addr<Connection>>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            users: HashMap::new(),
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<internal::AddConnection> for Server {
    type Result = <internal::AddConnection as Message>::Result;

    fn handle(&mut self, msg: internal::AddConnection, _: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.users.entry(msg.user_id).or_insert(HashMap::new()).insert(msg.device_id, msg.conn) {
            debug!("A device is added twice into connection list");
            addr.do_send(internal::StopConnection);
        }
    }
}

impl Handler<internal::RemoveConnection> for Server {
    type Result = <internal::RemoveConnection as Message>::Result;

    fn handle(&mut self, msg: internal::RemoveConnection, _: &mut Self::Context) -> Self::Result {
        if let Some(devices) = self.users.get_mut(&msg.user_id) {
            if devices.remove(&msg.device_id).is_none() {
                debug!("An unknown device is removed from connection list");
            }

            if devices.len() == 0 {
                self.users.remove(&msg.user_id);
            }
        } else {
            debug!("A device from unknown user is removed");
        }
    }
}

impl Handler<messages::SendDeviceMessage> for Server {
    type Result = <messages::SendDeviceMessage as Message>::Result;

    fn handle(&mut self, msg: messages::SendDeviceMessage, _: &mut Self::Context) -> Self::Result {
        if let Some(Some(addr)) = self.users.get(&msg.user_id).map(|devices| devices.get(&msg.device_id)) {
            addr.do_send(msg.message);
        }
    }
}

impl Handler<messages::SendExclusiveDeviceMessage> for Server {
    type Result = <messages::SendExclusiveDeviceMessage as Message>::Result;

    fn handle(&mut self, msg: messages::SendExclusiveDeviceMessage, _: &mut Self::Context) -> Self::Result {
        if let Some(devices) = self.users.get(&msg.user_id) {
            for (device_id, addr) in devices {
                if *device_id != msg.excluded_device_id {
                    addr.do_send(msg.message.clone());
                }
            }
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
            .send(internal::AddConnection {
                conn: ctx.address(),
                user_id: self.user_id,
                device_id: self.device_id,
            })
            .into_actor(self)
            .then(|_, _, _| fut::ready(()))
            .wait(ctx);

        // Close connection after 5 minutes.
        ctx.run_later(Duration::from_secs(60 * 5), |_, ctx| {
            ctx.text(serde_json::to_string(&messages::DeviceMessage::Timeout).unwrap());
            ctx.stop()
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server.do_send(internal::RemoveConnection {
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

impl Handler<messages::DeviceMessage> for Connection {
    type Result = <messages::DeviceMessage as Message>::Result;

    fn handle(&mut self, msg: messages::DeviceMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl Handler<internal::StopConnection> for Connection {
    type Result = <internal::StopConnection as Message>::Result;

    fn handle(&mut self, _: internal::StopConnection, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

mod internal {
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
}

pub mod messages {
    use actix::prelude::Message;
    use serde::Serialize;

    #[derive(Clone, Serialize)]
    #[serde(rename_all="snake_case")]
    pub enum DeviceMessage {
        AcceptPendingDevice,
        RefreshRequests,
        RefreshRemote,
        RefreshFolder(i32),
        RefreshNote { folder_id: i32, note_id: i32 },
        Text(String),
        Timeout,
    }

    impl Message for DeviceMessage {
        type Result = ();
    }

    pub struct SendDeviceMessage {
        pub user_id: i32,
        pub device_id: i32,
        pub message: DeviceMessage,
    }

    impl Message for SendDeviceMessage {
        type Result = ();
    }

    pub struct SendExclusiveDeviceMessage {
        pub user_id: i32,
        pub excluded_device_id: i32,
        pub message: DeviceMessage,
    }

    impl Message for SendExclusiveDeviceMessage {
        type Result = ();
    }
}
