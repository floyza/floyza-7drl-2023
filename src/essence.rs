use crate::{components::*, State};

pub fn gain_essence(state: &mut State, essence: Elemental) {
    let p = state
        .ecs
        .query_one_mut::<&mut Player>(state.player_entity)
        .unwrap();
    let Some(bp) = &mut p.current_blueprint else { return };
    let count = bp.img.lookup().gem_spots.len();
    for idx in 0..count {
        if bp.filled.iter().find(|(i, _e)| *i == idx).is_none() {
            bp.filled.push((idx, essence.clone()));
            state.messages.enqueue_message(&format!(
                "Zoop! {} essence is sucked into your blueprint.",
                essence
            ));
            return;
        }
    }
    // essence was not used, player didn't have a valid thing equipped
}