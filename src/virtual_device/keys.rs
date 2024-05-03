use uinput::event::keyboard::Key;
use uinput::event::controller::Mouse;

#[derive(Clone)]
pub enum MyKey {
    PressableKey(PressableKey),
    MouseMove([i32; 2])
}

#[derive(Clone, Debug)]
pub enum PressableKey {
    MouseClick(Mouse),
    Keyboard(Key),
}

// parse a key in the corresponding mouse or keyboard action
pub fn get_key(key_str: &str) -> Option<MyKey> {
    if let Some(mouse_str) = key_str.strip_prefix("mouse:") {
        match mouse_str {
            "left" | "leftclick" => return Some(MyKey::PressableKey(PressableKey::MouseClick(Mouse::Left))),
            "right" | "rightclick" => return Some(MyKey::PressableKey(PressableKey::MouseClick(Mouse::Right))),
            "middle" | "middleclick" => return Some(MyKey::PressableKey(PressableKey::MouseClick(Mouse::Middle))),
            _ => (),
        };
        let mut coords: [i32; 2] = [0,0];
        for (idx, coor) in mouse_str.split(",").enumerate() {
            if let Ok(int) = coor.parse() {
                coords[idx] = int;
            }
            else {
                return None
            }
        }
        return Some(MyKey::MouseMove(coords));
    };
    use Key::*;
    Some(MyKey::PressableKey(PressableKey::Keyboard(match key_str {
        "esc" => Esc,
        "minus" => Minus,
        "equal" => Equal,
        "backspace" => BackSpace,
        "tab" => Tab,
        "q" => Q,
        "w" => W,
        "e" => E,
        "r" => R,
        "t" => T,
        "y" => Y,
        "u" => U,
        "i" => I,
        "o" => O,
        "p" => P,
        "leftbrace" => LeftBrace,
        "rightbrace" => RightBrace,
        "enter" => Enter,
        "leftcontrol" | "leftctrl" | "control" | "ctrl" => LeftControl,
        "a" => A,
        "s" => S,
        "d" => D,
        "f" => F,
        "g" => G,
        "h" => H,
        "j" => J,
        "k" => K,
        "l" => L,
        ";" | "semicolon" => SemiColon,
        "'" | "apostrophe" => Apostrophe,
        "`" | "grave" => Grave,
        "leftshift" | "shift" => LeftShift,
        "backslash" | "\\" => BackSlash,
        "z" => Z,
        "x" => X,
        "c" => C,
        "v" => V,
        "b" => B,
        "n" => N,
        "m" => M,
        "," => Comma,
        "." => Dot,
        "/" | "slash" => Slash,
        "rightshift" => RightShift,
        "leftalt" | "alt" => LeftAlt,
        "space" => Space,
        "capslock" => CapsLock,
        "f1" => F1,
        "f2" => F2,
        "f3" => F3,
        "f4" => F4,
        "f5" => F5,
        "f6" => F6,
        "f7" => F7,
        "f8" => F8,
        "f9" => F9,
        "f10" => F10,
        "numlock" => NumLock,
        "scrolllock" => ScrollLock,
        "f11" => F11,
        "f12" => F12,
        "rightcontrol" | "rightctrl" => RightControl,
        "sysrq" => SysRq,
        "rightalt" => RightAlt,
        "linefeed" => LineFeed,
        "home" => Home,
        "up" => Up,
        "pageup" => PageUp,
        "left" => Left,
        "right" => Right,
        "end" => End,
        "down" => Down,
        "pagedown" => PageDown,
        "insert" => Insert,
        "delete" => Delete,
        "leftmeta" | "meta" | "super" | "leftsuper" => LeftMeta,
        "rightmeta" | "rightsuper" => RightMeta,
        "scrollup" => ScrollUp,
        "scrolldown" => ScrollDown,
        "f13" => F13,
        "f14" => F14,
        "f15" => F15,
        "f16" => F16,
        "f17" => F17,
        "f18" => F18,
        "f19" => F19,
        "f20" => F20,
        "f21" => F21,
        "f22" => F22,
        "f23" => F23,
        "f24" => F24,
        _ => return None,
  })))
}
