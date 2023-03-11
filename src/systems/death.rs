use crate::{components::*, essence::gain_essence, State};

pub fn system_kill_dead(state: &mut State) {
    let mut dead = vec![];
    let mut reaped_essence = vec![];
    for (id, (elem, health)) in state.ecs.query_mut::<(Option<&Elemental>, &Health)>() {
        if health.hp <= 0 {
            dead.push(id);
            if let Some(elem) = elem {
                if state.rng.range(0, 4) == 0 {
                    reaped_essence.push(elem.clone());
                }
            }
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
    for essence in reaped_essence {
        gain_essence(state, essence);
    }
}
