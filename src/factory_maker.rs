use crate::kingdom_type::KingdomType;
use crate::factory::{
    kingdom_factory::KingdomFactory,
    orc_kingdom_factory::OrcKingdomFactory,
    elf_kingdom_factory::ElfKingdomFactory,
};

pub fn make_factory(k_type: KingdomType) -> Box<dyn KingdomFactory> {
    match k_type {
        KingdomType::ELF => Box::new(ElfKingdomFactory::new()),
        KingdomType::ORC => Box::new(OrcKingdomFactory::new()),
    }
}
