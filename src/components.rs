use bracket_lib::prelude::*;

#[derive(Debug)]
pub struct Health(pub i32);

#[derive(Debug)]
pub struct Position(pub Point);

#[derive(Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Debug)]
pub struct Viewer {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Debug)]
pub struct Player {}

#[derive(Debug)]
pub struct Monster {}
