use bracket_lib::terminal::Point;
use hecs::Entity;

use crate::{components::*, raws::RAWS, State};

pub fn spawn_item(state: &mut State, dl: i32, pos: Point) -> Entity {
    let entity = state.ecs.spawn(());
    {
        let raws = RAWS.lock().unwrap();
        let items = &raws.items[&dl];
        let item_of_choice = state.rng.range(0, items.len());
        for component in items[item_of_choice].iter() {
            component.clone().insert(&mut state.ecs, entity).unwrap();
        }
    }
    state.ecs.insert(entity, (Item {}, Position(pos))).unwrap();
    entity
}
