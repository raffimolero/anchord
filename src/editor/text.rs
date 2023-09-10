use glyphon::{Attrs, Buffer, Family, FontSystem, Shaping};

#[derive(Debug, Default)]
pub struct Text {
    data: String,
    changed: bool,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn update_buffer(&self, buffer: &mut Buffer, font_system: &mut FontSystem) {
        if !self.changed() {
            return;
        }
        buffer.set_text(
            font_system,
            self.data.as_str(),
            Attrs::new().family(Family::Monospace),
            Shaping::Basic,
        );
    }
}
