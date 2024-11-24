use std::{fs::File, io::Read, collections::HashMap};
use serde_json::Value;
use lazy_static::lazy_static;
use std::sync::Mutex;
use super::uinput_device::MyCommand;
use super::uinput_device;

use dirs::config_dir;

// the map contains as key a string made of "{app_name}{action_name}" and as
// value one or more commands
lazy_static! {
    static ref MAP: Mutex<HashMap<String, CommandValue>> = Mutex::new(get_hashmap());
}

// here I don't put only Arrays because I expect most actions to be made of a
// single command
#[derive(Clone)]
pub enum CommandValue {
    Command(MyCommand),
    Array(Vec<MyCommand>),
}

// initialize the map at the start of the application by calling this function once
pub fn initialize_map() {
    if let Err(err) = MAP.lock() {
        println!("Couldn't load config");
        println!("{err}");
    }
}

// this function gets the commands corresponding to the action in input and
// execute them
pub fn execute_msg(action_name: &str) {
    if let Some(command) = get_binding(action_name) {
        match command {
            CommandValue::Command(command) => {
                if let Err(_) = uinput_device::execute_cmd(command) {
                    println!("Error in executing command");
                }
            }
            CommandValue::Array(command_array) => {
                for command in command_array {
                    if let Err(_) = uinput_device::execute_cmd(command) {
                        println!("Error in executing one of the commands in the action");
                    }
                }
            }
        }
    }
}

// this function only executes a string of command specified with kbapp launch,
// ignoring the focused window
pub fn execute_custom(command: &str) {
    if let Some(command) = uinput_device::get_mycommand(&command) {
        if let Err(_) = uinput_device::execute_cmd(command) {
            println!("Error in executing command");
        }
    }
    else {
        println!("Error in parsing requested command");
    }
}

// this function gets the commands in the map corresponding to a particular action
fn get_binding(action_name: &str) -> Option<CommandValue> {
    let mut app = super::focused::getfocusedwindow();

    let map = match MAP.lock() {
        Ok(map) => map,
        _ => return None,
    };
    println!("Received command {} in app {}", action_name, app);
    app.push_str(&action_name);

    let command = map.get(app.as_str());
    match command {
        Some(command) => return Some(command.clone()),
        // if the action for the focused app is not found, it searches the same
        // action for the default app
        None => {
            let mut app = String::from("default");
            app.push_str(&action_name);
            let default_command = map.get(app.as_str());
            match default_command {
                Some(command) => {
                    println!("Requested command not found. Using default command");
                    return Some(command.clone());
                },
                None => {
                    println!("Requested command and default command not found");
                    return None;
                },
            }
        },
    }
}

// this function reloads the map
pub fn reload_map(){
    let mut map = match MAP.lock() {
        Ok(map) => map,
        _ => return,
    };
    *map = get_hashmap();
}

// this function parse the configuration file and allocates the corresponding
// hashmap
fn get_hashmap() -> HashMap<String, CommandValue> {
    let mut new_map: HashMap<String, CommandValue> = HashMap::new();

    let mut config_path = match config_dir() {
        Some(home) => home,
        None => {
            println!("Couldn't find config directory");
            return new_map;
        }
    };
    config_path.push("kbapp/config.json");

    let mut file = match File::open(config_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Couldn't find configuration file. Add the file and reload if you want to use app-specific actions");
            return new_map;
        }
    };
    let mut config_string = String::new();
    file.read_to_string(&mut config_string).expect("Couldn't read config file");

    let config_json: Value = match serde_json::from_str(&config_string) {
        Ok(configuration_json) => configuration_json,
        Err(err) => {
            println!("Problem in parsing json file:");
            println!("{err}");
            panic!();
        },
    };

    for app_config in config_json.as_array().unwrap() {
        let app_name = app_config.get("name");
        let app_name = match app_name {
            Some(app_name) => app_name,
            None => {
                println!("Required field name not found");
                continue;
            }
        };
        let app_name = match app_name.as_str() {
            Some(app_name) => app_name,
            None => {
                println!("Required field name is not a string");
                continue;
            }
        };

        // we get the app json and iterate over all fields expect name
        let mut app_json = app_config.as_object().expect("Not an object").clone();
        app_json.remove("name");

        for (key, binding) in app_json {
            // the key of the map is made of "{app_name}{action_name}"
            let mut map_key = String::from(app_name);
            map_key.push_str(key.as_str());

            // check that the corresponding value is a single command or an
            // array of commands
            match binding {
                Value::String(binding) => {
                    if let Some(command) = uinput_device::get_mycommand(&binding) {
                        new_map.insert(map_key, CommandValue::Command(command));
                    }
                    else {
                        println!("Error in parsing {} for app {}", key, app_name);
                        continue;
                    }
                }
                Value::Array(command_list) => {
                    let mut command_array: Vec<MyCommand> = Vec::new();
                    for binding in command_list {
                        if let Some(binding) = binding.as_str() {
                            if let Some(command) = uinput_device::get_mycommand(binding) {
                                command_array.push(command);
                            }
                            else {
                                // if one command of an array has parsing errors, we only ignore
                                // that command
                                println!("Error in parsing one of the commands {} for app {}", key, app_name);
                                continue;
                            }
                        }
                    }
                    new_map.insert(map_key, CommandValue::Array(command_array));
                }
                _ => {
                    println!("Value of action {key} of app {app_name} is not a string or array of strings");
                }
            }
        }
    }
    println!("Configuration loaded correctly");

    new_map
}

