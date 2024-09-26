use super::{Drawable, PaintCommand};
use skia_safe::{Canvas, Color4f, Paint, Path, Point, Rect};

struct DrawLine {
    paint_command: PaintCommand,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    color: Color4f,
    stroke_width: f32,
}

impl DrawLine {
    fn new(
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        color: Color4f,
        stroke_width: f32,
    ) -> Self {
        let paint_command = PaintCommand::new(Rect::new(left, top, right, bottom));
        Self {
            paint_command,
            left,
            top,
            right,
            bottom,
            color,
            stroke_width,
        }
    }
}

impl Drawable for DrawLine {
    fn execute(&self, canvas: &Canvas) {
        let mut path = Path::new();
        path.move_to(Point::new(self.left, self.top))
            .line_to(Point::new(self.right, self.bottom));
        let mut paint = Paint::default();
        paint
            .set_stroke_width(self.stroke_width)
            .set_color4f(self.color, None);
        canvas.draw_path(&path, &paint);
    }
}
