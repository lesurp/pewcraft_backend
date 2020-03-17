use pewcraft_common::game_definition::map::CellId;
use pewcraft_common::game_definition::map::TeamId;
use pewcraft_common::game_definition::GameDefinition;
use pewcraft_common::id::MapBuilder;
use pewcraft_common::io::character::{Character, CharacterId, CharacterMapBuilder};
use pewcraft_common::io::Action;
use pewcraft_common::io::GameState;
use std::collections::HashMap;

mod game_definition_loader;

struct Games {
    // games are identified with a randomly generated string...
    game_states: HashMap<String, GameServerRepresentation>,
}

struct GameServerRepresentation {
    game_state: GameState,
    // Users "login" with a randomly generated string...
    login_to_character_id: HashMap<String, CharacterId>,
}

struct WiredAction {
    login: String,
    action: Action,
}

fn process_action(
    game_definition: &GameDefinition,
    game_server_representation: &mut GameServerRepresentation,
    wired_action: WiredAction,
) -> bool {
    let character_id = game_server_representation
        .login_to_character_id
        .get(&wired_action.login);
    let character_id = if let Some(id) = character_id {
        *id
    } else {
        return false;
    };

    let to_play_id = game_server_representation
        .game_state
        .player_to_play()
        .expect(
        "A player should always come next - we need to either start a new turn or finish the game!",
    );

    let is_correct_id = to_play_id == character_id;
    if !is_correct_id {
        return false;
    }

    game_server_representation
        .game_state
        .next_action(game_definition, wired_action.action);

    if game_server_representation
        .game_state
        .player_to_play()
        .is_none()
    {
        game_server_representation
            .game_state
            .new_turn(game_definition);
    }
    true
}

fn main() {
    env_logger::init();
    let game_definition = game_definition_loader::load("./data");
    let (map_id, map) = game_definition.maps.iter().next().unwrap();
    let (id, class) = game_definition.classes.iter().next().unwrap();

    let mut character_map_builder = CharacterMapBuilder::new(&map.teams, 1);
    character_map_builder.add(Character::new(
        *id,
        CellId::new(0),
        class,
        "Bob",
        TeamId::new(0),
    ));
    character_map_builder.add(Character::new(
        *id,
        CellId::new(1),
        class,
        "Alice",
        TeamId::new(1),
    ));

    let game_state = GameState::new(&game_definition, character_map_builder.build().unwrap(), *map_id);
}
