use std::fmt::{write, Display};

use wasm_bindgen::JsValue;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

use crate::token::Token;

#[derive(Debug, Clone, Copy)]
pub enum FontStyle {
    Normal,
    Italic,
}

impl Display for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Italic => write!(f, "italic"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FontWeight {
    Normal,
    Bold,
}

impl Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Bold => write!(f, "bold"),
        }
    }
}

pub struct DisplayInfo<'text> {
    pub x: f64,
    pub y: f64,
    pub text: &'text str,
    pub font_size: f64,
    pub font_style: FontStyle,
    pub font_weight: FontWeight,
}

pub struct Layout<'text, 'context, 'canvas> {
    display_list: Vec<DisplayInfo<'text>>,
    line_buffer: Vec<DisplayInfo<'text>>,
    current_font_size: f64,
    current_font_weight: FontWeight,
    current_font_style: FontStyle,
    cursor_x: f64,
    cursor_y: f64,
    context: &'context CanvasRenderingContext2d,
    canvas: &'canvas HtmlCanvasElement,
}

impl<'text, 'context, 'canvas> Layout<'text, 'context, 'canvas> {
    pub fn new(
        context: &'context CanvasRenderingContext2d,
        canvas: &'canvas HtmlCanvasElement,
    ) -> Self {
        Self {
            display_list: Vec::new(),
            line_buffer: Vec::new(),
            current_font_size: 20.0,
            current_font_weight: FontWeight::Normal,
            current_font_style: FontStyle::Normal,
            cursor_x: 0.0,
            cursor_y: 20.0,
            context,
            canvas,
        }
    }

    pub fn calc_display_list(mut self, tokens: &'text [Token]) -> Vec<DisplayInfo<'text>> {
        tokens.iter().for_each(|token| self.consume_token(token));
        self.flush_line_buffer();
        self.display_list
    }

    fn consume_token(&mut self, token: &'text Token) {
        match token {
            Token::Text(text) => {
                self.context.set_font(
                    format!(
                        "{} {} {}px serif",
                        self.current_font_style, self.current_font_weight, self.current_font_size
                    )
                    .as_str(),
                );

                let space_width = self.context.measure_text(" ").unwrap().width();

                text.split_whitespace().into_iter().for_each(|word| {
                    let word_width = self.context.measure_text(word).unwrap().width();
                    self.line_buffer.push(DisplayInfo {
                        x: self.cursor_x,
                        y: 0.0,
                        text: word,
                        font_style: self.current_font_style,
                        font_size: self.current_font_size,
                        font_weight: self.current_font_weight,
                    });

                    if self.cursor_x + word_width > self.canvas.width() as f64 - 40.0 {
                        self.flush_line_buffer();
                    } else {
                        self.cursor_x += word_width + space_width;
                    }
                });
            }
            Token::Tag(tag) => match *tag {
                "i" => self.current_font_style = FontStyle::Italic,
                "/i" => self.current_font_style = FontStyle::Normal,
                "b" => self.current_font_weight = FontWeight::Bold,
                "/b" => self.current_font_weight = FontWeight::Normal,
                "small" => self.current_font_size -= 4.0,
                "/small" => self.current_font_size += 4.0,
                "big" => self.current_font_size += 4.0,
                "/big" => self.current_font_size -= 4.0,
                _ => {}
            },
        }
    }

    fn flush_line_buffer(&mut self) {
        if self.line_buffer.is_empty() {
            return;
        }

        let (max_ascent, max_descent) = self
            .line_buffer
            .iter()
            .map(|display_info| {
                self.context.set_font(
                    format!(
                        "{} {} {}px serif",
                        display_info.font_style, display_info.font_weight, display_info.font_size
                    )
                    .as_str(),
                );

                let font_metric = self.context.measure_text(display_info.text).unwrap();
                (
                    font_metric.font_bounding_box_ascent(),
                    font_metric.font_bounding_box_ascent(),
                )
            })
            .fold(
                (f64::NEG_INFINITY, f64::NEG_INFINITY),
                |(ascent, descent), (x, y)| (ascent.max(x), descent.max(y)),
            );

        let base_line = self.cursor_y + 1.25 * max_ascent;
        self.line_buffer.iter().for_each(|display_info| {
            self.display_list.push(DisplayInfo {
                y: base_line,
                ..*display_info
            });
        });

        self.cursor_x = 0.0;
        self.cursor_y = base_line + 1.25 * max_descent;
        self.line_buffer.clear();
    }
}
