use std::sync::{Arc, Mutex};
use crate::castle::castle::Castle;

#[derive(Default, Debug)]
pub struct OrcCastle {
    pub(crate) description: Mutex<String>,
}

thread_local! {
    static ORC_CASTLE_POOL: Arc<OrcCastle> = Arc::new(
        OrcCastle {
            description: Mutex::new("orc, a mythical creature of horrid form or aspect.".to_string()),
        }
    );
}

impl OrcCastle {
    pub fn new() -> Arc<OrcCastle> {
        ORC_CASTLE_POOL.with(|orc_castle_pool| orc_castle_pool.clone())
    }
}

impl Castle for Arc<OrcCastle> {
    fn get_description(&self) -> String {
        let orc_castle = OrcCastle::new();
        let desc = orc_castle.description.try_lock().unwrap();

        desc.to_string()
    }
}
