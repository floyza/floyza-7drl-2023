use std::fmt;

use bracket_lib::prelude::*;
use hecs::Entity;
use serde::{Deserialize, Serialize};

use crate::{
    blueprint::BPImage,
    components::*,
    essence::Essence,
    math::normalize_pt,
    util::{get_thing_with_thing_at_pos, push_entity_in_line_to},
    State,
};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EquipmentType {
    Sword,   // 1 arg - effect
    Armor,   // 1 arg - effect
    Gun,     // 1 arg - effect
    Grapple, // 1 arg - targetting
}

#[derive(Debug, Clone)]
pub struct Equipment {
    pub ingredients: (EquipmentType, Vec<Essence>),
    pub effect: EquipmentEffect,
    pub img: BPImage,
}

pub fn print_desc(typ: EquipmentType, ess: &Vec<Option<Essence>>, builder: &mut TextBuilder) {
    match typ {
        EquipmentType::Sword => sword_desc(ess, builder),
        EquipmentType::Armor => armor_desc(ess, builder),
        EquipmentType::Gun => gun_desc(ess, builder),
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
            Elemental::Water => builder.fg(RGB::named(BLUE3)).line_wrap(water),
            Elemental::Air => builder.fg(RGB::named(SKYBLUE)).line_wrap(air),
        };
        match e.power {
            0 => builder.line_wrap("(v1)"),
            1 => builder.line_wrap("(v2)"),
            2 => builder.line_wrap("(v3)"),
            _ => panic!("invalid power"),
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
        .line_wrap("your attacker when hit, and also block damage.");
}

fn grapple_desc(ess: &Vec<Option<Essence>>, builder: &mut TextBuilder) {
    builder.fg(RGB::named(WHITE)).line_wrap("Yank");
    colorize_print_element(
        "and damage an enemy, bringing it",
        "one enemy all the way",
        "enemies chain-lightning style",
        ess[0].clone(),
        builder,
    );
    builder.fg(RGB::named(WHITE)).line_wrap("to you.");
}

fn gun_desc(ess: &Vec<Option<Essence>>, builder: &mut TextBuilder) {
    builder.fg(RGB::named(WHITE)).line_wrap("Shoot a");
    colorize_print_element("damaging", "TODO", "pushing", ess[0].clone(), builder);
    builder
        .fg(RGB::named(WHITE))
        .line_wrap("bullet at an enemy.");
}

#[derive(Debug, Clone)]
pub enum EquipmentEffect {
    Active(ActiveEquipment),
    Passive(PassiveEquipment),
}

#[derive(Clone)]
pub enum ActiveEquipment {
    TargetEffect(fn(&mut State, Point, &Vec<Essence>)),
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
    GotHitEffect(fn(&mut State, Entity, &Vec<Essence>) -> i32),
}

impl fmt::Debug for PassiveEquipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AttackEffect(_) => write!(f, "AttackEffect"),
            Self::GotHitEffect(_) => write!(f, "GotHitEffect"),
        }
    }
}

pub fn execute_active_target(state: &mut State, ability_idx: usize, target: Point) {
    let player = state
        .ecs
        .query_one_mut::<&mut Player>(state.player_entity)
        .unwrap();
    let equip = player.active_equipment[ability_idx].take().unwrap();
    let EquipmentEffect::Active(ActiveEquipment::TargetEffect(eff)) = equip.effect else {panic!()};
    eff(state, target, &equip.ingredients.1);
    let player = state
        .ecs
        .query_one_mut::<&mut Player>(state.player_entity)
        .unwrap();
    player.active_equipment[ability_idx] = Some(equip);
}

pub fn execute_attack_effects(state: &mut State, target: Entity) {
    let player = state
        .ecs
        .query_one_mut::<&mut Player>(state.player_entity)
        .unwrap();
    let mut equip = vec![];
    for (i, eq_maybe) in player.passive_equipment.iter_mut().enumerate() {
        let Some(eq) = eq_maybe else {continue};
        if matches!(
            eq.effect,
            EquipmentEffect::Passive(PassiveEquipment::AttackEffect(_))
        ) {
            equip.push((i, eq_maybe.take().unwrap()));
        }
    }
    for (i, eq) in equip {
        let EquipmentEffect::Passive(PassiveEquipment::AttackEffect(eff)) =
                                eq.effect else {panic!()};
        eff(state, target, &eq.ingredients.1);
        let player = state
            .ecs
            .query_one_mut::<&mut Player>(state.player_entity)
            .unwrap();
        debug_assert!(player.passive_equipment[i].is_none());
        player.passive_equipment[i] = Some(eq);
    }
}

