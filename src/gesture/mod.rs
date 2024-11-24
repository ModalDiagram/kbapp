use std::{fs::{File, OpenOptions}, os::{fd::OwnedFd, unix::prelude::OpenOptionsExt}, path::Path};
use input::{event::DeviceEvent, Event, Libinput, LibinputInterface};
use input::event::EventTrait;
use input::event::gesture::*;
use input::Event::Gesture;
use nix::poll::{poll, PollFlags, PollFd};

struct State {
    x: f64,
    y: f64,
    start_time: u32,
    long_event: bool,
}

impl State {
    fn start_hold(&mut self) { }

    fn end_hold(&mut self, event: GestureHoldEndEvent) {
        let command = format!("{}hold", event.finger_count());
        super::app_bindings::execute_msg(&command);
    }

    fn start_swipe(&mut self, event: GestureSwipeBeginEvent) {
        self.x = 0.; self.y = 0.;
        self.start_time = event.time();
    }

    fn update_swipe(&mut self, event: GestureSwipeUpdateEvent) {
        self.x = self.x + event.dx();
        self.y = self.y - event.dy();
        if (event.time() - self.start_time) > 200 {
            self.long_event = true;
        }
        if !self.long_event { return; }
        if self.x * self.x + self.y * self.y > 2000. {
            let command = format!("{}{}", event.finger_count(), self.choose_direction());
            super::app_bindings::execute_msg(&command);
            self.x = 0.; self.y = 0.;
        }
    }

    fn end_swipe(&mut self, event: GestureSwipeEndEvent) {
        let current_state = self.long_event;
        self.long_event = false;
        if current_state { return; }
        let command = format!("{}{}", event.finger_count(), self.choose_direction());
        super::app_bindings::execute_msg(&command);
    }

    fn choose_direction(&self) -> &str {
        let horiz = self.x.abs() > self.y.abs();
        let oblique = (horiz && self.y.abs() / self.x.abs() > 0.414) || (!horiz && self.x.abs() / self.y.abs() > 0.414);
        let left = self.x < 0.;
        let down = self.y < 0.;
        let direction: &str = if horiz && !oblique { if left { "left" } else { "right" } }
        else if !horiz && !oblique { if down { "down" } else { "up" } }
        else {
            if left && down { "leftdown" }
            else if left && !down {"leftup"}
            else if !left && down { "rightdown" }
            else { "rightup" }
        };
        return direction;
    }
}

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read(true)
            .write(true)
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}


/// Handles a libinput event.
///
/// Currently, only tab and pen events are of interest.
fn handle_event(event: Event, state: &mut State){
    if let Gesture(event) = event {
        if event.finger_count() < 3 { return; }
        match event {
            GestureEvent::Swipe(gesture_event) => {
                // println!("Got event: {:?}", event);
                match gesture_event {
                    GestureSwipeEvent::Begin(swipe_event) => {
                        state.start_swipe(swipe_event);
                    }
                    GestureSwipeEvent::Update(swipe_event) => {
                        state.update_swipe(swipe_event);
                    }
                    GestureSwipeEvent::End(swipe_event) => {
                        state.end_swipe(swipe_event);
                    }
                    _ => { println!("Niente"); }
                }
            }

            GestureEvent::Hold(hold_event) => {
                match hold_event {
                    GestureHoldEvent::Begin(_) => {
                        state.start_hold();
                    }
                    GestureHoldEvent::End(hold_event) => {
                        if !hold_event.cancelled() { state.end_hold(hold_event); }
                    }
                    _ => { println!("Niente"); }
                }
            }
            GestureEvent::Pinch(_) => { }
            _ => {}
        };
    }
}

/// Adds connected devices to input if they have at least one
/// of the capabilities in capabilities.
///
/// This opens a udev context and gets the events of added devices, then
/// checks the capabilities of each device
fn add_connected_devices(input: &mut Libinput, capabilities: &Vec<input::DeviceCapability>) {
    let mut input_udev = Libinput::new_with_udev(Interface);
    input_udev.udev_assign_seat("seat0").unwrap();
    input_udev.dispatch().unwrap();
    for event in &mut input_udev {
        // if the event is a device addition
        if let Event::Device(event) = event {
            if let DeviceEvent::Added(_) = event {
                let device = event.device();
                // if the device has at least one of the capabilities in capabilities
                for capability in capabilities {
                    if device.has_capability(*capability) {
                        let sysname = format!("/dev/input/{}", device.sysname());
                        input.path_add_device(&sysname);
                        println!("Added connected device");
                        break;
                    }
                }
            }
        }
    }
}

// Handle events in input
fn handle_libinput(input: &mut Libinput, state: &mut State) {
    input.dispatch().unwrap();
    for event in input {
        handle_event(event, state);
    }
}

pub fn listen() {
    let mut input = Libinput::new_from_path(Interface);
    let capabilities = vec![input::DeviceCapability::Gesture];
    // 1) add connected devices
    add_connected_devices(&mut input, &capabilities);
    let input_clone = input.clone();
    let poll_input = PollFd::new(&input_clone, PollFlags::POLLIN);
    let mut state = State{x: 0.0, y: 0.0, start_time: 0, long_event: false};
    while poll(&mut [poll_input ], -1).is_ok() {
        handle_libinput(&mut input, &mut state);
    }
}
