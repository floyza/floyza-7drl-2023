use serde::{Deserialize, Serialize};

use crate::{components::*, State};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Essence {
    pub element: Elemental,
    pub power: i32,
}

pub fn gain_essence(state: &mut State, essence: Essence) {
    let p = state
        .ecs
        .query_one_mut::<&mut Player>(state.player_entity)
        .unwrap();
    let Some(bp) = &mut p.current_blueprint else { return };
    let count = bp.img.lookup().gem_spots.len();
    for idx in 0..count {
        if bp.filled.iter().find(|(i, _e)| *i == idx).is_none() {
            bp.filled.push((idx, essence.clone()));
            let strength = match essence.power {
                0 => "weak ",
                1 => "",
                2 => "strong ",
                _ => panic!("invalid strength"),
            };
            state.messages.enqueue_message(&format!(
                "Zoop! {} {}essence is sucked into your artifact.",
                essence.element, strength,
            ));
            return;
        }
    }
    // essence was not used, player didn't have a valid thing equipped
}
