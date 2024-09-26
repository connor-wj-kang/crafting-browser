mod draw_cursor;
mod draw_image;
mod draw_line;
mod draw_outline;
mod draw_rect;
mod draw_round_rect;
mod draw_text;

use skia_safe::{Canvas, Rect};

trait Drawable {
    fn execute(&self, canvas: &Canvas);
}

struct PaintCommand {
    rect: Rect,
    children: Vec<usize>,
}

impl PaintCommand {
    fn new(rect: Rect) -> Self {
        Self {
            rect,
            children: Vec::new(),
        }
    }
}
