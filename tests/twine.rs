use twyne::twine::{Game, Link, Passage};

mod utils;

#[test]
fn test_game() {
    let game: Game = utils::lobby_game();
    assert_eq!(game.name, "Lobby");
    assert_eq!(game.passages.len(), 6);
}

#[test]
fn test_link() {
    let link: Link = utils::lobby_game().passages[0].links[0].clone();
    assert_eq!(link.link_text, "Create Character");
    assert_eq!(link.passage_name, "Create Character");

}

#[test]
fn test_passage() {
    let passage: Passage = utils::lobby_game().passages[0].clone();
    let text: String = passage.text.chars().take(31).collect();
    assert_eq!(text, "You are in a lobby -- THE lobby");
}
