use std::sync::Arc;

use crate::army::army::Army;
use crate::castle::castle::Castle;
use crate::king::king::King;
use crate::army::elf_army::ElfArmy;
use crate::castle::elf_castle::ElfCastle;
use crate::king::elf_king::ElfKing;
use crate::factory::kingdom_factory::KingdomFactory;

#[derive(Default, Debug)]
pub struct ElfKingdomFactory;

thread_local! {
    static ELF_K_FACTORY_POOL: Arc<ElfKingdomFactory> = Arc::new(Default::default());
}

impl ElfKingdomFactory {
    pub fn new() -> Arc<ElfKingdomFactory> {
        ELF_K_FACTORY_POOL.with(|elf_k_factory_pool| elf_k_factory_pool.clone())
    }
}

impl KingdomFactory for Arc<ElfKingdomFactory> {
    fn create_army(&self) -> Box<dyn Army> {
        Box::new(ElfArmy::new())
    }

    fn create_castle(&self) -> Box<dyn Castle> {
        Box::new(ElfCastle::new())
    }

    fn create_king(&self) -> Box<dyn King> {
        Box::new(ElfKing::new())
    }
}
