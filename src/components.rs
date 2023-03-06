use bracket_lib::prelude::*;
use hecs::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(from = "HealthHelper")]
pub struct Health {
    pub hp: i32,
    pub max_hp: i32,
}

#[derive(Deserialize)]
struct HealthHelper {
    hp: Option<i32>,
    max_hp: i32,
}

impl From<HealthHelper> for Health {
    fn from(c: HealthHelper) -> Self {
        if let Some(hp) = c.hp {
            Health {
                hp,
                max_hp: c.max_hp,
            }
        } else {
            Health {
                hp: c.max_hp,
                max_hp: c.max_hp,
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Attack {
    pub damage: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Position(pub Point);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(from = "RenderableHelper")]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub layer: i32, // higher is rendered first
}

#[derive(Debug, Deserialize)]
struct RenderableHelper {
    glyph: Glyph,
    fg: Color,
    bg: Color,
    layer: i32,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Glyph {
    Integer(FontCharType),
    Char(char),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Color {
    Triplet(RGB),
    String(String),
}

impl From<RenderableHelper> for Renderable {
    fn from(value: RenderableHelper) -> Self {
        let glyph = match value.glyph {
            Glyph::Integer(i) => i,
            Glyph::Char(c) => to_cp437(c),
        };
        let fg = match value.fg {
            Color::Triplet(c) => c,
            Color::String(s) => RGB::from_hex(s).unwrap(),
        };
        let bg = match value.bg {
            Color::Triplet(c) => c,
            Color::String(s) => RGB::from_hex(s).unwrap(),
        };
        Renderable {
            glyph,
            fg,
            bg,
            layer: value.layer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renderable_serde() {
        let json = r###"{"glyph":"a","fg":"#ffffff","bg":"#000000","layer":0}"###;
        let parsed: Renderable = serde_json::from_str(&json).unwrap();
        let saved = serde_json::to_string(&parsed).unwrap();
        let reparsed: Renderable = serde_json::from_str(&saved).unwrap();
        let data = Renderable {
            glyph: to_cp437('a'),
            fg: RGB::from_hex("#ffffff").unwrap(),
            bg: RGB::from_hex("#000000").unwrap(),
            layer: 0,
        };
        assert_eq!(data, parsed);
        assert_eq!(data, reparsed);
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Viewer {
    #[serde(default)]
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    #[serde(default = "_default_true")]
    pub dirty: bool,
}

const fn _default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Player {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Monster {
    #[serde(default)]
    pub tracking: Option<Point>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
    #[serde(default)]
    pub contents: Vec<Entity>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Name(pub String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Grower {
    Growing { seed: Entity, num_left: u32 },
    Empty,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Blocker {}
