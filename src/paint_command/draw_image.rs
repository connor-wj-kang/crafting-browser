use super::{Drawable, PaintCommand};
use skia_safe::{
    Canvas, CubicResampler, FilterMode, Image, MipmapMode, Point, Rect, SamplingOptions,
};
use std::borrow::BorrowMut;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum ImageRendering {
    Auto,
    HighQuality,
    CrispEdges,
}

struct DrawImage {
    paint_command: PaintCommand,
    image: Image,
    quality: SamplingOptions,
}

impl DrawImage {
    fn parse_image_rendering(quality: ImageRendering) -> SamplingOptions {
        use ImageRendering::*;
        match quality {
            HighQuality => {
                let mut sampling = SamplingOptions::default();
                sampling.borrow_mut().cubic = CubicResampler::mitchell();
                sampling
            }
            CrispEdges => SamplingOptions::new(FilterMode::Nearest, MipmapMode::None),
            _ => SamplingOptions::new(FilterMode::Nearest, MipmapMode::Linear),
        }
    }

    fn new(image: Image, rect: Rect, quality: ImageRendering) -> Self {
        Self {
            paint_command: PaintCommand::new(rect),
            image,
            quality: Self::parse_image_rendering(quality),
        }
    }
}

impl Drawable for DrawImage {
    fn execute(&self, canvas: &Canvas) {
        canvas.draw_image_with_sampling_options(
            &self.image,
            Point::new(self.paint_command.rect.left, self.paint_command.rect.top),
            self.quality,
            None,
        );
    }
}
