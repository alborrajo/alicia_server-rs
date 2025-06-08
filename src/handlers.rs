use crate::commands::Command;

pub mod lobby;

trait Handler<T> where T: Command {
    fn handle(command: T);
}