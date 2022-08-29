use async_graphql::SimpleObject;
use serde::Serialize;
use utoipa::ToSchema;

use crate::health::entities;

// Used in conjuction with JSON response
#[derive(Debug, SimpleObject, Serialize, ToSchema)]
pub struct HealthPayload {
    pub status: String,
}

impl From<entities::Health> for HealthPayload {
    fn from(health: entities::Health) -> Self {
        Self {
            status: health.status,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub data: HealthPayload,
}
