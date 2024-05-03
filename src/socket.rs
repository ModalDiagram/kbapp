use std::{os::unix::net::UnixDatagram, fs};
use std::path::PathBuf;
use dirs::home_dir;


// this function acts as the server
// It listens on a Unix Domain Socket for Datagrams that contain the name of the
// action to be executed.
pub fn listen() {
    let mut socket_path = match home_dir() {
        Some(home) => home,
        None => PathBuf::from("/tmp")
    };
    socket_path.push(".kbapp_socket");
    if socket_path.exists() {
        let _ = fs::remove_file(&socket_path);
    }

    let listener = UnixDatagram::bind(socket_path).unwrap();
    println!("Socket correctly initialized");
    super::virtual_device::initialize_virtual_device();
    super::app_bindings::initialize_map();

    // Indefinitely, it listens for messages to execute
    loop {
        let mut message: Vec<u8> = vec![0;128];
        if let Ok(bytes) = listener.recv(&mut message){
            if let Ok(mut message) = String::from_utf8(message){
                // response contains the read bytes and remaining null \0 characters
                // which should be removed for comparisons with the hashmap's keys
                message.truncate(bytes);
                // in this case we only execute the given command
                if let Some(command) = message.strip_prefix("__launch:"){
                    super::app_bindings::execute_custom(command);
                    continue;
                }
                match message.as_str() {
                    // in this case we reload the config
                    "__reload" => super::app_bindings::reload_map(),
                    // in this case we execute the action
                    _ => super::app_bindings::execute_msg(&message),
                }
            }
        }
    }
}

// this function acts as the client
// action_name is the name of the action to be executed. It is sent as a Datagram
// to the server, which executes it
pub fn send(action_name: &str) {
    let sock = UnixDatagram::unbound().expect("Couldn't get socket");
    let mut path = match home_dir() {
        Some(home) => home,
        None => PathBuf::from("/tmp")
    };
    path.push(".kbapp_socket");
    sock.connect(path).expect("Couldn't connect to socket");
    sock.send(action_name.as_bytes()).expect("Couldn't send message");
}
