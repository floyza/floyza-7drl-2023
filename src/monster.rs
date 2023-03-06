use crate::{components::*, State};
use bracket_lib::prelude::*;
use hecs::{Entity, World};

pub fn monster_act(state: &mut State, entity: Entity) {
    let player_pos = state
        .ecs
        .query_one_mut::<&Position>(state.player_entity)
        .unwrap()
        .0
        .clone();
    let mut target = None;
    if let Ok(viewer) = state.ecs.query_one_mut::<&Viewer>(entity) {
        if viewer.visible_tiles.contains(&player_pos) {
            target = Some(player_pos);
            if let Ok(mon) = state.ecs.query_one_mut::<&mut Monster>(entity) {
                mon.tracking = target;
            }
        }
    }
    if target == None {
        if let Ok(mon) = state.ecs.query_one_mut::<&Monster>(entity) {
            target = mon.tracking;
        }
    }
    let Some(target) = target else { return };
    let pos = state.ecs.query_one_mut::<&mut Position>(entity).unwrap();
    let start = state.map.point2d_to_index(pos.0);
    let end = state.map.point2d_to_index(target);
    let path = a_star_search(start, end, &state.map);
    if path.success && path.steps.len() > 1 {
        let step_idx = path.steps[1];
        let pt = state.map.index_to_point2d(step_idx);
        if pt == player_pos {
            // do attack
        } else {
            pos.0 = pt;
            if let Ok(mon) = state.ecs.query_one_mut::<&mut Monster>(entity) {
                if Some(pt) == mon.tracking {
                    mon.tracking = None;
                }
            }
            if let Ok(viewer) = state.ecs.query_one_mut::<&mut Viewer>(entity) {
                viewer.dirty = true;
            }
        }
    }
}

pub fn spawn_monster(ecs: &mut World, pos: Point) -> Entity {
    spawners::orc(ecs, pos)
}

mod spawners {
    use super::*;

    pub fn orc(ecs: &mut World, pos: Point) -> Entity {
        ecs.spawn((
            Name("Orc".to_owned()),
            Monster { tracking: None },
            Viewer {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            },
            Health { hp: 30, max_hp: 30 },
            Position(pos),
            Renderable {
                glyph: to_cp437('o'),
                fg: RGB::named(GREEN),
                bg: RGB::named(BLACK),
                layer: 1,
            },
            Blocker {},
        ))
    }
}
