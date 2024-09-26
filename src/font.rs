use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use skia_safe::{utils::CustomTypefaceBuilder, Font, Typeface};

lazy_static! {
    pub static ref FONTS: Mutex<HashMap<(FontStyle, FontWeight), Typeface>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum FontStyle {
    Roman,
    Italic,
}

impl From<&String> for FontStyle {
    fn from(value: &String) -> Self {
        use FontStyle::*;
        match value.as_str() {
            "italic" => Italic,
            _ => Roman,
        }
    }
}

impl From<&str> for FontStyle {
    fn from(value: &str) -> Self {
        use FontStyle::*;
        match value {
            "italic" => Italic,
            _ => Roman,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum FontWeight {
    Normal,
    Bold,
}

impl From<&String> for FontWeight {
    fn from(value: &String) -> Self {
        use FontWeight::*;
        match value.as_str() {
            "bold" => Bold,
            _ => Normal,
        }
    }
}

impl From<&str> for FontWeight {
    fn from(value: &str) -> Self {
        use FontWeight::*;
        match value {
            "bold" => Bold,
            _ => Normal,
        }
    }
}

pub fn get_font(size: f32, weight: FontWeight, style: FontStyle) -> Font {
    use FontStyle::*;
    use FontWeight::*;

    let key = (style, weight);
    let mut typefaces = FONTS.lock().unwrap();

    if !typefaces.contains_key(&key) {
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
        let typeface = CustomTypefaceBuilder::new()
            .set_font_style(style_info)
            .detach()
            .unwrap();

        typefaces.insert(key, typeface);
    }

    Font::from_typeface(typefaces.get(&key).unwrap(), size)
}

pub fn linespace(font: &Font) -> f32 {
    let metrics = font.metrics().1;
    metrics.descent - metrics.ascent
}
