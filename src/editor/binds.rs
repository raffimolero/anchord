mod scancodes;

use std::{
    io::{stdout, Write},
    ops::ControlFlow,
};

use winit::event::{KeyboardInput, VirtualKeyCode};

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
    anchor: Pos,
    cursor: Pos,
    mode: Mode,
}

impl State {
    pub fn new() -> Self {
        Self {
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
        match self.mode {
            Mode::Normal => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    return ControlFlow::Break(());
                }
                match input.scancode {
                    scancodes::A => {
                        println!("a");
                    }
                    code => {
                        print!("unknown code: ")
                    }
                }
                println!("{:?} {}", input.virtual_keycode, input.scancode);
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
