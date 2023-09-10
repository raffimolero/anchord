use super::{TextBufRef, TextBuffers};

use glyphon::{Attrs, Family, FontSystem, Shaping};

/// Window Dimension
pub type WinDim = u16;
type SplitDim = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct MaybeSplit(SplitDim);

impl MaybeSplit {
    const NONE: Self = Self(0);

    fn horizontal(left_width: WinDim) -> Self {
        Self(left_width as SplitDim)
    }

    fn vertical(top_height: WinDim) -> Self {
        Self(-(top_height as SplitDim))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WindowNode {
    buf: TextBufRef,
    next: MaybeSplit,
}

#[derive(Debug)]
pub struct ActiveWindows {
    w: WinDim,
    h: WinDim,
    nodes: [Option<WindowNode>; Self::MAX_WINDOWS],
}

impl ActiveWindows {
    pub const MAX_WINDOWS: usize = 24;

    pub fn new(w: WinDim, h: WinDim, init_buffer: TextBufRef) -> Self {
        let mut nodes = [None; Self::MAX_WINDOWS];

        nodes[0] = Some(WindowNode {
            buf: init_buffer,
            next: MaybeSplit::NONE,
        });

        Self {
            w, h, nodes
        }
    }

    // TODO: merge TextBuffer and glyphon::Buffer?
    pub fn render(&self, txt_bufs: &TextBuffers, glyph_buffer: &mut glyphon::Buffer, font_system: &mut FontSystem) {
        todo!()
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

    pub fn len(&self) -> usize {
        self.nodes.iter().position(|e| e.is_none()).unwrap_or(Self::MAX_WINDOWS)
    }
}

