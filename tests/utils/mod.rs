use std::fs;
use std::path::Path;
use twyne::schema;

pub const LOBBY_GAME: &str = "resources/samples/lobby-game.json";

#[allow(dead_code)]
pub fn lobby_game_data_str() -> String {
    fs::read_to_string(Path::new(LOBBY_GAME)).unwrap()
}

#[allow(dead_code)]
pub fn lobby_game_data_bytes() -> Vec<u8> {
    fs::read(Path::new(LOBBY_GAME)).unwrap()
}

#[allow(dead_code)]
pub fn lobby_game() -> schema::Game {
    let data = lobby_game_data_str();
    let g: schema::Game = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(error) => panic!("Tests can't run without the sample file: {:?}", error),
    };
    g
}

#[allow(dead_code)]
pub fn check_all_data(game: &schema::Game) {
    check_game_data(game);
    check_passages_data(game);
    check_links_data(game);
}

#[allow(dead_code)]
pub fn check_game_data(game: &schema::Game) {
    assert_eq!(game.name, "Lobby");
}

#[allow(dead_code)]
pub fn check_passages_data(game: &schema::Game) {
    assert_eq!(game.passages.len(), 6);
    let passage: schema::Passage = game.passages[0].clone();
    let text: String = passage.text.chars().take(31).collect();
    assert_eq!(text, "You are in a lobby -- THE lobby");
}

#[allow(dead_code)]
pub fn check_links_data(game: &schema::Game) {
    let passage: schema::Passage = game.passages[0].clone();
    let link: schema::Link = passage.links[0].clone();
    assert_eq!(link.link_text, "Create Character");
    assert_eq!(link.passage_name, "Create Character");
}
