use pulldown_cmark::{Event, Options, Parser as CmarkParser, Tag};

use super::Service;
use crate::{parser::entities::Parser, Error};

impl Service {
    pub async fn parse_link(&self, content: String) -> Result<Parser, Error> {
        let mut links: Vec<String> = Vec::new();

        let parser = CmarkParser::new_ext(&content, Options::all());
        for event in parser {
            match &event {
                Event::Start(tag) => match tag {
                    Tag::Link(_, url, _) => links.push(format!("{}", url)),
                    _ => (),
                },
                _ => (),
            };
        }
        Ok(Parser { links })
    }
}
