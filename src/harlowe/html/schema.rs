use serde::{Deserialize, Serialize};
// use semver;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    // pub startnode: String,
    // pub creator: String,
    // pub creator_version: String,
    // pub format: String,
    // pub format_version: String,
    // pub ifid: String,
    // pub all_tags: Vec<Tag>,
    // pub passages: Vec<Passage>,
    // TODO: Wait, I don't see room-linking data in the HTML ...
    // what's up with that?! Ah, looking at what other projects
    // do about this, they are created based upon links in the
    // text -- we're going to have to parse the text to get the
    // list of linked rooms.
    //
    // TODO: Not sure what's up with the rest of these ... gonna put them
    // behind an 'attr_' prefix
    // pub attr_options: String,
    // pub attr_tags: String,
    // pub attr_zoom: Uint,
}

// impl Game {
//     pub fn creator_version(&self) -> semver::Version {
//         semver::Version::parse(&self.creator_version).unwrap()
//     }

//     pub fn format_version(&self) -> semver::Version {
//         semver::Version::parse(&self.format_version).unwrap()
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Passage {
    pub pid: String,
    pub name: String,
    pub tag_names: Vec<String>,
    pub tags: Vec<Tag>,
    pub position: Coordinate,
    pub size: Size2D,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Size2D {
    pub height: u16,
    pub width: u16,
}
