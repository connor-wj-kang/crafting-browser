use super::{Drawable, PaintCommand};
use skia_safe::{Canvas, Color4f, Paint, RRect, Rect};

struct DrawRoundRect {
    paint_command: PaintCommand,
    round_rect: RRect,
    color: Color4f,
}

impl DrawRoundRect {
    fn new(rect: Rect, radius: f32, color: Color4f) -> Self {
        let paint_command = PaintCommand::new(rect);
        let round_rect = RRect::new_rect_xy(rect, radius, radius);
        Self {
            paint_command,
            round_rect,
            color,
        }
    }
}

impl Drawable for DrawRoundRect {
    fn execute(&self, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_color4f(self.color, None);
        canvas.draw_rrect(self.round_rect, &paint);
    }
}
