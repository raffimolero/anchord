mod scancodes;

use self::scancodes::ScanCode;

use std::{
    io::{stdout, Write},
    ops::ControlFlow,
};

use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Mode {
    #[default]
    Normal,
    Insert,
}

#[derive(Debug)]
pub struct State {
    held_keys: Vec<ScanCode>,
    max_held: u8,
    history: Vec<(ElementState, ScanCode)>,
    anchor: Pos,
    cursor: Pos,
    mode: Mode,
}

impl State {
    pub fn new() -> Self {
        const DEFAULT_MAX_HELD: u8 = 2;
        Self {
            held_keys: Vec::with_capacity(10),
            max_held: DEFAULT_MAX_HELD,
            history: Vec::with_capacity(DEFAULT_MAX_HELD as usize),
            anchor: Pos::default(),
            cursor: Pos::default(),
            mode: Mode::default(),
        }
    }

    fn filter_held(&mut self, input: KeyboardInput) -> ControlFlow<()> {
        // TODO: allow key repeat and holding more than max keys
        // but trigger key releases before then
        // use a keypress queue for macros
        let item_idx = self
            .held_keys
            .iter()
            .position(|code| *code == input.scancode);

        match input.state {
            ElementState::Pressed => {
                if let Some(_pos) = item_idx {
                    return ControlFlow::Break(());
                }
                self.held_keys.push(input.scancode);
            }
            ElementState::Released => {
                if let Some(pos) = item_idx {
                    self.held_keys.remove(pos);
                }
            }
        }
        ControlFlow::Continue(())
    }

    // TODO:
    // pub fn from_config(path: Path) -> Self

    pub fn input_char(&mut self, chr: char) -> ControlFlow<()> {
        match self.mode {
            Mode::Normal => {}
            Mode::Insert => {
                print!("{chr}");
                stdout().flush().unwrap();
            }
        }
        ControlFlow::Continue(())
    }

    pub fn input_key(&mut self, input: KeyboardInput) -> ControlFlow<()> {
        if self.filter_held(input).is_break() {
            return ControlFlow::Continue(());
        }
        if self.history.len() as u8 >= self.max_held {
            self.history.remove(0);
        }
        self.history.push((input.state, input.scancode));

        use scancodes::*;
        use ElementState::{Pressed as Dn, Released as Up};

        match self.mode {
            Mode::Normal => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    return ControlFlow::Break(());
                }
                match self.history.as_slice() {
                    (&[(Dn, J), (Up, J)]) => println!("J"),
                    (&[(Dn, K), (Dn, J)]) => println!("KJ"),
                    (&[(Dn, J), (Dn, K)]) => println!("JK"),
                    (&[(Dn, L), (Dn, SEMI)]) => println!("L;"),
                    (&[(Dn, SEMI), (Dn, L)]) => println!(";L"),
                    code => print!("unknown code: "),
                }
                println!(
                    "{:?} {} {:?}",
                    input.virtual_keycode, input.scancode, self.history
                )
            }
            Mode::Insert => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    self.mode = Mode::Normal;
                }
            }
        }
        // println!("Input: {input:?}");
        ControlFlow::Continue(())
    }
}
