use crate::kingdom_type::KingdomType;
use crate::factory::{
    kingdom_factory::KingdomFactory,
    elf_kingdom_factory::ElfKingdomFactory,
    orc_kingdom_factory::OrcKingdomFactory,
};

pub fn make_factory(k_type: KingdomType) -> Box<dyn KingdomFactory> {
    match k_type {
        KingdomType::ELF => Box::new(ElfKingdomFactory::new()),
        KingdomType::ORC => Box::new(OrcKingdomFactory::new()),
    }
}
