use crate::components::*;
use crate::State;
use bracket_lib::prelude::*;
use hecs::Satisfies;

pub fn system_calc_viewpoints(state: &mut State) {
    for (_id, (viewer, position, is_player)) in
        state
            .ecs
            .query_mut::<(&mut Viewer, &Position, Satisfies<&Player>)>()
    {
        if viewer.dirty {
            viewer.dirty = false;
            viewer.visible_tiles.clear();
            viewer.visible_tiles = field_of_view(position.0, viewer.range, &state.map);
            viewer.visible_tiles.retain(|p| {
                p.x >= 0 && p.x < state.map.width && p.y >= 0 && p.y < state.map.height
            });
        }
        if is_player {
            for t in state.map.visible_tiles.iter_mut() {
                *t = false;
            }
            for vis in viewer.visible_tiles.iter() {
                let idx = state.map.point2d_to_index(*vis);
                state.map.revealed_tiles[idx] = true;
                state.map.visible_tiles[idx] = true;
            }
        }
    }
}
