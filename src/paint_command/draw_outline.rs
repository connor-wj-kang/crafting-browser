use super::{Drawable, PaintCommand};
use skia_safe::{Canvas, Color4f, Paint, Rect};

struct DrawOutline {
    paint_command: PaintCommand,
    stroke_width: f32,
    color: Color4f,
}

impl DrawOutline {
    fn new(rect: Rect, stroke_width: f32, color: Color4f) -> Self {
        let paint_command = PaintCommand::new(rect);
        Self {
            paint_command,
            stroke_width,
            color,
        }
    }
}

impl Drawable for DrawOutline {
    fn execute(&self, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint
            .set_color4f(self.color, None)
            .set_stroke_width(self.stroke_width);
        canvas.draw_rect(self.paint_command.rect, &paint);
    }
}
