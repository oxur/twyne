use twyne::twine::{Game};

pub fn lobby_game() -> Game {
    let data = include_str!("../../resources/samples/lobby-game.json");
    let g: Game = match serde_json::from_str(data) {
        Ok(parsed) => parsed,
        Err(error) => panic!("Tests can't run without the sample file: {:?}", error),
    };
    g
}
