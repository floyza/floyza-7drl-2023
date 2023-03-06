use crate::{components::Health, State};

pub fn system_kill_dead(state: &mut State) {
    let mut dead = vec![];
    for (id, health) in state.ecs.query_mut::<&Health>() {
        if health.hp <= 0 {
            dead.push(id);
        }
    }
    for id in dead {
        if id == state.player_entity {
            continue;
        }
        state.ecs.despawn(id).unwrap();
        if let Some((i, _)) = state.turn_order.iter().enumerate().find(|(_, e)| **e == id) {
            state.turn_order.remove(i);
        }
    }
}
