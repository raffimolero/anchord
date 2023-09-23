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
    data: TextBufRef,
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
            data: init_buffer,
            next: MaybeSplit::NONE,
        });

        Self { w, h, nodes }
    }

    pub fn render(
        &self,
        txt_bufs: &TextBuffers,
        glyph_buffer: &mut glyphon::Buffer,
        font_system: &mut FontSystem,
    ) {
        let mut l = 0;
        let mut t = 0;
        let mut r = self.w;
        let mut b = self.h;
        // let mut stack = vec![];

        let mut char_buf_map = vec![255_u8; self.w as usize * self.h as usize];
        let mut i = 0;
        self.nodes[i];
        // TODO: polish notation traversal
        //       take self.nodes, start from 0, and "recursively" map
        //       each char in char_buf_map to a buffer index
        // then use the buf map to push the appropriate chars into the glyphon buffer

        // { // inlined from glyphon::Buffer::set_text
        //     let attrs = Attrs::new().family(Family::Monospace);
        //     let shaping = Shaping::Basic;
        //     glyph_buffer.lines.clear();
        //     for line in BidiParagraphs::new(text) {
        //         glyph_buffer.lines.push(BufferLine::new(
        //             line.to_string(),
        //             AttrsList::new(attrs),
        //             shaping,
        //         ));
        //     }
        //     // Make sure there is always one line
        //     if glyph_buffer.lines.is_empty() {
        //         glyph_buffer.lines.push(BufferLine::new(
        //             String::new(),
        //             AttrsList::new(attrs),
        //             shaping,
        //         ));
        //     }

        //     glyph_buffer.scroll = 0;

        //     glyph_buffer.shape_until_scroll(font_system);
        // };
    }

    pub fn len(&self) -> usize {
        self.nodes
            .iter()
            .position(|e| e.is_none())
            .unwrap_or(Self::MAX_WINDOWS)
    }
}
