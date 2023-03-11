use bracket_lib::prelude::*;
use hecs::With;

use crate::{
    components::*,
    equipment::{build_blueprint, EquipmentEffect, PassiveEquipment},
    map,
    mapping::Command,
    ui, OperatingMode, State,
};

pub fn player_act(state: &mut State, command: &Command) -> bool {
    match *command {
        Command::Move { target: move_pt } => {
            let position = state
                .ecs
                .query_one_mut::<&Position>(state.player_entity)
                .unwrap()
                .clone();
            let new_pt = position.0 + move_pt;
            let new_idx = state.map.point2d_to_index(new_pt);

            if let Ok(attacker) = state
                .ecs
                .query_one_mut::<&Attack>(state.player_entity)
                .cloned()
            {
                let mut found_target = None;
                for entity in state.map.tile_contents[new_idx].iter() {
                    if let Ok((health, name)) = state
                        .ecs
                        .query_one_mut::<With<(&mut Health, &Name), &Monster>>(*entity)
                    {
                        found_target = Some(*entity);
                        break;
                    }
                }
                if let Some(target) = found_target {
                    let (health, name) = state
                        .ecs
                        .query_one_mut::<With<(&mut Health, &Name), &Monster>>(target)
                        .unwrap();
                    state.messages.enqueue_message(&format!(
                        "You hit the {} for {} damage.",
                        name.0, attacker.damage
                    ));
                    health.hp -= attacker.damage;
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
                    return true;
                }
            }
            if state.map.is_available_exit(new_idx) {
                let position = state
                    .ecs
                    .query_one_mut::<&mut Position>(state.player_entity)
                    .unwrap();
                position.0 = new_pt;
                if let Ok(viewer) = state.ecs.query_one_mut::<&mut Viewer>(state.player_entity) {
                    viewer.dirty = true;
                }
                true
            } else {
                false
            }
        }
        Command::Grab => {
            let position = state
                .ecs
                .query_one_mut::<&Position>(state.player_entity)
                .unwrap();
            let mut items = Vec::new();
            for item in state.map.tile_contents[state.map.point2d_to_index(position.0)].iter() {
                if state
                    .ecs
                    .satisfies::<(&Item, &Position)>(*item)
                    .unwrap_or(false)
                {
                    items.push(*item);
                }
            }
            if let Some(item) = items.first() {
                state.ecs.remove_one::<Position>(*item).unwrap(); // we already ascertained that it has a component
                let inv = state
                    .ecs
                    .query_one_mut::<&mut Inventory>(state.player_entity)
                    .unwrap();
                inv.contents.push(*item);
                if let Some(name) = state.ecs.query_one_mut::<&Name>(*item).ok() {
                    state
                        .messages
                        .enqueue_message(&format!("You pick up a {}.", name.0));
                } else {
                    state.messages.enqueue_message("You pick something up.");
                }
                true
            } else {
                false
            }
        }
        Command::OpenInventory => {
            let inv = state
                .ecs
                .query_one_mut::<&Inventory>(state.player_entity)
                .unwrap();
            state.operating_mode = OperatingMode::OpenInventory(ui::InvUIState {
                selection: 0,
                length: inv.contents.len() as u32,
                confirming: None,
            });
            false
        }
        Command::OpenMessageLog => {
            state.operating_mode = OperatingMode::OpenMessageLog;
            false
        }
        Command::OpenExamine => {
            let player_pos = state
                .ecs
                .query_one_mut::<&Position>(state.player_entity)
                .unwrap();
            state.operating_mode = OperatingMode::OpenExamine(ui::ExamineUIState {
                point: Point::new(map::MAP_UI_DIM.width() / 2, map::MAP_UI_DIM.height() / 2),
            });
            false
        }
        Command::Wait => true,
        Command::DescendStairs => {
            let player_pos = state
                .ecs
                .query_one_mut::<&Position>(state.player_entity)
                .unwrap();
            let idx = state.map.point2d_to_index(player_pos.0);
            if state.map.tiles[idx] == map::Tile::Stairs {
                state.messages.enqueue_message("You descend the stairs.");
                map::new_floor(state);
                return true;
            }
            false
        }
        Command::CreateItem => {
            let p = state
                .ecs
                .query_one_mut::<&mut Player>(state.player_entity)
                .unwrap();
            let Some(bp) = &p.current_blueprint else { return false };
            if bp.filled.len() == bp.img.lookup().gem_spots.len() {
                let thing = build_blueprint(bp);
                match thing.effect {
                    EquipmentEffect::Active(_) => p.active_equipment.push(Some(thing)),
                    EquipmentEffect::Passive(_) => p.passive_equipment.push(Some(thing)),
                }
                state
                    .messages
                    .enqueue_message(&format!("You build a {:?}!", bp.equipment));
                p.current_blueprint = None;
                return true;
            }
            state
                .messages
                .enqueue_message("Slots not full: cannot build yet.");
            false
        }
        _ => false,
    }
}
