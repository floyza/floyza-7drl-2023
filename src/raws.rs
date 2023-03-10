use std::{collections::HashMap, sync::Mutex};

use bracket_lib::prelude::*;
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::components_serde::Component;

#[derive(Deserialize, Debug)]
pub struct Raws {
    pub monsters: HashMap<i32, Vec<Vec<Component>>>, // no need to manually add `Monster` component
    pub items: HashMap<i32, Vec<Vec<Component>>>,    // same as above, but with `Item` component
}

embedded_resource!(RAW_FILE, "../raws/spawns.json");

lazy_static! {
    pub static ref RAWS: Mutex<Raws> = Mutex::new(Raws {
        monsters: HashMap::new(),
        items: HashMap::new(),
    });
}

pub fn load_raws() {
    link_resource!(RAW_FILE, "../raws/spawns.json");
    let data = EMBED
        .lock()
        .get_resource("../raws/spawns.json".to_string())
        .unwrap();
    let string = std::str::from_utf8(&data).expect("Unable to convert to a valid UTF-8 string.");
    let raws: Raws = serde_json::from_str(string).expect("Unable to parse json");
    *RAWS.lock().unwrap() = raws;
}
