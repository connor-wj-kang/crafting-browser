use html_parser::HtmlParser;

mod html_parser;

fn main() {
    let a = HtmlParser::new(
        "<script>dsd</script><p>4-4<em>Quoted attributes</em>. Quoted attributes can contain spaces and right angle brackets. Fix the lexer so that this is supported properly. Hint: the current lexer is a finite state machine, with two states (determined by <code>in_tag</code>). You’ll need more states.</p>",
    )
    .parse();

    println!("{}", a);
}
