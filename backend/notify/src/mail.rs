use std::rc::Rc;

use actix::{Actor, Context, ContextFutureSpawner, Handler, WrapFuture, Recipient};

pub type MailRecipient = Recipient<messages::SendMail>;

pub struct Server {
    mail_address: String,
    endpoint: Rc<String>,
    key: Rc<String>,
}

impl Server {
    pub fn new(mail_address: String, endpoint: String, key: String) -> Self {
        Server {
            mail_address,
            endpoint: Rc::new(endpoint),
            key: Rc::new(key),
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<messages::SendMail> for Server {
    type Result = ();

    fn handle(&mut self, msg: messages::SendMail, ctx: &mut Self::Context) -> Self::Result {
        log::debug!("Sending mail {msg:?}");

        let mail_address = self.mail_address.clone();
        let endpoint = self.endpoint.clone();
        let key = self.key.clone();

        async move {
            let client = awc::Client::new();

            let request = client
                .post(endpoint.as_ref())
                .basic_auth("api", key.as_ref())
                .send_form(&internal::MailRequest {
                    from: mail_address,
                    to: msg.to,
                    subject: msg.subject,
                    html: msg.html,
                })
                .await;

            let mut request = match request {
                Ok(req) => req,
                Err(e) => {
                    log::error!("failed to send request, {e:?}");
                    return;
                }
            };

            let body = match request.body().await {
                Ok(body) => body,
                Err(e) => {
                    log::error!("failed to receive response, {e:?}");
                    return;
                }
            };

            log::debug!("Received response {body:?}");
        }
        .into_actor(self)
        .spawn(ctx);
    }
}

mod internal {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct MailRequest {
        pub from: String,
        pub to: String,
        pub subject: String,
        pub html: String,
    }
}

pub mod messages {
    use actix::Message;

    #[derive(Debug)]
    pub struct SendMail {
        pub to: String,
        pub subject: String,
        pub html: String,
    }

    impl Message for SendMail {
        type Result = ();
    }
}
