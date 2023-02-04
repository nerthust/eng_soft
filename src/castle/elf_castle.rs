use std::sync::{Arc, Mutex};
use crate::castle::castle::Castle;

#[derive(Default, Debug)]
pub struct ElfCastle {
    pub(crate) description: Mutex<String>,
}

thread_local! {
    static ELF_CASTLE_POOL: Arc<ElfCastle> = Arc::new(
        ElfCastle {
            description: Mutex::new("orc, a mythical creature of horrid form or aspect.".to_string()),
        }
    );
}

impl ElfCastle {
    pub fn new() -> Arc<ElfCastle> {
        ELF_CASTLE_POOL.with(|elf_castle_pool| elf_castle_pool.clone())
    }
}

impl Castle for Arc<ElfCastle> {
    fn get_description(&self) -> String {
        let elf_castle = ElfCastle::new();
        let desc = elf_castle.description.try_lock().unwrap();

        desc.to_string()
    }
}
