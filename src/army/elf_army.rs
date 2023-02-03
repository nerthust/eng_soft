use std::sync::{Arc, Mutex};
use crate::army::army::Army;

#[derive(Default)]
pub struct ElfArmy {
    pub(crate) description: Mutex<String>,
}

thread_local! {
    static ELF_ARMY_POOL: Arc<ElfArmy> = Arc::new(
        ElfArmy {
            description: Mutex::new("An elf is a type of humanoid supernatural".to_string()),
        }
    );
}

impl ElfArmy {
    pub fn new() -> Arc<ElfArmy> {
        ELF_ARMY_POOL.with(|elf_army_pool| elf_army_pool.clone())
    }
}

impl Army for Arc<ElfArmy> {
    fn get_description(&self) -> String {
        let elf_army = ElfArmy::new();
        let desc = elf_army.description.try_lock().unwrap();

        desc.to_string()
    }
}
