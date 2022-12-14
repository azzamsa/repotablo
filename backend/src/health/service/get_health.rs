use super::Service;
use crate::{errors::Error, health::entities::Health};

impl Service {
    pub async fn get_health(&self) -> Result<Health, Error> {
        let health = Health {
            status: "running".to_string(),
        };
        Ok(health)
    }
}
