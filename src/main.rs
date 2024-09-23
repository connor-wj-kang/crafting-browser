use std::rc::Rc;

use browser::{
    css_parser::{style, CssParser},
    html_parser::HtmlParser,
    layout_new::{DocumentLayout, Drawable, Layout},
};

fn paint_tree<'html>(
    layout_object: Rc<dyn Layout<'html> + 'html>,
) -> Vec<Box<dyn Drawable<'html> + 'html>> {
    let mut display_list: Vec<Box<dyn Drawable<'html> + 'html>> = Vec::new();
    display_list.extend(layout_object.clone().paint().into_iter());
    layout_object.children().iter().for_each(|child| {
        let a = paint_tree(child.clone() as Rc<dyn Layout<'html>>);
        display_list.extend(a.into_iter());
    });
    display_list
}

fn main() {
    let nodes = HtmlParser::new("<a>Block Layout</a><p>So far, we’ve focused on text layout—and text is laid out horizontally in lines.<span><span>In European languages, at least!</span></span> But web pages are really constructed out of larger blocks, like headings, paragraphs, and menus, that stack vertically one after another. We need to add support for this kind of layout to our browser, and the way we’re going to do that involves expanding on the layout tree we’ve already built.</p><p>The core idea is that we’ll have a whole tree of <code>BlockLayout</code> objects (with a <code>DocumentLayout</code> at the root). Some will represent leaf blocks that contain text, and they’ll lay out their contents the way we’ve already implemented. But there will also be new, intermediate <code>BlockLayout</code>s with <code>BlockLayout</code> children, and they will stack their children vertically. (An example is shown in Figure 1.)</p>").parse();

    let default_style_sheet = CssParser::new(
        r#"
        pre { background-color: gray; }
        a { color: blue; }
        i { font-style: italic; }
        b { font-weight: bold; }
        small { font-size: 90%; }
        big { font-size: 110%; }
    "#,
    )
    .parse();

    style(nodes.clone(), &default_style_sheet);
    let document = DocumentLayout::new(nodes.clone());
    document.clone().layout();
    let a = paint_tree(document.clone());
    a.iter().for_each(|cmd| {
        cmd.execute(0.0);
    });
}
