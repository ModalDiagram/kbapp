// this module parse the command arguments and has their definitions
pub mod cli;

// this module finds the name of the focused window
pub mod focused;

// this module manages the server and the client using Unix Domain Sockets
pub mod socket;

// this module contains the hashmap with action relative to each application
pub mod app_bindings;

// this module takes care of input emulation and execution of commands
pub mod uinput_device;

pub mod gesture;

#[tokio::main]
async fn main() {
    let listeners = cli::parse_input();
    for listener in listeners {
        let _ = listener.await;
    }
}
