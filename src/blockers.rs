use crate::{components::*, map, State};
use bracket_lib::prelude::*;
use hecs::With;

pub fn system_calc_blockers(state: &mut State) {
    for (idx, tile) in state.map.tiles.iter().enumerate() {
        if *tile == map::Tile::Wall {
            state.map.blocked_tiles[idx] = true;
        } else {
            state.map.blocked_tiles[idx] = false;
        }
    }
    for (_id, pos) in state.ecs.query_mut::<With<&Position, &Blocker>>() {
        let idx = state.map.point2d_to_index(pos.0);
        state.map.blocked_tiles[idx] = true;
    }
}
