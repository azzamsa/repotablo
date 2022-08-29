use async_graphql::SimpleObject;

use crate::parser::entities;

#[derive(Debug, SimpleObject)]
pub struct ParserPayload {
    pub links: Vec<String>,
}

impl From<entities::Parser> for ParserPayload {
    fn from(parser: entities::Parser) -> Self {
        Self {
            links: parser.links,
        }
    }
}
