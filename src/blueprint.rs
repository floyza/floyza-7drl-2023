use std::{collections::HashMap, sync::Mutex};

use bracket_lib::prelude::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BPImage {
    Sword,
    Armor,
    Grapple,
    Gun,
}

impl BPImage {
    pub fn lookup(self) -> BPIData {
        BLUEPRINTS.lock().unwrap()[&self].clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BPIData {
    pub img: XpFile,
    pub gem_spots: Vec<Point>,
}

embedded_resource!(BP_SWORD, "../assets/sword.xp");
embedded_resource!(BP_ARMOR, "../assets/armor.xp");
embedded_resource!(BP_HOOK, "../assets/grappling-hook.xp");
embedded_resource!(BP_GUN, "../assets/elemental-gun.xp");

lazy_static! {
    pub static ref BLUEPRINTS: Mutex<HashMap<BPImage, BPIData>> = Mutex::new(HashMap::new());
}

pub fn load_blueprints() {
    link_resource!(BP_SWORD, "../assets/sword.xp");
    link_resource!(BP_ARMOR, "../assets/armor.xp");
    link_resource!(BP_HOOK, "../assets/grappling-hook.xp");
    link_resource!(BP_GUN, "../assets/elemental-gun.xp");
    let mut map = HashMap::new();
    {
        let xp = XpFile::from_resource("../assets/sword.xp").unwrap();
        map.insert(
            BPImage::Sword,
            BPIData {
                img: xp,
                gem_spots: vec![Point::new(8, 19)],
            },
        );
    }
    {
        let xp = XpFile::from_resource("../assets/armor.xp").unwrap();
        map.insert(
            BPImage::Armor,
            BPIData {
                img: xp,
                gem_spots: vec![Point::new(8, 15)],
            },
        );
    }
    {
        let xp = XpFile::from_resource("../assets/grappling-hook.xp").unwrap();
        map.insert(
            BPImage::Grapple,
            BPIData {
                img: xp,
                gem_spots: vec![Point::new(4, 8)],
            },
        );
    }
    {
        let xp = XpFile::from_resource("../assets/elemental-gun.xp").unwrap();
        map.insert(
            BPImage::Gun,
            BPIData {
                img: xp,
                gem_spots: vec![Point::new(3, 15)],
            },
        );
    }
    *BLUEPRINTS.lock().unwrap() = map;
}
