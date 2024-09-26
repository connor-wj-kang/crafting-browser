use super::{Drawable, PaintCommand};
use skia_safe::{Canvas, Color4f, Paint, Rect};

struct DrawRect {
    paint_command: PaintCommand,
    rect: Rect,
    color: Color4f,
}

impl DrawRect {
    fn new(rect: Rect, color: Color4f) -> Self {
        let paint_command = PaintCommand::new(rect);
        Self {
            paint_command,
            rect,
            color,
        }
    }
}

impl Drawable for DrawRect {
    fn execute(&self, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_color4f(self.color, None);
        canvas.draw_rect(self.rect, &paint);
    }
}
