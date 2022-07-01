use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};

use crate::session::Session;

pub struct Server {
    users: HashMap<i32, HashMap<u64, Addr<Session>>>,
}

impl Server {
    pub fn new() -> Self {
        Server { users: HashMap::new() }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<messages::Connect> for Server {
    type Result = <messages::Connect as actix::Message>::Result;

    fn handle(&mut self, msg: messages::Connect, _: &mut Self::Context) -> Self::Result {
        if let Some(sessions) = self.users.get_mut(&msg.user_id) {
            sessions.insert(msg.session_id, msg.addr);
        } else {
            let mut sessions = HashMap::new();
            sessions.insert(msg.session_id, msg.addr);
            self.users.insert(msg.user_id, sessions);
        }
    }
}

impl Handler<messages::Disconnect> for Server {
    type Result = <messages::Disconnect as actix::Message>::Result;

    fn handle(&mut self, msg: messages::Disconnect, _: &mut Self::Context) -> Self::Result {
        if let Some(sessions) = self.users.get_mut(&msg.user_id) {
            sessions.remove(&msg.session_id);
        }
    }
}

impl Handler<messages::SendMessage> for Server {
    type Result = <messages::SendMessage as actix::Message>::Result;

    fn handle(&mut self, msg: messages::SendMessage, _: &mut Self::Context) -> Self::Result {
        if let Some(sessions) = self.users.get_mut(&msg.user_id) {
            for session in sessions.iter() {
                session.1.do_send(crate::session::messages::SendMessage { message: msg.message.clone() });
            }
        }
    }
}

pub mod messages {
    use super::{Addr, Message, Session};

    pub struct Connect {
        pub user_id: i32,
        pub session_id: u64,
        pub addr: Addr<Session>,
    }

    impl Message for Connect {
        type Result = ();
    }

    pub struct Disconnect {
        pub user_id: i32,
        pub session_id: u64,
    }

    impl Message for Disconnect {
        type Result = ();
    }

    pub struct SendMessage {
        pub user_id: i32,
        pub message: String,
    }

    impl Message for SendMessage {
        type Result = ();
    }
}
