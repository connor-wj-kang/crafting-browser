use super::{Drawable, PaintCommand};
use skia_safe::{Canvas, Color4f, Font, Paint, Point, Rect};

struct DrawText<'text, 'font> {
    text: &'text str,
    font: &'font Font,
    paint_command: PaintCommand,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    color: Color4f,
}

impl<'text, 'font> DrawText<'text, 'font> {
    fn new(left: f32, top: f32, text: &'text str, font: &'font Font, color: Color4f) -> Self {
        let right = left + font.measure_str(text, None).1.width();
        let bottom = top - font.metrics().1.ascent + font.metrics().1.descent;
        let paint_command = PaintCommand::new(Rect::new(left, top, right, bottom));
        Self {
            text,
            font,
            paint_command,
            left,
            top,
            right,
            bottom,
            color,
        }
    }
}

impl<'text, 'font> Drawable for DrawText<'text, 'font> {
    fn execute(&self, canvas: &Canvas) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true).set_color4f(self.color, None);
        let baseline = self.top - self.font.metrics().1.ascent;
        canvas.draw_str(
            self.text,
            Point::new(self.left, baseline),
            self.font,
            &paint,
        );
    }
}
