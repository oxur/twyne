use twyne::harlow::json::{Game};

use super::utils;

#[test]
fn test_game() {
    let game: Game = utils::lobby_game();
    utils::check_game_data(&game);
}

#[test]
fn test_link() {
    let game: Game = utils::lobby_game();
    utils::check_links_data(&game);

}

#[test]
fn test_passage() {
    let game: Game = utils::lobby_game();
    utils::check_links_data(&game);
}
