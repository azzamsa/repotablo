use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use crate::{
    health::resolver::HealthQuery, meta::resolver::MetaQuery, parser::resolver::ParserQuery,
};

#[derive(MergedObject, Default)]
pub struct Query(MetaQuery, HealthQuery, ParserQuery);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
