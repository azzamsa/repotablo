use pulldown_cmark::{Event, Options, Parser as CmarkParser, Tag};

use super::Service;
use crate::{parser::entities::Parser, Error};

impl Service {
    pub async fn parse_link(&self, content: String) -> Result<Parser, Error> {
        let mut links: Vec<String> = Vec::new();

        let parser = CmarkParser::new_ext(&content, Options::all());
        for event in parser {
            if let Event::Start(Tag::Link(_, url, _)) = &event {
                links.push(format!("{}", url))
            };
        }
        Ok(Parser { links })
    }
}
