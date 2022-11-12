use twyne::schema;

mod utils;

#[test]
fn test_game() {
    let game: schema::Game = utils::lobby_game();
    utils::check_game_data(&game);
}

#[test]
fn test_link() {
    let game: schema::Game = utils::lobby_game();
    utils::check_links_data(&game);

}

#[test]
fn test_passage() {
    let game: schema::Game = utils::lobby_game();
    utils::check_links_data(&game);
}
