use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use actix::prelude::*;
use actix_web_actors::ws::{Message as WebsocketMessage, ProtocolError, WebsocketContext};

use crate::server::Server;

type UserSession = (i32, u64);

pub struct Manager {
    sessions: HashSet<UserSession>,
    server: Addr<Server>,
}

impl Manager {
    pub fn new(server: Addr<Server>) -> Self {
        Manager {
            sessions: HashSet::new(),
            server,
        }
    }
}

impl Actor for Manager {
    type Context = Context<Self>;
}

impl Handler<messages::CreateSession> for Manager {
    type Result = <messages::CreateSession as Message>::Result;

    fn handle(&mut self, msg: messages::CreateSession, ctx: &mut Self::Context) -> Self::Result {
        let mut session_id = 0;

        while self.sessions.contains(&(msg.user_id, session_id)) {
            session_id += 1;
        }

        self.sessions.insert((msg.user_id, session_id));

        Session::new(session_id, msg.user_id, self.server.clone(), ctx.address())
    }
}

impl Handler<messages::RemoveSession> for Manager {
    type Result = <messages::RemoveSession as Message>::Result;

    fn handle(&mut self, msg: messages::RemoveSession, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&(msg.user_id, msg.session_id));
    }
}

#[derive(MessageResponse)]
pub struct Session {
    id: u64,
    user_id: i32,
    server: Addr<Server>,
    manager: Addr<Manager>,
    hb: Instant,
}

impl Session {
    const HB_CHECK_INTERVAL: Duration = Duration::from_secs(60);
    const CONNECTION_TIMEOUT: Duration = Duration::from_secs(120);

    pub fn new(id: u64, user_id: i32, server: Addr<Server>, manager: Addr<Manager>) -> Self {
        Session {
            id,
            user_id,
            server,
            manager,
            hb: Instant::now(),
        }
    }

    fn check_hb(&mut self) -> bool {
        Instant::now().duration_since(self.hb) < Self::CONNECTION_TIMEOUT
    }
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::debug!("starting session with ({},{})", self.user_id, self.id);

        self.server
            .send(crate::server::messages::Connect {
                session_id: self.id,
                user_id: self.user_id,
                addr: ctx.address(),
            })
            .into_actor(self)
            .then(|_, _, _| fut::ready(()))
            .wait(ctx);

        ctx.run_interval(Self::HB_CHECK_INTERVAL, |act, ctx| {
            if !act.check_hb() {
                ctx.stop();
            }
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // If we are stopping due to timeout, send a timeout message
        if !self.check_hb() {
            //ctx.text(messages::outgoing::Timeout.value());
        }

        self.server.do_send(crate::server::messages::Disconnect {
            session_id: self.id,
            user_id: self.user_id,
        });

        self.manager.do_send(messages::RemoveSession {
            session_id: self.id,
            user_id: self.user_id,
        });

        Running::Stop
    }
}

impl StreamHandler<Result<WebsocketMessage, ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<WebsocketMessage, ProtocolError>, ctx: &mut Self::Context) {
        let msg = if let Ok(msg) = msg { msg } else { return };

        match msg {
            WebsocketMessage::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            WebsocketMessage::Pong(_) => self.hb = Instant::now(),
            WebsocketMessage::Text(text) => {
                ctx.text(text);
            }
            WebsocketMessage::Close(_) => ctx.stop(),
            _ => {}
        }
    }
}

impl Handler<messages::SendMessage> for Session {
    type Result = <messages::SendMessage as actix::Message>::Result;

    fn handle(&mut self, msg: messages::SendMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.message);
    }
}

pub mod messages {
    use super::{Message, Session};

    pub struct CreateSession {
        pub user_id: i32,
    }

    impl Message for CreateSession {
        type Result = Session;
    }

    pub struct RemoveSession {
        pub user_id: i32,
        pub session_id: u64,
    }

    impl Message for RemoveSession {
        type Result = ();
    }

    pub struct SendMessage {
        pub message: String,
    }

    impl Message for SendMessage {
        type Result = ();
    }
}