pub fn execute_defence_effects(state: &mut State, target: Entity) -> i32 {
    let player = state
        .ecs
        .query_one_mut::<&mut Player>(state.player_entity)
        .unwrap();
    let mut equip = vec![];
    for (i, eq_maybe) in player.passive_equipment.iter_mut().enumerate() {
        let Some(eq) = eq_maybe else {continue};
        if matches!(
            eq.effect,
            EquipmentEffect::Passive(PassiveEquipment::GotHitEffect(_))
        ) {
            equip.push((i, eq_maybe.take().unwrap()));
        }
    }
    let mut blocked = 0;
    for (i, eq) in equip {
        let EquipmentEffect::Passive(PassiveEquipment::GotHitEffect(eff)) =
                                eq.effect else {panic!()};
        blocked += eff(state, target, &eq.ingredients.1);
        let player = state
            .ecs
            .query_one_mut::<&mut Player>(state.player_entity)
            .unwrap();
        debug_assert!(player.passive_equipment[i].is_none());
        player.passive_equipment[i] = Some(eq);
    }
    return blocked;
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
                    let (health, name) = s.ecs.query_one_mut::<(&mut Health, &Name)>(e).unwrap();
                    let dam = (gems[0].power + 1) * 2;
                    health.hp -= dam;
                    s.messages.enqueue_message(&format!(
                        "Your armor hits the attacking {} for {} damage.",
                        name.0, dam
                    ));
                    return gems[0].power + 1;
                },
                Elemental::Water => |s: &mut State, e, gems: &Vec<Essence>| {
                    let slowed = s.ecs.query_one_mut::<Option<&Slowed>>(e).unwrap().cloned();
                    if let Some(slowed) = slowed {
                        s.ecs
                            .insert_one(
                                e,
                                Slowed {
                                    duration: (gems[0].power as u32 + 1) * 1 + slowed.duration,
                                },
                            )
                            .unwrap();
                    } else {
                        s.ecs
                            .insert_one(
                                e,
                                Slowed {
                                    duration: (gems[0].power as u32 + 1) * 1,
                                },
                            )
                            .unwrap();
                    }
                    let name = s.ecs.query_one_mut::<&Name>(e).unwrap();
                    s.messages
                        .enqueue_message(&format!("Your armor slows the attacking {}.", name.0,));
                    return gems[0].power + 1;
                },
                Elemental::Air => |s: &mut State, e, gems: &Vec<Essence>| {
                    let player_pos = s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
                    let target_pos = s.ecs.query_one_mut::<&Position>(e).unwrap().0;
                    let dest =
                        target_pos + normalize_pt(target_pos - player_pos) * (gems[0].power + 1);
                    push_entity_in_line_to(s, e, dest);
                    let name = s.ecs.query_one_mut::<&Name>(e).unwrap();
                    s.messages.enqueue_message(&format!(
                        "Your armor blasts back the attacking {}.",
                        name.0,
                    ));
                    return gems[0].power + 1;
                },
            };
            let eff = EquipmentEffect::Passive(PassiveEquipment::GotHitEffect(eff));
            return Equipment {
                ingredients: (EquipmentType::Armor, gems),
                effect: eff,
                img: bp.img,
            };
        }
        EquipmentType::Sword => {
            debug_assert!(gems.len() == 1);
            let eff = match gems[0].element {
                Elemental::Fire => |s: &mut State, e, gems: &Vec<Essence>| {
                    let (health, name) = s.ecs.query_one_mut::<(&mut Health, &Name)>(e).unwrap();
                    let dam = (gems[0].power + 1) * 5;
                    health.hp -= dam;
                    s.messages.enqueue_message(&format!(
                        "Your sword flames, dealing {} extra damage to the {}.",
                        dam, name.0
                    ));
                },
                Elemental::Water => |s: &mut State, e, gems: &Vec<Essence>| {
                    let slowed = s.ecs.query_one_mut::<Option<&Slowed>>(e).unwrap().cloned();
                    if let Some(slowed) = slowed {
                        s.ecs
                            .insert_one(
                                e,
                                Slowed {
                                    duration: (gems[0].power as u32 + 1) * 1 + slowed.duration,
                                },
                            )
                            .unwrap();
                    } else {
                        s.ecs
                            .insert_one(
                                e,
                                Slowed {
                                    duration: (gems[0].power as u32 + 1) * 1,
                                },
                            )
                            .unwrap();
                    }
                    let name = s.ecs.query_one_mut::<&Name>(e).unwrap();
                    s.messages.enqueue_message(&format!(
                        "Your sword glistens with ice, slowing the {}.",
                        name.0
                    ));
                },
                Elemental::Air => |s: &mut State, e, gems: &Vec<Essence>| {
                    let player_pos = s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
                    let target_pos = s.ecs.query_one_mut::<&Position>(e).unwrap().0;
                    let dest =
                        target_pos + normalize_pt(target_pos - player_pos) * (gems[0].power + 1);
                    push_entity_in_line_to(s, e, dest);
                    let name = s.ecs.query_one_mut::<&Name>(e).unwrap();
                    s.messages
                        .enqueue_message(&format!("Your sword blows back the {}.", name.0));
                },
            };
            let eff = EquipmentEffect::Passive(PassiveEquipment::AttackEffect(eff));
            return Equipment {
                ingredients: (EquipmentType::Sword, gems),
                effect: eff,
                img: bp.img,
            };
        }
        EquipmentType::Grapple => {
            debug_assert!(gems.len() == 1);
            let eff = match gems[0].element {
                Elemental::Fire => |s: &mut State, pt, gems: &Vec<Essence>| {
                    if let Some(e) = get_thing_with_thing_at_pos::<&Monster>(s, pt) {
                        let player_pos =
                            s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
                        let dest = pt + normalize_pt(player_pos - pt) * (gems[0].power + 1);
                        push_entity_in_line_to(s, e, dest);
                        let health = s.ecs.query_one_mut::<&mut Health>(e).unwrap();
                        let dam = (gems[0].power + 1) * 1;
                        health.hp -= dam;
                        let name = s.ecs.query_one_mut::<&Name>(e).unwrap();
                        s.messages.enqueue_message(&format!(
                            "You hook the {} and deal {dam} damage.",
                            name.0
                        ));
                    }
                },
                Elemental::Water => |s: &mut State, pt, _gems: &Vec<Essence>| {
                    if let Some(e) = get_thing_with_thing_at_pos::<&Monster>(s, pt) {
                        let player_pos =
                            s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
                        push_entity_in_line_to(s, e, player_pos);
                        let name = s.ecs.query_one_mut::<&Name>(e).unwrap();
                        s.messages
                            .enqueue_message(&format!("You hook the {}.", name.0));
                    }
                },
                Elemental::Air => |s: &mut State, mut pt, gems: &Vec<Essence>| {
                    let player_pos = s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
                    let mut targets = Vec::new();
                    if let Some(mut target) = get_thing_with_thing_at_pos::<&Monster>(s, pt) {
                        let dest = pt + normalize_pt(player_pos - pt) * (gems[0].power + 1);
                        targets.push((target, dest));
                        'chain: loop {
                            for x in -2..=2 {
                                for y in -2..=2 {
                                    if let Some(potential) = get_thing_with_thing_at_pos::<&Monster>(
                                        s,
                                        pt + Point::new(x, y),
                                    ) {
                                        if targets.iter().all(|(e, _)| *e != potential) {
                                            pt += Point::new(x, y);
                                            target = potential;
                                            let dest = pt
                                                + normalize_pt(player_pos - pt)
                                                    * (gems[0].power + 1);
                                            targets.push((target, dest));
                                            continue 'chain;
                                        }
                                    }
                                }
                            }
                            break;
                        }
                    }
                    let mut first = true;
                    for (e, dest) in targets {
                        let name = s.ecs.query_one_mut::<&Name>(e).unwrap();
                        if first {
                            s.messages
                                .enqueue_message(&format!("You hook the {}...", name.0));
                            first = false;
                        } else {
                            s.messages
                                .enqueue_message(&format!("...and the {}...", name.0));
                        }
                        push_entity_in_line_to(s, e, dest);
                    }
                },
            };
            let eff = EquipmentEffect::Active(ActiveEquipment::TargetEffect(eff));
            return Equipment {
                ingredients: (EquipmentType::Grapple, gems),
                effect: eff,
                img: bp.img,
            };
        }
        EquipmentType::Gun => {
            debug_assert!(gems.len() == 1);
            let eff = match gems[0].element {
                Elemental::Fire => |s: &mut State, pt, gems: &Vec<Essence>| {
                    if let Some(e) = get_thing_with_thing_at_pos::<&Monster>(s, pt) {
                        let health = s.ecs.query_one_mut::<&mut Health>(e).unwrap();
                        let dam = (gems[0].power + 1) * 2;
                        health.hp -= dam;
                        let name = s.ecs.query_one_mut::<&Name>(e).unwrap();
                        s.messages.enqueue_message(&format!(
                            "You shoot the {} and deal {dam} damage.",
                            name.0
                        ));
                    }
                },
                Elemental::Water => |s: &mut State, pt, gems: &Vec<Essence>| {
                    todo!();
                    // if let Some(e) = get_thing_with_thing_at_pos::<&Monster>(s, pt) {
                    //     let player_pos =
                    //         s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
                    //     push_entity_in_line_to(s, e, player_pos);
                    //     let name = s.ecs.query_one_mut::<&Name>(e).unwrap();
                    //     s.messages
                    //         .enqueue_message(&format!("You hook the {}.", name.0));
                    // }
                },
                Elemental::Air => |s: &mut State, pt, gems: &Vec<Essence>| {
                    let player_pos = s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
                    if let Some(target) = get_thing_with_thing_at_pos::<&Monster>(s, pt) {
                        let dest = pt + normalize_pt(pt - player_pos) * (gems[0].power + 1);
                        push_entity_in_line_to(s, target, dest);
                        let name = s.ecs.query_one_mut::<&Name>(target).unwrap();
                        s.messages
                            .enqueue_message(&format!("You blast the {} backwards.", name.0));
                    }
                },
            };
            let eff = EquipmentEffect::Active(ActiveEquipment::TargetEffect(eff));
            return Equipment {
                ingredients: (EquipmentType::Gun, gems),
                effect: eff,
                img: bp.img,
            };
        }
    }
}
