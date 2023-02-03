use strum::{IntoEnumIterator, EnumIter};

#[derive(Debug, EnumIter)]
pub enum KingdomType {
    ELF,
    ORC,
}

impl KingdomType {
    pub fn value_of(k_type: &str) -> Option<KingdomType> {
        match k_type {
            "ELF" => Some(KingdomType::ELF),
            "ORC" => Some(KingdomType::ORC),
            _ => None,
        }
    }

    pub fn values() -> Vec<KingdomType> {
        KingdomType::iter().collect::<Vec<_>>()
    }
}
