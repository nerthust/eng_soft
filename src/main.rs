use kingdom_hearts::app::{App, Get};
use kingdom_hearts::factory_maker::make_factory;
use kingdom_hearts::kingdom_type::KingdomType; 
use kingdom_hearts::castle::orc_castle::OrcCastle;

fn main() {
    let mut app = App::new();
    let kingdom_type = KingdomType::value_of("ELF");
    let factory = make_factory(kingdom_type.unwrap());
    let kingdom = app.create_kingdom(factory.as_ref());

    let army = app.get_army().unwrap();
    let orc_castle = OrcCastle::new();
    app.set_castle(Some(Box::new(orc_castle)));
}
