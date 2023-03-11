use bracket_lib::prelude::*;
use hecs::Entity;

use crate::{
    components::{Position, Viewer},
    State,
};

pub fn push_entity_in_line_to(s: &mut State, e: Entity, dest: Point) {
    let player_pos = s.ecs.query_one_mut::<&Position>(s.player_entity).unwrap().0;
    let (target_pos, viewer) = s
        .ecs
        .query_one_mut::<(&mut Position, &mut Viewer)>(e)
        .unwrap();
    viewer.dirty = true;
    let target = dest;
    let line = Bresenham::new(target_pos.0, target);
    let mut success = true;
    for step in line.skip(1) {
        let idx = s.map.point2d_to_index(step);
        if !s.map.is_available_exit(idx) || step == player_pos {
            success = false;
            break;
        }
        target_pos.0 = step;
    }
    let idx = s.map.point2d_to_index(target);
    if success && s.map.is_available_exit(idx) && target != player_pos {
        target_pos.0 = target;
    }
}

pub fn get_thing_with_thing_at_pos<Q: hecs::Query>(s: &mut State, pos: Point) -> Option<Entity> {
    let idx = s.map.point2d_to_index(pos);
    for ent in s.map.tile_contents[idx].iter() {
        if s.ecs.satisfies::<Q>(*ent).unwrap() {
            return Some(*ent);
        }
    }
    None
}
