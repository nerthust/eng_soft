use std::sync::{Arc, Mutex};
use crate::army::army::Army;

#[derive(Default)]
pub struct OrcArmy {
    pub(crate) description: Mutex<String>,
}

thread_local! {
    static ORC_ARMY_POOL: Arc<OrcArmy> = Arc::new(
        OrcArmy {
            description: Mutex::new("orc, a mythical creature of horrid form or aspect.".to_string()),
        }
    );
}

impl OrcArmy {
    pub fn new() -> Arc<OrcArmy> {
        ORC_ARMY_POOL.with(|orc_army_pool| orc_army_pool.clone())
    }
}

impl Army for Arc<OrcArmy> {
    fn get_description(&self) -> String {
        let orc_army = OrcArmy::new();
        let desc = orc_army.description.try_lock().unwrap();

        desc.to_string()
    }
}
