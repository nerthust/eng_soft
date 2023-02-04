use kingdom_hearts::app::{App, get_army};
use kingdom_hearts::kingdom_type::KingdomType; 
use kingdom_hearts::factory_maker::make_factory;

fn main() {
    let mut app = App::new();
    let kingdom_type = KingdomType::value_of("ORC");
    let factory = make_factory(kingdom_type.unwrap());
    app.create_kingdom(factory.as_ref());
    println!("{:?}", app);

    let army = app.get_army().unwrap();
    let army_fac = get_army(factory.as_ref());

    println!("{:?}", army);
    println!("{:?}", army_fac);
}
