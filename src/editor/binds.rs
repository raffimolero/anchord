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
struct Input {
    held_keys: Vec<ScanCode>,
    max_held: u8,
    held_key_overflow: u8,
}

impl Input {
    fn new() -> Self {
        const DEFAULT_MAX_HELD: u8 = 2;
        Self {
            held_keys: Vec::with_capacity(10),
            max_held: DEFAULT_MAX_HELD,
            held_key_overflow: 0,
        }
    }

    fn filter_held(
        &mut self,
        input: KeyboardInput,
    ) -> ControlFlow<(), (&[ScanCode], ElementState, ScanCode)> {
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
                if self.held_keys.len() as u8 - self.held_key_overflow > self.max_held {
                    self.held_key_overflow += 1;
                }
            }
            ElementState::Released => {
                if let Some(pos) = item_idx {
                    self.held_keys.remove(pos);
                    if (pos as u8) < self.held_key_overflow {
                        self.held_key_overflow -= 1;
                    }
                }
            }
        }

        let offset = match input.state {
            ElementState::Pressed => 1,
            ElementState::Released => 0,
        };
        let effective_held =
            &self.held_keys[self.held_key_overflow as usize..self.held_keys.len() - offset];
        let input = (effective_held, input.state, input.scancode);
        ControlFlow::Continue(input)
    }
}

#[derive(Debug)]
pub struct State {
    input: Input,
    anchor: Pos,
    cursor: Pos,
    mode: Mode,
}

impl State {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            anchor: Pos::default(),
            cursor: Pos::default(),
            mode: Mode::default(),
        }
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
        let ControlFlow::Continue(effective_input) = self.input.filter_held(input) else {
            return ControlFlow::Continue(());
        };

        use scancodes::*;
        use ElementState::{Pressed as Dn, Released as Up};

        match self.mode {
            Mode::Normal => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    return ControlFlow::Break(());
                }
                match effective_input {
                    (&[J], Up, J) => println!("J"),
                    (&[K], Dn, J) => println!("KJ"),
                    (&[J], Dn, K) => println!("JK"),
                    (&[L], Dn, SEMI) => println!("L;"),
                    (&[SEMI], Dn, L) => println!(";L"),
                    code => print!("unknown code: "),
                }
                println!("{effective_input:?}");
                println!(
                    "{:?} {} {:?}",
                    input.virtual_keycode, input.scancode, self.input,
                );
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
