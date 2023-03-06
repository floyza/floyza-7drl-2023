use bracket_lib::prelude::*;

use crate::{
    commands::Command,
    components::*,
    ui::{inventory_ui::InventoryUI, UI},
    State,
};

pub fn player_act(state: &mut State, key: VirtualKeyCode) -> bool {
    let act: Option<Command> = match key {
        VirtualKeyCode::H | VirtualKeyCode::Left => Some(Command::Move {
            target: Point::new(-1, 0),
        }),
        VirtualKeyCode::L | VirtualKeyCode::Right => Some(Command::Move {
            target: Point::new(1, 0),
        }),
        VirtualKeyCode::K | VirtualKeyCode::Up => Some(Command::Move {
            target: Point::new(0, -1),
        }),
        VirtualKeyCode::J | VirtualKeyCode::Down => Some(Command::Move {
            target: Point::new(0, 1),
        }),
        VirtualKeyCode::Y => Some(Command::Move {
            target: Point::new(-1, -1),
        }),
        VirtualKeyCode::U => Some(Command::Move {
            target: Point::new(1, -1),
        }),
        VirtualKeyCode::B => Some(Command::Move {
            target: Point::new(-1, 1),
        }),
        VirtualKeyCode::N => Some(Command::Move {
            target: Point::new(1, 1),
        }),
        VirtualKeyCode::G => Some(Command::Grab),
        VirtualKeyCode::I => Some(Command::OpenInventory),
        _ => None,
    };
    match act {
        Some(Command::Move { target: move_pt }) => {
            let position = state
                .ecs
                .query_one_mut::<&mut Position>(state.player_entity)
                .unwrap();
            let new_pt = position.0 + move_pt;
            let new_idx = state.map.point2d_to_index(new_pt);
            if state.map.is_available_exit(new_idx) {
                position.0 = new_pt;
                if let Ok(viewer) = state.ecs.query_one_mut::<&mut Viewer>(state.player_entity) {
                    viewer.dirty = true;
                }
            }
            true
        }
        Some(Command::Grab) => {
            let position = state
                .ecs
                .query_one_mut::<&Position>(state.player_entity)
                .unwrap();
            let mut items = Vec::new();
            for item in state.map.tile_contents[state.map.point2d_to_index(position.0)].iter() {
                if state
                    .ecs
                    .satisfies::<(&Item, &Position)>(*item)
                    .unwrap_or(false)
                {
                    items.push(*item);
                }
            }
            if let Some(item) = items.first() {
                state.ecs.remove_one::<Position>(*item).unwrap(); // we already ascertained that it has a component
                let inv = state
                    .ecs
                    .query_one_mut::<&mut Inventory>(state.player_entity)
                    .unwrap();
                inv.contents.push(*item);
                if let Some(name) = state.ecs.query_one_mut::<&Name>(*item).ok() {
                    state
                        .messages
                        .enqueue_message(&format!("You pick up a {}.", name.0));
                } else {
                    state.messages.enqueue_message("You pick something up.");
                }
                true
            } else {
                false
            }
        }
        Some(Command::OpenInventory) => {
            let inv = state
                .ecs
                .query_one_mut::<&mut Inventory>(state.player_entity)
                .unwrap();
            state.ui = UI::Inventory {
                ui: InventoryUI {
                    selection: 0,
                    length: inv.contents.len() as u32,
                },
            };
            false
        }
        None => false,
    }
}
