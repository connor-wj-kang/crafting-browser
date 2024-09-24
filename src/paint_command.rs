use skia_safe::{Canvas, Color4f, ColorType, Paint, Path, Point, RRect, Rect};

trait Drawable {
    fn execute(&self, canvas: &Canvas) {}
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

struct DrawText<'text> {
    text: &'text str,
    paint_command: PaintCommand,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    color: Color4f,
    stroke_width: f32,
}

impl<'text> DrawText<'text> {
    fn new(left: f32, top: f32, text: &'text str, color: Color4f) -> Self {}
}
