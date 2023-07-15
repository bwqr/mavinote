pub mod ws {
    use actix::Actor;

    pub fn create_server() -> crate::ws::AddrServer {
        crate::ws::Server::new().start()
    }
}
