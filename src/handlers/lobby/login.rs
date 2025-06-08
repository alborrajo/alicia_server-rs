use crate::{commands::lobby::login::Login, handlers::Handler};

struct LoginHandler {}
impl Handler<Login> for LoginHandler {
    fn handle(command: Login) {
        todo!();
    }
}