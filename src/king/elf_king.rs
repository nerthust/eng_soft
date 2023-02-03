use std::sync::{Arc, Mutex};
use crate::king::king::King;


#[derive(Default, Debug)]
pub struct ElfKing {
    pub(crate) description: Mutex<String>,
}

thread_local! {
    static ELF_KING_POOL: Arc<ElfKing> = Arc::new(
        ElfKing {
            description: Mutex::new("He is an expertx and string magician".to_string()),
        }
    );
}

impl ElfKing {
    pub fn new() -> Arc<ElfKing> {
        ELF_KING_POOL.with(|elf_king_pool| elf_king_pool.clone())
    }
}

impl King for Arc<ElfKing> {
    fn get_description(&self) -> String {
        let elf_king = ElfKing::new();
        let desc = elf_king.description.try_lock().unwrap();

        desc.to_string()
    }
}
