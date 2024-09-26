use html_parser::HtmlParser;

mod accessibility;
mod browser;
mod css_parser;
mod html_parser;
mod paint_command;

extern crate sdl2;

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use std::time::Duration;

// pub fn main() {
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();

//     let window = video_subsystem
//         .window("Browser", WIDTH, HEIGHT)
//         .position_centered()
//         .allow_highdpi()
//         .resizable()
//         .build()
//         .unwrap();

//     let mut canvas = window.into_canvas().build().unwrap();

//     canvas.set_draw_color(Color::RGB(0, 255, 255));
//     canvas.clear();
//     canvas.present();
//     let mut event_pump = sdl_context.event_pump().unwrap();
//     let mut i = 0;
//     'running: loop {
//         i = (i + 1) % 255;
//         canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
//         canvas.clear();
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => break 'running,
//                 _ => {}
//             }
//         }
//         // The rest of the game loop goes here...

//         canvas.present();
//         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//     }
// }

fn main() {
    let html = r#"
<header>
<h1 class="title">Laying Out Pages</h1>
<a href="https://twitter.com/browserbook">Twitter</a>
<a href="https://browserbook.substack.com/">Blog</a>
<a href="https://patreon.com/browserengineering">Patreon</a>
<a href="https://github.com/browserengineering/book/discussions">Discussions</a>
</header>

<nav class="links">
  <a href="index.html" title="Table of Contents">Web Browser Engineering</a>
  <a rel="prev" title="Previous chapter" href="html.html">&lt;</a>
  <a rel="next" title="Next chapter" href="styles.html">&gt;</a>
</nav>"#;

    let a = HtmlParser::new(html).parse();
    a.paint_tree(0);
}
