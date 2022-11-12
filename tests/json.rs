use twyne::{json, schema};

mod utils;

#[test]
fn test_parse_file() {
    let game: schema::Game = json::parse_file(&utils::LOBBY_GAME).unwrap();
    utils::check_all_data(&game);
}

#[test]
fn test_parse_str() {
    let game: schema::Game = json::parse_str(&utils::lobby_game_data_str()).unwrap();
    utils::check_all_data(&game);
}

#[test]
fn test_parse_bytes() {
    let game: schema::Game = json::parse_bytes(&utils::lobby_game_data_bytes()).unwrap();
    utils::check_all_data(&game);
}
