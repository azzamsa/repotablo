use async_graphql::SimpleObject;

use crate::parser::entities;

#[derive(Debug, SimpleObject)]
pub struct Parser {
    pub links: Vec<String>,
}

impl From<entities::Parser> for Parser {
    fn from(parser: entities::Parser) -> Self {
        Self {
            links: parser.links,
        }
    }
}
