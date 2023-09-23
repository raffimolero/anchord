//! for quick reference on my QWERTY keyboard.
//! we are not using virtual keycodes, as we do not care about mnemonics,
//! and would rather not break when keyboard layouts change.
//! if your keyboard isn't supported then i may change my mind.

pub const F1: u32 = 59;
pub const F2: u32 = 60;
pub const F3: u32 = 61;
pub const F4: u32 = 62;
pub const F5: u32 = 63;
pub const F6: u32 = 64;
pub const F7: u32 = 65;
pub const F8: u32 = 66;
pub const F9: u32 = 67;
pub const F10: u32 = 68;
pub const F11: u32 = 87;
pub const F12: u32 = 88;

pub const GRAVE: u32 = 41;
pub const KEY1: u32 = 2;
pub const KEY2: u32 = 3;
pub const KEY3: u32 = 4;
pub const KEY4: u32 = 5;
pub const KEY5: u32 = 6;
pub const KEY6: u32 = 7;
pub const KEY7: u32 = 8;
pub const KEY8: u32 = 9;
pub const KEY9: u32 = 10;
pub const KEY0: u32 = 11;
pub const MINUS: u32 = 12;
pub const EQUALS: u32 = 13;
pub const BACK: u32 = 14;

pub const TAB: u32 = 15;
pub const Q: u32 = 16;
pub const W: u32 = 17;
pub const R: u32 = 19;
pub const T: u32 = 20;
pub const Y: u32 = 21;
pub const U: u32 = 22;
pub const I: u32 = 23;
pub const O: u32 = 24;
pub const P: u32 = 25;
pub const L_BRACKET: u32 = 26;
pub const R_BRACKET: u32 = 27;
pub const BACKSLASH: u32 = 43;

pub const A: u32 = 30;
pub const S: u32 = 31;
pub const D: u32 = 32;
pub const F: u32 = 33;
pub const G: u32 = 34;
pub const H: u32 = 35;
pub const J: u32 = 36;
pub const K: u32 = 37;
pub const L: u32 = 38;
pub const SEMICOLON: u32 = 39;
pub const APOSTROPHE: u32 = 40;
pub const RETURN: u32 = 28;

pub const Z: u32 = 44;
pub const X: u32 = 45;
pub const C: u32 = 46;
pub const V: u32 = 47;
pub const B: u32 = 48;
pub const N: u32 = 49;
pub const M: u32 = 50;
pub const COMMA: u32 = 51;
pub const PERIOD: u32 = 52;
pub const SLASH: u32 = 53;
pub const R_SHIFT: u32 = 54;

pub const L_CONTROL: u32 = 29;
pub const L_ALT: u32 = 56;
pub const R_ALT: u32 = 57400;
pub const R_CONTROL: u32 = 57373;

pub const UP: u32 = 57416;
pub const DOWN: u32 = 57424;
pub const LEFT: u32 = 57419;
pub const RIGHT: u32 = 57421;

pub const INSERT: u32 = 57426;
pub const DELETE: u32 = 57427;
pub const HOME: u32 = 57415;
pub const END: u32 = 57423;
pub const PAGE_UP: u32 = 57417;
pub const PAGE_DOWN: u32 = 57425;

pub const NUMPAD_ADD: u32 = 78;
pub const NUMPAD_SUBTRACT: u32 = 74;
pub const NUMPAD_MULTIPLY: u32 = 55;
pub const NUMPAD_DIVIDE: u32 = 57397;
pub const NUMPAD_RETURN: u32 = 57372;

pub const NUMPAD1: u32 = 79;
pub const NUMPAD2: u32 = 80;
pub const NUMPAD3: u32 = 81;
pub const NUMPAD4: u32 = 75;
pub const NUMPAD5: u32 = 76;
pub const NUMPAD6: u32 = 77;
pub const NUMPAD7: u32 = 71;
pub const NUMPAD8: u32 = 72;
pub const NUMPAD9: u32 = 73;
pub const NUMPAD0: u32 = 82;
