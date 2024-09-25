use sdl2::libc::NOEXPR;
use skia_safe::{Canvas, Color4f, ColorType, Font, Paint, Path, Point, RRect, Rect};

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
