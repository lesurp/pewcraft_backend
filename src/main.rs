use pewcraft_common::game_definition::map::CellId;
use pewcraft_common::id::MapBuilder;
use pewcraft_common::io::character::Character;
use pewcraft_common::io::GameState;

mod game_definition_loader;

fn main() {
    env_logger::init();
    let game_definition = game_definition_loader::load("./data");
    let map = *game_definition.maps.iter().next().unwrap().0;
    let (id, class) = game_definition.classes.iter().next().unwrap();

    let character_1 = Character::new(*id, CellId::new(0), class, "Bob");
    let character_2 = Character::new(*id, CellId::new(1), class, "Alice");

    let mut character_builder = MapBuilder::new();
    character_builder.add(character_1);
    character_builder.add(character_2);

    let game_state = GameState::new(&game_definition, character_builder.build(), map);
}
