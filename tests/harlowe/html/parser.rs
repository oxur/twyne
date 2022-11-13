use twyne::harlowe::html::{parse_file};

use super::utils;

#[test]
fn test_parse_file() {
    let game = parse_file(&utils::LOBBY_GAME).unwrap();
    // TODO: current html parser is a hack that we'll be moving
    // away from ... So not going to bother figuring out how to
    // get string literals from the source with no quotes.
    assert_eq!(game.name, "\"Lobby\"");
}
