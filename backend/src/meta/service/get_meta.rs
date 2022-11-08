use super::Service;
use crate::{errors::Error, meta::entities::Meta};

impl Service {
    pub async fn get_meta(&self) -> Result<Meta, Error> {
        let meta = Meta {
            build: option_env!("VCS_REVISION").unwrap_or("unknown").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        Ok(meta)
    }
}
