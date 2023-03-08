use crate::{components::*, OperatingMode, State};
use bracket_lib::prelude::*;

pub fn draw_messages(state: &State, ctx: &mut BTerm) {
    for (i, message) in state
        .messages
        .current_messages
        .iter()
        .rev()
        .take(4)
        .enumerate()
    {
        ctx.print(0, state.size.y - 1 - i as i32, message);
    }
}

pub fn draw_side_info(state: &State, ctx: &mut BTerm) {
    let mut query = state.ecs.query_one::<&Health>(state.player_entity).unwrap();
    let health = query.get().unwrap();

    let base_x = 1;
    let base_y = 1;

    ctx.print(
        base_x,
        base_y,
        format!("Health: {}/{}", health.hp, health.max_hp),
    );

    let bar_width = 15;

    ctx.draw_bar_horizontal(
        base_x,
        base_y + 1,
        bar_width,
        health.hp,
        health.max_hp,
        RGB::named(RED),
        RGB::named(GRAY),
    );
}

pub fn update_message_log(key: VirtualKeyCode) -> bool {
    match key {
        VirtualKeyCode::Escape | VirtualKeyCode::Q => {
            return true;
        }
        _ => {}
    }
    return false;
}

pub fn draw_message_log(state: &State, ctx: &mut BTerm) {
    let x = 5;
    let y = 5;
    let w = 50;
    let h = 20;
    ctx.draw_box(x, y, w, h, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print_centered_at(x + w / 2, y, "Message Log");

    for (i, message) in state.messages.log.iter().rev().enumerate() {
        let line = y + h - 1 - i;
        if line == y {
            break;
        }
        ctx.print(x + 1, line, message);
    }
}

#[derive(Clone)]
pub struct InvUIState {
    pub selection: u32,
    pub length: u32,
}

pub fn update_inventory_ui(mut state: InvUIState, key: VirtualKeyCode) -> (bool, InvUIState) {
    match key {
        VirtualKeyCode::K | VirtualKeyCode::Up => {
            if state.selection > 0 {
                state.selection -= 1;
            }
        }
        VirtualKeyCode::J | VirtualKeyCode::Down => {
            if state.selection + 1 < state.length {
                state.selection += 1;
            }
        }
        VirtualKeyCode::Escape | VirtualKeyCode::Q => {
            return (true, state);
        }
        _ => {}
    }
    return (false, state);
}

pub fn draw_inventory_ui(ui_state: &InvUIState, state: &State, ctx: &mut BTerm) {
    let x = 5;
    let y = 5;
    let w = 30;
    let h = 20;
    ctx.draw_box(x, y, w, h, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print_centered_at(x + w / 2, y, "Inventory");
    let mut inv_query = state
        .ecs
        .query_one::<&Inventory>(state.player_entity)
        .unwrap();
    let inv = inv_query.get().unwrap();
    for (idx, item) in inv.contents.iter().enumerate() {
        let mut name_query = state.ecs.query_one::<&Name>(*item).unwrap();
        let name = name_query
            .get()
            .map(|n| n.0.clone())
            .unwrap_or("UNNAMED_OBJECT".to_string());
        let line = y + 1 + idx;
        if ui_state.selection == idx as u32 {
            ctx.set(
                x + 1,
                line,
                RGB::named(WHITE),
                RGB::named(BLACK),
                to_cp437('>'),
            );
        } else {
            ctx.set(
                x + 1,
                line,
                RGB::named(WHITE),
                RGB::named(BLACK),
                to_cp437('-'),
            );
        }
        ctx.print(x + 2, line, name);
    }
}
