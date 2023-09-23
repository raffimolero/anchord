mod scancodes;

use self::scancodes::{self as k, ScanCode};

use std::{
    io::{stdout, Write},
    ops::ControlFlow,
};

use winit::event::{ElementState as S, KeyboardInput, VirtualKeyCode};

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
    anchor: Pos,
    cursor: Pos,
    mode: Mode,
}

impl State {
    pub fn new() -> Self {
        Self {
            held_keys: Vec::with_capacity(2),
            max_held: 2,
            anchor: Pos::default(),
            cursor: Pos::default(),
            mode: Mode::default(),
        }
    }

    fn filter_held(&mut self, input: KeyboardInput) -> ControlFlow<()> {
        let item_idx = self
            .held_keys
            .iter()
            .position(|code| *code == input.scancode);

        match input.state {
            S::Pressed => {
                if let Some(_pos) = item_idx {
                    return ControlFlow::Break(());
                }
                while self.held_keys.len() as u8 >= self.max_held {
                    let last_scancode = self.held_keys.remove(0);
                    let synthetic_input = KeyboardInput {
                        scancode: last_scancode,
                        state: S::Released,
                        virtual_keycode: None,
                        ..input
                    };
                    self.input_key(synthetic_input);
                }
                self.held_keys.push(input.scancode);
            }
            S::Released => {
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

        match self.mode {
            Mode::Normal => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    return ControlFlow::Break(());
                }
                match (input.state, input.scancode) {
                    (S::Pressed, k::A) => {
                        println!("a");
                    }
                    code => {
                        print!("unknown code: ")
                    }
                }
                println!("{:?} {}", input.virtual_keycode, input.scancode);
                println!("{:?}", self.held_keys);
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
