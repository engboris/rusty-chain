mod handle;
use crate::handle::create_listener;

fn main() {
    env_logger::init();
    match create_listener("127.0.0.1:8080") {
        Ok(()) => (),
        Err(e) => log::error!("{e}"),
    }
}
