use std::collections::HashMap;

use lazy_static::lazy_static;
use skia_safe::Font;

lazy_static! {
    static ref FONTS: HashMap<(FontStyle, FontWeight), Font> = HashMap::new();
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum FontStyle {
    Roman,
    Italic,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum FontWeight {
    Normal,
    Bold,
}

fn get_font(size: f32, weight: FontWeight, style: FontStyle) {
    use FontStyle::*;
    use FontWeight::*;

    let key = (style, weight);

    if !FONTS.contains_key(&key) {
        let skia_weight;
        let skia_style;
        let skia_width = skia_safe::FontStyle::normal().width();

        match style {
            Italic => skia_style = skia_safe::FontStyle::italic().slant(),
            _ => skia_style = skia_safe::FontStyle::normal().slant(),
        }
        match weight {
            Bold => skia_weight = skia_safe::FontStyle::bold().weight(),
            _ => skia_weight = skia_safe::FontStyle::normal().weight(),
        }

        let style_info = skia_safe::FontStyle::new(skia_weight, skia_width, skia_style);
        // let font = skia_safe::Typeface::
    }
}
