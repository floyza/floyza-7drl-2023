use hecs::Entity;
use serde::{Deserialize, Serialize};

use crate::{components::*, essence::Essence, State};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EquipmentType {
    Sword,
    Gun,     // parameters: targeting, effect
    Grapple, // parameters: intensity, targeting
}

pub struct Equipment {
    pub ingredients: (EquipmentType, Vec<Essence>),
    pub effect: EquipmentEffect,
}

pub enum EquipmentEffect {
    Active(ActiveEquipment),
    Passive(PassiveEquipment),
}

pub enum ActiveEquipment {}

pub enum PassiveEquipment {
    AttackEffect(fn(&mut State, Entity)),
}

pub fn build_blueprint(bp: &Blueprint) -> Equipment {
    let mut gems = vec![];
    for i in 0..bp.filled.len() {
        gems.push(bp.filled[i].1.clone());
    }
    match bp.equipment {
        EquipmentType::Sword => {
            let eff = EquipmentEffect::Passive(PassiveEquipment::AttackEffect(|s, e| {
                let health = s.ecs.query_one_mut::<&mut Health>(e).unwrap();
                health.hp -= 5;
            }));
            return Equipment {
                ingredients: (EquipmentType::Sword, gems),
                effect: eff,
            };
        }
        EquipmentType::Grapple => {}
        EquipmentType::Gun => {}
    }
    todo!();
}
