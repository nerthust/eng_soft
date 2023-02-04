use std::sync::{Arc, Mutex};
use crate::king::king::King;

#[derive(Default, Debug)]
pub struct OrcKing {
    pub(crate) description: Mutex<String>,
}

thread_local! {
    static ORC_KING_POOL: Arc<OrcKing> = Arc::new(
        OrcKing {
            description: Mutex::new("He is the taller of all, him height is 10 meters".to_string()),
        }
    );
}

impl OrcKing {
    pub fn new() -> Arc<OrcKing> {
        ORC_KING_POOL.with(|orc_king_pool| orc_king_pool.clone())
    }
}

impl King for Arc<OrcKing> {
    fn get_description(&self) -> String {
        let orc_king = OrcKing::new();
        let desc = orc_king.description.try_lock().unwrap();

        desc.to_string()
    }
}
