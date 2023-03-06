use crate::{components::Position, State};
use bracket_lib::prelude::*;

pub fn system_tile_contents(state: &mut State) {
    for tile in state.map.tile_contents.iter_mut() {
        tile.clear();
    }
    for (id, pos) in state.ecs.query_mut::<&Position>() {
        let idx = state.map.point2d_to_index(pos.0);
        state.map.tile_contents[idx].push(id);
    }
}
