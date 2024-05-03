mod keys;

use std::process::Command;
use uinput::device::Device;

use uinput::Event::Controller;
use uinput::event::Controller::Mouse;
use uinput::event::controller::Mouse::{Left, Right, Middle};

use uinput::event::Relative::Position;
use uinput::event::relative::Position::{X,Y};

use uinput::event::Keyboard;
use std::thread;
use std::time::Duration;
use keys::get_key;
use keys::MyKey;

use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DEVICE: Mutex<Device> = Mutex::new(create_virtual_device().expect("Couldn't initialize virtual device"));
}

// A command can either be a cmd to execute or a series of keyboard and mouse actions
#[derive(Clone)]
pub enum MyCommand {
    Keys(Vec<MyKey>),
    Cmd(Box<str>),
}

// create a virtual device able of emulating keyboard and mouse
fn create_virtual_device() -> Result<Device, uinput::Error> {
    let device = uinput::open(std::path::Path::new("/dev/uinput"))?
        .name("kbapp-virtual-device")?
        // enable keyboard events
        .event(Keyboard::All)?
        // enable mouse events
        .event(Controller(Mouse(Left)))?
        .event(Controller(Mouse(Right)))?
        .event(Controller(Mouse(Middle)))?
        .event(Position(X))?
        .event(Position(Y))?
        .create()?;
    Ok(device)
}

// initialize the device at the start of the application by calling this function once
pub fn initialize_virtual_device() {
    if let Err(err) = DEVICE.lock() {
        println!("Couldn't initialize virtual device");
        println!("{err}");
    }
    else {
        println!("Virtual device started correctly");
    }
}

// parse a string to get the corresponding command
pub fn get_mycommand(string_command: &str) -> Option<MyCommand> {
    // if the string starts with exec:, the rest of the string is a cmd to execute
    if let Some(cmd) = string_command.strip_prefix("exec:") {
        return Some(MyCommand::Cmd(Box::from(cmd)));
    };
    // else we parse the individual keyboard and mouse actions
    let mut command_keys: Vec<MyKey> = vec![];
    for key in string_command.split(" "){
        if let Some(key) = get_key(key) {
            command_keys.push(key);
        }
        else {
            // if one action of the command couldn't be parsed, the whole command is discarded
            return None;
        };
    }
    Some(MyCommand::Keys(command_keys))
}

// this function executes a command
pub fn execute_cmd(command: MyCommand) -> Result<i32, uinput::Error> {
    // check if the command requires keyboard and mouse emulation or it is
    // just a cmd to execute
    match command {
        MyCommand::Keys(keys) => {
            let mut input_dev = match DEVICE.lock() {
                Ok(device) => device,
                _ => return Err(uinput::Error::NotFound),
            };

            // press all the keys in order and put the pressed keys in an array
            // so that they can be released later
            let mut pressed_keys: Vec<keys::PressableKey> = vec![];
            for key in keys {
                match key {
                    MyKey::PressableKey(key) => {
                        pressed_keys.push(key.clone());
                        // println!("Pressing {:?}", key);
                        match key {
                            keys::PressableKey::MouseClick(key) => {
                                input_dev.press(&Mouse(key))?;
                            },
                            keys::PressableKey::Keyboard(key) => {
                                input_dev.press(&key)?;
                            }
                        }
                    }
                    MyKey::MouseMove(coords) => {
                        println!("Moving to {},{}", coords[0], coords[1]);
                        input_dev.position(&X, -100000)?;
                        input_dev.position(&Y, -100000)?;
                        input_dev.synchronize()?;
                        thread::sleep(Duration::from_millis(25));
                        input_dev.position(&X, coords[0])?;
                        input_dev.position(&Y, coords[1])?;
                    }
                }
                thread::sleep(Duration::from_millis(10));
                input_dev.synchronize()?;
            }

            // release all the keys in order opposite to the pressing
            for key in pressed_keys.iter().rev() {
                // println!("Releasing {:?}", key);
                match key {
                    keys::PressableKey::MouseClick(key) => {
                        input_dev.release(&Mouse(*key))?;
                    },
                    keys::PressableKey::Keyboard(key) => {
                        input_dev.release(key)?;
                    }
                }
                thread::sleep(Duration::from_millis(25));
                input_dev.synchronize()?;
            }
            return Ok(0)
        },
        MyCommand::Cmd(cmd) => {
            let cmd_pieces: Vec<&str> = cmd.split(" ").collect();
            if let Some(first) = cmd_pieces.get(0){
                let mut cmd_exec = Command::new(first);
                for key in &cmd_pieces[1..] {
                    cmd_exec.arg(key);
                }
                let _ = cmd_exec.spawn();
                return Ok(0)
            }
            return Err(uinput::Error::NotFound)
        }
    }
}



