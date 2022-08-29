use std::sync::Arc;

use crate::{health, meta, parser};

#[derive(Clone)]
pub struct ServerContext {
    pub meta_service: Arc<meta::Service>,
    pub health_service: Arc<health::Service>,
    pub parser_service: Arc<parser::Service>,
}
