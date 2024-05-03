# Commands

To execute the command *command_name arg1 arg2*:

```
"name_action": "exec:command_name arg1 arg2"
```

Example:

```
{
  "name": "Spotify Premium",
  "action_1": "exec:playerctl -p spotify previous",
  "action_2": [
    "exec:playerctl -p spotify previous",
    "ctrl tab"
  ]
}
```

# Mouse

To press the left, right or middle button

```
"name_action": "mouse:left"
"name_action": "mouse:right"
"name_action": "mouse:middle"
```

To move the mouse at the position (x,y):

```
"name_action": "mouse:x,y"
```

Example:

```
{
  "name": "firefox",
  "action_1": "mouse:right",
  "action_2": "mouse:100,200 ctrl mouse:left"
}
```

# Keyboard

This is the list of all available keys. On the left are the names to use in your actions:

## Modifiers

* "leftmeta" | "meta" | "super" | "leftsuper" => LeftMeta,

* "rightmeta" | "rightsuper" => RightMeta,

* "leftcontrol" | "leftctrl" | "control" | "ctrl" => LeftControl,

* "rightcontrol" | "rightctrl" => RightControl,

* "leftshift" | "shift" => LeftShift,

* "rightshift" => RightShift,

* "leftalt" | "alt" => LeftAlt,

* "rightalt" => RightAlt,

* "capslock" => CapsLock,

## Special characters

* "esc" => Esc,

* "enter" => Enter,

* "backspace" => BackSpace,

* "tab" => Tab,

* "up" => Up,

* "left" => Left,

* "right" => Right,

* "down" => Down,

* "home" => Home,

* "pageup" => PageUp,

* "pagedown" => PageDown,

* "end" => End,

* "insert" => Insert,

* "delete" => Delete,

* "scrollup" => ScrollUp,

* "scrolldown" => ScrollDown,

* "numlock" => NumLock,

* "scrolllock" => ScrollLock,

* "sysrq" => SysRq,

* "linefeed" => LineFeed,

## Letters

* "q" => Q,

* "w" => W,

* "e" => E,

* "r" => R,

* "t" => T,

* "y" => Y,

* "u" => U,

* "i" => I,

* "o" => O,

* "p" => P,

* "a" => A,

* "s" => S,

* "d" => D,

* "f" => F,

* "g" => G,

* "h" => H,

* "j" => J,

* "k" => K,

* "l" => L,

* "z" => Z,

* "x" => X,

* "c" => C,

* "v" => V,

* "b" => B,

* "n" => N,

* "m" => M,

## Punctuation

* "leftbrace" => [,

* "rightbrace" => ],

* ";" | "semicolon" => ;,

* "'" | "apostrophe" => \',

* "\`" | "grave" => \`,

* "backslash" | "\\" => \\,

* "," => Comma,

* "." => Dot,

* "/" | "slash" => Slash,

* "minus" => Minus,

* "equal" => Equal,

* "space" => Space,

## F

* "f1" => F1,

* "f2" => F2,

* "f3" => F3,

* "f4" => F4,

* "f5" => F5,

* "f6" => F6,

* "f7" => F7,

* "f8" => F8,

* "f9" => F9,

* "f10" => F10,

* "f11" => F11,

* "f12" => F12,

* "f13" => F13,

* "f14" => F14,

* "f15" => F15,

* "f16" => F16,

* "f17" => F17,

* "f18" => F18,

* "f19" => F19,

* "f20" => F20,

* "f21" => F21,

* "f22" => F22,

* "f23" => F23,

* "f24" => F24,
