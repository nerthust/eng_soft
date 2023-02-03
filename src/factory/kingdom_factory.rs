use crate::army::army::Army;
use crate::castle::castle::Castle;
use crate::king::king::King;

pub trait KingdomFactory {
    fn create_army(&self) -> Box<dyn Army>;
    fn create_castle(&self) -> Box<dyn Castle>;
    fn create_king(&self) -> Box<dyn King>;
}
