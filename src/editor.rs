mod text;

use self::text::Text;

use glyphon::{Buffer, FontSystem};
use winit::{
    event::{KeyboardInput, VirtualKeyCode},
    event_loop::ControlFlow,
};

#[derive(Debug, Default)]
pub struct Editor {
    text: Text,
}

impl Editor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_char(&mut self, control_flow: &mut ControlFlow, chr: char) {
        println!("Char: {chr:?} ({})", chr as u8);
    }

    pub fn input_key(&mut self, control_flow: &mut ControlFlow, input: KeyboardInput) {
        println!("Input: {input:?}");
        if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
            control_flow.set_exit();
        }
    }

    pub fn update_buffer(&self, buffer: &mut Buffer, font_system: &mut FontSystem) {
        self.text.update_buffer(buffer, font_system);
    }
}
