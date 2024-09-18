use html_parser::HtmlParser;

mod html_parser;

fn main() {
    let nodes = HtmlParser::new("<h1>Block Layout</h1><p>So far, we’ve focused on text layout—and text is laid out horizontally in lines.<span><span>In European languages, at least!</span></span> But web pages are really constructed out of larger blocks, like headings, paragraphs, and menus, that stack vertically one after another. We need to add support for this kind of layout to our browser, and the way we’re going to do that involves expanding on the layout tree we’ve already built.</p><p>The core idea is that we’ll have a whole tree of <code>BlockLayout</code> objects (with a <code>DocumentLayout</code> at the root). Some will represent leaf blocks that contain text, and they’ll lay out their contents the way we’ve already implemented. But there will also be new, intermediate <code>BlockLayout</code>s with <code>BlockLayout</code> children, and they will stack their children vertically. (An example is shown in Figure 1.)</p>").parse();

    println!("{nodes}");
}
