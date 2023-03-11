use bracket_lib::prelude::*;
use hecs::Entity;
use serde::{Deserialize, Serialize};

use crate::{
    blueprint::BPImage,
    equipment::{Equipment, EquipmentType},
    essence::Essence,
    skill::Skill,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Attack {
    pub damage: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub current_blueprint: Option<Blueprint>,
    #[serde(skip)] // TODO
    /// This contains optional because when effects are executing, we need to `Option::take` the ingredients that we
    /// are baking with, so that the effect can't go over and stir them up
    pub passive_equipment: Vec<Option<Equipment>>,
    #[serde(skip)] // TODO
    /// This contains optional because when effects are executing, we need to `Option::take` the ingredients that we
    /// are baking with, so that the effect can't go over and stir them up
    pub active_equipment: Vec<Option<Equipment>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Monster {
    #[serde(default)]
    pub tracking: Option<Point>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Skilled {
    /// (probability_weight, skill) -- default action (e.g. move/hit) has weight of 1.0
    pub skills: Vec<(f32, Skill)>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
    #[serde(default)]
    pub contents: Vec<Entity>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Grower {
    Growing { seed: Entity, num_left: u32 },
    Empty,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Blocker {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Blueprint {
    pub img: BPImage,
    pub equipment: EquipmentType,
    #[serde(default)]
    pub filled: Vec<(usize, Essence)>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// gets removed when we go into a new level
pub struct Ephermal;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Elemental {
    Fire,
    Water,
    Air,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Rank(pub i32);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Slowed {
    pub duration: u32,
}

impl std::fmt::Display for Elemental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Elemental::Fire => "Fire",
            Elemental::Water => "Water",
            Elemental::Air => "Air",
        };
        write!(f, "{}", str)
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
