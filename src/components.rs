use bracket_lib::prelude::*;
use hecs::Entity;

#[derive(Debug)]
pub struct Health(pub i32);

#[derive(Debug)]
pub struct Position(pub Point);

#[derive(Clone, Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub layer: i32, // higher is rendered first
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

#[derive(Debug)]
pub struct Item {}

#[derive(Debug)]
pub struct Inventory {
    pub contents: Vec<Entity>,
}

#[derive(Debug)]
pub struct Name(pub String);

#[derive(Debug)]
pub enum Grower {
    Growing { seed: Entity, num_left: u32 },
    Empty,
}
