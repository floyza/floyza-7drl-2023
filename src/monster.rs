use crate::{components::*, raws::RAWS, State};
use bracket_lib::prelude::*;
use hecs::Entity;
use rand::{distributions::WeightedIndex, prelude::Distribution};

pub fn monster_act(state: &mut State, entity: Entity) {
    let player_pos = state
        .ecs
        .query_one_mut::<&Position>(state.player_entity)
        .unwrap()
        .0
        .clone();
    let (mon, viewer, skilled) = state
        .ecs
        .query_one_mut::<(&mut Monster, Option<&Viewer>, Option<&Skilled>)>(entity)
        .unwrap();
    let mut target = None;
    if let Some(viewer) = viewer.as_ref() {
        if viewer.visible_tiles.contains(&player_pos) {
            target = Some(player_pos);
            mon.tracking = target;
        }
    }
    if let Some(skilled) = skilled {
        let len = skilled.skills.len();
        let mut weights: Vec<f32> = skilled
            .skills
            .iter()
            .map(|x| x.get_prob() / len as f32)
            .collect();
        weights.push(1.0); // for normal action
        let dist = WeightedIndex::new(weights).unwrap();
        let roll = dist.sample(state.rng.get_rng());
        let mut success = false;
        if roll != len {
            // if not normal attack
            let skill = skilled.skills[roll];
            success = skill.apply(entity, state.player_entity, state).is_some();
        }
        if success {
            return;
        }
    }
    let (mon, viewer, pos) = state
        .ecs
        .query_one_mut::<(&mut Monster, Option<&mut Viewer>, &mut Position)>(entity)
        .unwrap();
    if target == None {
        target = mon.tracking;
    }
    let Some(target) = target else { return };
    let start = state.map.point2d_to_index(pos.0);
    let end = state.map.point2d_to_index(target);
    let path = a_star_search(start, end, &state.map);
    if path.success && path.steps.len() > 1 {
        let step_idx = path.steps[1];
        let pt = state.map.index_to_point2d(step_idx);
        if pt == player_pos {
            let (name, attack) = state
                .ecs
                .query_one_mut::<(&Name, Option<&Attack>)>(entity)
                .unwrap();
            if let Some(attack) = attack {
                let damage = attack.damage;
                state
                    .messages
                    .enqueue_message(&format!("The {} hits you for {} damage.", name.0, damage));
                let player_hp = state
                    .ecs
                    .query_one_mut::<&mut Health>(state.player_entity)
                    .unwrap();
                player_hp.hp -= damage;
            }
        } else {
            pos.0 = pt;
            if Some(pt) == mon.tracking {
                mon.tracking = None;
            }
            if let Some(viewer) = viewer {
                viewer.dirty = true;
            }
        }
    }
}

pub fn spawn_monster(state: &mut State, dl: i32, pos: Point) -> Entity {
    let entity = state.ecs.spawn(());
    {
        let raws = RAWS.lock().unwrap();
        let monsters = &raws.monsters[&dl];
        let monster_of_choice = state.rng.range(0, monsters.len());
        for component in monsters[monster_of_choice].iter() {
            component.clone().insert(&mut state.ecs, entity).unwrap();
        }
    }
    state
        .ecs
        .insert(
            entity,
            (
                Monster { tracking: None },
                Position(pos),
                Ephermal,
                Blocker {},
            ),
        )
        .unwrap();
    entity
}
