use std::sync::Arc;
use crate::factory::kingdom_factory::KingdomFactory;
use crate::army::army::Army;
use crate::castle::castle::Castle;
use crate::king::king::King;
use crate::king::orc_king::OrcKing;
use crate::castle::orc_castle::OrcCastle;
use crate::army::orc_army::OrcArmy;

#[derive(Default)]
pub struct OrcKingdomFactory;

thread_local! {
    static ORC_K_FACTORY_POOL: Arc<OrcKingdomFactory> = Arc::new(Default::default());
}

impl OrcKingdomFactory {
    pub fn new() -> Arc<OrcKingdomFactory> {
        ORC_K_FACTORY_POOL.with(|orc_k_factory_pool| orc_k_factory_pool.clone())
    }
}

impl KingdomFactory for Arc<OrcKingdomFactory> {
    fn create_army(&self) -> Box<dyn Army> {
        Box::new(OrcArmy::new())
    }

    fn create_castle(&self) -> Box<dyn Castle> {
        Box::new(OrcCastle::new())
    }

    fn create_king(&self) -> Box<dyn King> {
        Box::new(OrcKing::new())
    }
}
