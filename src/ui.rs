use crate::{components::*, debug, map, mapping::Command, OperatingMode, State};
use bracket_lib::prelude::*;

const SIDEBAR_EXTRA_POS: Point = Point { x: 1, y: 10 };

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
    let mut query = state
        .ecs
        .query_one::<(&Health, &Name)>(state.player_entity)
        .unwrap();
    let (health, name) = query.get().unwrap();

    ctx.print(1, 1, format!("Name: {}", name.0));

    let hp_x = 1;
    let hp_y = 2;

    ctx.print(
        hp_x,
        hp_y,
        format!("Health: {}/{}", health.hp, health.max_hp),
    );

    let bar_width = 15;

    ctx.draw_bar_horizontal(
        hp_x,
        hp_y + 1,
        bar_width,
        health.hp,
        health.max_hp,
        RGB::named(RED),
        RGB::named(GRAY),
    );
}

pub fn update_message_log(command: Command) -> bool {
    match command {
        Command::Back => {
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

#[derive(Debug, Clone)]
pub struct InvUIState {
    pub selection: u32,
    pub length: u32,
}

pub fn update_inventory_ui(mut state: InvUIState, command: Command) -> (bool, InvUIState) {
    match command {
        Command::Move {
            target: Point { x: 0, y: -1 },
        } => {
            if state.selection > 0 {
                state.selection -= 1;
            }
        }
        Command::Move {
            target: Point { x: 0, y: 1 },
        } => {
            if state.selection + 1 < state.length {
                state.selection += 1;
            }
        }
        Command::Back => {
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

#[derive(Debug, Clone)]
pub struct ExamineUIState {
    pub point: Point,
}

pub fn update_examine_ui(
    mut ui_state: ExamineUIState,
    state: &State,
    command: Command,
) -> (bool, ExamineUIState) {
    match command {
        Command::Move { target: offset } => {
            if state.map.in_bounds(ui_state.point + offset) {
                ui_state.point += offset;
            }
        }
        Command::Back => {
            return (true, ui_state);
        }
        _ => {}
    }
    return (false, ui_state);
}

pub fn draw_examine_ui(ui_state: &ExamineUIState, state: &State, ctx: &mut BTerm) {
    ctx.set(
        ui_state.point.x,
        ui_state.point.y,
        RGB::named(PURPLE),
        RGB::named(BLACK),
        to_cp437('*'),
    );
    ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y, "You see:");
    let idx = state.map.point2d_to_index(ui_state.point);
    if state.map.visible_tiles[idx] {
        let mut success = false;
        let mut line = 0;
        for entity in state.map.tile_contents[idx].iter() {
            let mut query = state.ecs.query_one::<&Name>(*entity).unwrap();
            if let Some(name) = query.get() {
                ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1 + line, &name.0);
                line += 1;
                if state.debug {
                    let stuff = debug::get_entity_components(state.ecs.entity(*entity).unwrap());
                    for comp in stuff {
                        ctx.print(
                            SIDEBAR_EXTRA_POS.x,
                            SIDEBAR_EXTRA_POS.y + 1 + line,
                            format!("-> {:?}", comp),
                        );
                        line += 1;
                    }
                }
                success = true;
            }
        }
        if success {
            return;
        }
        match state.map.tiles[idx] {
            map::Tile::Wall => {
                ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1, "Wall");
            }
            map::Tile::Floor => {
                ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1, "Floor");
            }
        }
    } else if state.map.revealed_tiles[idx] {
        match state.map.tiles[idx] {
            map::Tile::Wall => {
                ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1, "Wall");
            }
            map::Tile::Floor => {
                ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1, "Floor");
            }
        }
    } else {
        ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1, "Nothing");
    }
}
