mod get_meta;

#[derive(Debug)]
pub struct Service;

impl Service {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}
