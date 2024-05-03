use clap::{Parser,Subcommand};

/// A daemon for UI automation, can simulate keyboard, mouse and is
/// aware of the focused Hyprland window.
///
/// Requires a running istance of kbapp and r/w access to /dev/uinput (see Wiki)
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// get name of the current window
    GetName,
    /// start daemon
    Start,
    /// reload config
    Reload,
    /// execute an action from the config file
    Action {
        /// name of the action
        name: String
    },
    /// execute a single command
    Launch {
        command: String
    },
}

pub fn parse_input() {
    let args = Args::parse();
    match args.command {
        Subcommands::GetName => {
            println!("The name of the focused window is {}", super::focused::getfocusedwindow());
            return;
        }
        Subcommands::Start => super::socket::listen(),
        Subcommands::Action{name} => super::socket::send(&name),
        Subcommands::Launch{command} => super::socket::send(&format!("__launch:{}", command)),
        Subcommands::Reload => super::socket::send("__reload"),
    }
}
