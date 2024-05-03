# What is this

kbapp is an UI automation app. It emulates mouse and keyboard input to do scripted actions.

These actions may depend on the focused application (in Hyprland) if specified in a configuration file.

# Requires


* access to /dev/uinput (to emulate mouse and keyboard without needing root access),
that can be granted for example with a udev rule that modifies the group of */dev/uinput* and by adding your user to the uinput group:

```
KERNEL=="uinput", GROUP="uinput", MODE="0660"
```

* a running instance of kbapp that works as a daemon and is launched through

```
kbapp start
```

# Examples

Once the daemon is running you can emulate mouse and keyboard with the syntax [here](syntax.md):

* press control+tab

```
kbapp launch "control tab"
```

* press control and left click

```
kbapp launch "control mouse:left"
```


* move mouse to coordinates (123,123) and left click

```
kbapp launch "mouse:123,123 mouse:left"
```

# Configuration

A working configuration can be found [here](config_examples/).

One of the key functionalities of kbapp is that the executed action can depend on the focused window.

A json configuration file is required at:

```
~/.config/kbapp/config.json
```

in the following syntax:

```
[
  {
    "name": "app_1",
    "name_action_1": "control shift tab",
    "name_action_2": "alt left"
  },{
    "name": "app_2",
    "name_action_1": "control mouse:left",
    "name_action_2": "control h"
  },
  ...,
  {
    "name": "default",
    "name_action_1": "control mouse:345,678",
    "name_action_2": "control j"
  }
]
```

Once this configuration file is loaded (you can load it for a running daemon by launching *kbapp reload*) you can execute an action with

```
kbapp action name_action_1
```

and one of the *name_action_1* will be executed depending on the name of the focused app.

The default one is selected if the focused app's name doesn't match any of the preceding names.

You can get the name that kbapp will use for the focused window by launching in another terminal (sleep is added to have time to focus the chosen window):

```
sleep 3; kbapp get-name
```

# Notes

* an action can be defined for any number of apps

* "default" app or actions are not necessary, but often useful

* for correct mouse movements, acceleration should be disabled. For correct emulation of some characters layout should be set to US.
These can be achieved in Hyprland with (make sure that names match those you get with *hyprctl devices*):

```
device {
  name = kbapp-virtual-device
  kb_layout = us
}

device {
  name = kbapp-virtual-device-1
  accel_profile = flat
  sensitivity = 0
}
```

* if you launch actions through Hyprland binds, the keys used to trigger the bind might interfere with the emulated one.
For this reason, I usually map binds to CAPS_LOCK + another key, as done [here](config_examples/hyprland_options.conf).

# Advanced config

* multiple commands (to be executed in succession) can be specified for any action with an array of strings:

```
{
  "name": "app_1",
  "name_action_1": [ "control shift tab", "mouse:123,456", "control mouse:right" ],
  "name_action_2": "alt left"
}
```

* Other than emulating mouse and keyboard it can also launch commands (*please exercise caution*),
with the syntax *exec:command_name arg1 arg2*. To combine this with input emulation you must use an array of string:

```
{
  "name": "app_1",
  "name_action_1": [ "super tab", "exec:firefox --new-instance", "super shift tab" ]
}
```
