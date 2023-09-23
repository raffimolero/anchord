use std::ops::ControlFlow;

use winit::event::{KeyboardInput, VirtualKeyCode};

#[derive(Debug)]
pub struct Binds {}

impl Binds {
    pub fn new() -> Self {
        Self {}
    }

    // TODO:
    // pub fn from_config(path: Path) -> Self

    pub fn input_char(&mut self, chr: char) -> ControlFlow<()> {
        println!("Char: {chr:?} ({:?})", u8::try_from(chr));
        ControlFlow::Continue(())
    }

    pub fn input_key(&mut self, input: KeyboardInput) -> ControlFlow<()> {
        println!("Input: {input:?}");
        if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    }
}
