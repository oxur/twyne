use twyne::harlowe::json::{Game, parse_file, parse_str, parse_bytes};

use super::utils;

#[test]
fn test_parse_file() {
    let game: Game = parse_file(&utils::LOBBY_GAME).unwrap();
    utils::check_all_data(&game);
}

#[test]
fn test_parse_str() {
    let game: Game = parse_str(&utils::lobby_game_data_str()).unwrap();
    utils::check_all_data(&game);
}

#[test]
fn test_parse_bytes() {
    let game: Game = parse_bytes(&utils::lobby_game_data_bytes()).unwrap();
    utils::check_all_data(&game);
}
