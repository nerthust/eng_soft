use crate::factory::kingdom_factory::KingdomFactory;
use crate::army::army::Army;
use crate::castle::castle::Castle;
use crate::king::king::King;

#[derive(Default, Debug)]
pub struct App {
    army: Option<Box<dyn Army>>,
    castle: Option<Box<dyn Castle>>,
    king: Option<Box<dyn King>>,
}

impl App {
    pub fn new() -> App {
        App{
            army: None,
            castle: None,
            king: None,
        }
    }

    pub fn create_kingdom(&mut self, king_fac: &dyn KingdomFactory) {
        self.army = Some(king_fac.create_army());
    }

    pub fn get_army(&self) -> Option<&Box<dyn Army>> {
        self.army.as_ref()
    }

    pub fn get_castle(&self) -> Option<&Box<dyn Castle>> {
        self.castle.as_ref()
    }

    pub fn get_king(&self) -> Option<&Box<dyn King>> {
        self.king.as_ref()
    }

    #[allow(dead_code)]
    fn set_army(&mut self, army: Option<Box<dyn Army>>) {
        self.army = army;
    }

    #[allow(dead_code)]
    fn set_castle(&mut self, castle: Option<Box<dyn Castle>>) {
        self.castle = castle;
    }

    #[allow(dead_code)]
    fn set_king(&mut self, king: Option<Box<dyn King>>) {
        self.king = king;
    }
}

pub fn get_army(kd: &dyn KingdomFactory) -> Box<dyn Army> {
    kd.create_army()
}

pub fn get_castle(kd: &dyn KingdomFactory) -> Box<dyn Castle> {
    kd.create_castle()
}

pub fn get_king(kd: &dyn KingdomFactory) -> Box<dyn King> {
    kd.create_king()
}
