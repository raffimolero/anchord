mod text;

use self::text::{ActiveTextWindow, ActiveWindows};
use std::{
    num::NonZeroUsize,
    ops::{Index, IndexMut},
};

use glyphon::{Buffer, FontSystem};
use winit::{
    event::{KeyboardInput, VirtualKeyCode},
    event_loop::ControlFlow,
};

// Text Buffers

#[derive(Debug, Clone, Default)]
pub struct TextBuffer {
    data: (), // TODO
}

impl TextBuffer {
    fn new_scratch() -> Self {
        Self { data: todo!() }
    }
}

#[derive(Debug, Clone)]
pub struct TextBuffers(Vec<TextBuffer>);

impl TextBuffers {
    fn new_scratch() -> Self {
        Self(vec![TextBuffer::new_scratch()])
    }
}

// TextBufRef

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextBufRef(NonZeroUsize);

impl From<usize> for TextBufRef {
    fn from(value: usize) -> Self {
        Self(NonZeroUsize::try_from(value + 1).unwrap())
    }
}

impl Index<TextBufRef> for TextBuffers {
    type Output = TextBuffer;

    fn index(&self, index: TextBufRef) -> &Self::Output {
        &mut self.0[index.0.get() - 1]
    }
}

impl IndexMut<TextBufRef> for TextBuffers {
    fn index_mut(&mut self, index: TextBufRef) -> &mut Self::Output {
        &mut self.0[index.0.get() - 1]
    }
}

// Editor

#[derive(Debug)]
pub struct Editor {
    text_buffers: TextBuffers,
    active_windows: ActiveWindows,
}

impl Editor {
    pub fn new_scratch(w: u16, h: u16) -> Self {
        Self {
            text_buffers: TextBuffers::new_scratch(),
            active_windows: ActiveWindows::new(w, h, TextBufRef::from(1)),
        }
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
        self.active_windows
            .render(&self.text_buffers, buffer, font_system);
    }
}
