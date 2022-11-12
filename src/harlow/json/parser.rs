use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json;
use crate::harlow::json::schema;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<schema::Game, Box<dyn Error>> {
    let file =  File::open(path)?;
    let reader = BufReader::new(file);
    let g = serde_json::from_reader(reader)?;
    Ok(g)
}

pub fn parse_bytes<'a>(data: &'a [u8]) -> serde_json::Result<schema::Game> {
    serde_json::from_slice(data)
}

pub fn parse_str<'a>(data: &'a str) -> serde_json::Result<schema::Game> {
    serde_json::from_str(data)
}
