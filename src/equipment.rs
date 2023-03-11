use std::fmt;

use bracket_lib::prelude::*;
use hecs::Entity;
use serde::{Deserialize, Serialize};

use crate::{components::*, essence::Essence, math::normalize_pt, State};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EquipmentType {
    Sword,   // 1 arg - effect
    Armor,   // 1 arg - effect
    Gun,     // parameters: targeting, effect
    Grapple, // 1 arg - targetting
}

#[derive(Debug, Clone)]
pub struct Equipment {
    pub ingredients: (EquipmentType, Vec<Essence>),
    pub effect: EquipmentEffect,
}

pub fn print_desc(typ: EquipmentType, ess: &Vec<Option<Essence>>, builder: &mut TextBuilder) {
    match typ {
        EquipmentType::Sword => sword_desc(ess, builder),
        EquipmentType::Armor => armor_desc(ess, builder),
        EquipmentType::Gun => todo!(),
        EquipmentType::Grapple => grapple_desc(ess, builder),
    };
}

fn colorize_print_element(
    fire: &str,
    water: &str,
    air: &str,
    e: Option<Essence>,
    builder: &mut TextBuilder,
) {
    if let Some(e) = e {
        match e.element {
            Elemental::Fire => builder.fg(RGB::named(RED)).line_wrap(fire),
            Elemental::Water => builder.fg(RGB::named(DARKBLUE)).line_wrap(water),
            Elemental::Air => builder.fg(RGB::named(SKYBLUE)).line_wrap(air),
        };
    } else {
        builder.fg(RGB::named(GRAY)).line_wrap("___");
    }
}

fn sword_desc(ess: &Vec<Option<Essence>>, builder: &mut TextBuilder) {
    colorize_print_element(
        "Damage",
        "Freeze and slow",
        "Blast back",
        ess[0].clone(),
        builder,
    );
    builder
        .fg(RGB::named(WHITE))
        .line_wrap("your target on hit.");
}

fn armor_desc(ess: &Vec<Option<Essence>>, builder: &mut TextBuilder) {
    colorize_print_element(
        "Damage",
        "Freeze and slow",
        "Blast back",
        ess[0].clone(),
        builder,
    );
    builder
        .fg(RGB::named(WHITE))
        .line_wrap("your attacker when hit.");
}

fn grapple_desc(ess: &Vec<Option<Essence>>, builder: &mut TextBuilder) {
    builder.fg(RGB::named(WHITE)).line_wrap("Yank");
    colorize_print_element(
        "enemies with a shotgun blast of hooks",
        "one enemy all the way to you",
        "enemies chain-lightning style",
        ess[0].clone(),
        builder,
    );
}

#[derive(Debug, Clone)]
pub enum EquipmentEffect {
    Active(ActiveEquipment),
    Passive(PassiveEquipment),
}

#[derive(Clone)]
pub enum ActiveEquipment {
    TargetEffect(fn(&mut State, Entity, &Vec<Essence>)),
}

impl fmt::Debug for ActiveEquipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TargetEffect(_) => write!(f, "TargetEffect"),
        }
    }
}

#[derive(Clone)]
pub enum PassiveEquipment {
    AttackEffect(fn(&mut State, Entity, &Vec<Essence>)),
    GotHitEffect(fn(&mut State, Entity, &Vec<Essence>)),
}

impl fmt::Debug for PassiveEquipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AttackEffect(_) => write!(f, "AttackEffect"),
            Self::GotHitEffect(_) => write!(f, "GotHitEffect"),
        }
    }
}

pub fn build_blueprint(bp: &Blueprint) -> Equipment {
    let mut gems = vec![];
    for i in 0..bp.filled.len() {
        gems.push(bp.filled[i].1.clone());
    }
    match bp.equipment {
        EquipmentType::Armor => {
            debug_assert!(gems.len() == 1);
            let eff = match gems[0].element {
                Elemental::Fire => |s: &mut State, e, gems: &Vec<Essence>| {
                    let health = s.ecs.query_one_mut::<&mut Health>(e).unwrap();
                    health.hp -= (gems[0].power + 1) * 5;
                },
                Elemental::Water => |s: &mut State, e, gems: &Vec<Essence>| {
                    s.ecs
                        .insert_one(
                            e,
                            Slowed {
                                duration: (gems[0].power as u32 + 1) * 2,
                            },
                        )
                        .unwrap();
                },
                Elemental::Air => |s: &mut State, e, gems: &Vec<Essence>| {
                    let player_pos = s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
                    let target_pos = s.ecs.query_one_mut::<&mut Position>(e).unwrap();
                    let offset = normalize_pt(target_pos.0 - player_pos);
                    let target = target_pos.0 + offset * (gems[0].power + 1);
                    let line = Bresenham::new(target_pos.0, target);
                    let mut success = true;
                    for step in line.skip(1) {
                        let idx = s.map.point2d_to_index(step);
                        if !s.map.is_available_exit(idx) {
                            success = false;
                            break;
                        }
                        target_pos.0 = step;
                    }
                    let idx = s.map.point2d_to_index(target);
                    if success && s.map.is_available_exit(idx) {
                        target_pos.0 = target;
                    }
                },
            };
            let eff = EquipmentEffect::Passive(PassiveEquipment::GotHitEffect(eff));
            return Equipment {
                ingredients: (EquipmentType::Armor, gems),
                effect: eff,
            };
        }
        EquipmentType::Sword => {
            debug_assert!(gems.len() == 1);
            let eff = match gems[0].element {
                Elemental::Fire => |s: &mut State, e, gems: &Vec<Essence>| {
                    let health = s.ecs.query_one_mut::<&mut Health>(e).unwrap();
                    health.hp -= (gems[0].power + 1) * 5;
                },
                Elemental::Water => |s: &mut State, e, gems: &Vec<Essence>| {
                    s.ecs
                        .insert_one(
                            e,
                            Slowed {
                                duration: (gems[0].power as u32 + 1) * 2,
                            },
                        )
                        .unwrap();
                },
                Elemental::Air => |s: &mut State, e, gems: &Vec<Essence>| {
                    let player_pos = s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
                    let target_pos = s.ecs.query_one_mut::<&mut Position>(e).unwrap();
                    let offset = normalize_pt(target_pos.0 - player_pos);
                    let target = target_pos.0 + offset * (gems[0].power + 1);
                    let line = Bresenham::new(target_pos.0, target);
                    let mut success = true;
                    for step in line.skip(1) {
                        let idx = s.map.point2d_to_index(step);
                        if !s.map.is_available_exit(idx) {
                            success = false;
                            break;
                        }
                        target_pos.0 = step;
                    }
                    let idx = s.map.point2d_to_index(target);
                    if success && s.map.is_available_exit(idx) {
                        target_pos.0 = target;
                    }
                },
            };
            let eff = EquipmentEffect::Passive(PassiveEquipment::AttackEffect(eff));
            return Equipment {
                ingredients: (EquipmentType::Sword, gems),
                effect: eff,
            };
        }
        EquipmentType::Grapple => {
            debug_assert!(gems.len() == 1);
            let eff = match gems[0].element {
                Elemental::Fire => |s: &mut State, e, gems: &Vec<Essence>| {},
                Elemental::Water => todo!(),
                Elemental::Air => todo!(),
            };
            let eff = EquipmentEffect::Active(ActiveEquipment::TargetEffect(eff));
            return Equipment {
                ingredients: (EquipmentType::Grapple, gems),
                effect: eff,
            };
        }
        EquipmentType::Gun => {}
    }
    todo!();
}
