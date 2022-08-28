use pulldown_cmark::{Event, Options, Parser, Tag};

fn main() {
    let markdown_input = concat!("# My Heading\n", "[My Link](http://example.com)\n", "\n",);
    let mut links: Vec<String> = Vec::new();

    let parser = Parser::new_ext(markdown_input, Options::all());
    for event in parser {
        match &event {
            Event::Start(tag) => match tag {
                Tag::Link(_, url, _) => links.push(format!("{}", url)),
                _ => (),
            },
            _ => (),
        };
    }

    println!("{:?}", links);
}
