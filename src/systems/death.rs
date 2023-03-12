use crate::{
    components::*,
    essence::{gain_essence, Essence},
    OperatingMode, State,
};

pub fn system_kill_dead(state: &mut State) {
    let mut dead = vec![];
    let mut reaped_essence = vec![];
    for (id, (elem, rank, health)) in state
        .ecs
        .query_mut::<(Option<&Elemental>, Option<&Rank>, &Health)>()
    {
        if health.hp <= 0 {
            dead.push(id);
            if let (Some(elem), Some(rank)) = (elem, rank) {
                if state.rng.range(0, 4) == 0 {
                    reaped_essence.push(Essence {
                        element: elem.clone(),
                        power: rank.clone().0,
                    });
                }
            }
        }
    }
    for id in dead {
        if id == state.player_entity {
            state.messages.enqueue_message("You are DEAD.");
            state.operating_mode = OperatingMode::GameOver;
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
