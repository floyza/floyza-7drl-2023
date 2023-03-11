use std::{collections::HashMap, sync::Mutex};

use bracket_lib::prelude::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BPImage {
    Sword,
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

lazy_static! {
    pub static ref BLUEPRINTS: Mutex<HashMap<BPImage, BPIData>> = Mutex::new(HashMap::new());
}

pub fn load_blueprints() {
    link_resource!(BP_SWORD, "../assets/sword.xp");
    let xp = XpFile::from_resource("../assets/sword.xp").unwrap();
    let map = HashMap::from([(
        BPImage::Sword,
        BPIData {
            img: xp,
            gem_spots: vec![Point::new(8, 19), Point::new(8, 8)],
        },
    )]);
    *BLUEPRINTS.lock().unwrap() = map;
}
