use bracket_lib::terminal::Point;
use hecs::Entity;

use crate::{components::*, raws::RAWS, State};

pub fn spawn_item(state: &mut State, pos: Point) -> Entity {
    let entity = state.ecs.spawn(());
    {
        let raws = RAWS.lock().unwrap();
        let item_of_choice = state.rng.range(0, raws.items.len());
        for component in raws.items[item_of_choice].iter() {
            component.clone().insert(&mut state.ecs, entity).unwrap();
        }
    }
    state.ecs.insert(entity, (Item {}, Position(pos))).unwrap();
    entity
}
