use std::sync::Arc;

use async_graphql::{Context, Error, FieldResult, Object};

use super::model::ParserPayload;
use crate::context::ServerContext;

#[derive(Default)]
pub struct ParserQuery;

#[Object]
impl ParserQuery {
    pub async fn parse(&self, ctx: &Context<'_>, content: String) -> FieldResult<ParserPayload> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.parser_service.parse_link(content).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}
