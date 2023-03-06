use crate::{components::*, raws::RAWS, State};
use bracket_lib::prelude::*;
use hecs::Entity;

pub fn monster_act(state: &mut State, entity: Entity) {
    let player_pos = state
        .ecs
        .query_one_mut::<&Position>(state.player_entity)
        .unwrap()
        .0
        .clone();
    let (name, mon, viewer, attack, pos) = state
        .ecs
        .query_one_mut::<(
            &Name,
            &mut Monster,
            Option<&mut Viewer>,
            &Attack,
            &mut Position,
        )>(entity)
        .unwrap();
    let mut target = None;
    if let Some(viewer) = viewer.as_ref() {
        if viewer.visible_tiles.contains(&player_pos) {
            target = Some(player_pos);
            mon.tracking = target;
        }
    }
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
            let damage = attack.damage;
            state
                .messages
                .enqueue_message(&format!("The {} hits you for {} damage.", name.0, damage));
            let player_hp = state
                .ecs
                .query_one_mut::<&mut Health>(state.player_entity)
                .unwrap();
            player_hp.hp -= damage;
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

pub fn spawn_monster(state: &mut State, pos: Point) -> Entity {
    let entity = state.ecs.spawn(());
    {
        let raws = RAWS.lock().unwrap();
        let monster_of_choice = state.rng.range(0, raws.monsters.len());
        for component in raws.monsters[monster_of_choice].iter() {
            component.clone().insert(&mut state.ecs, entity).unwrap();
        }
    }
    state
        .ecs
        .insert(entity, (Monster { tracking: None }, Position(pos)))
        .unwrap();
    entity
}
