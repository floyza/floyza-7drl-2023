use crate::{components::*, debug, map, mapping::Command, OperatingMode, State};
use bracket_lib::prelude::*;

pub const SIDEBAR_EXTRA_POS: Point = Point { x: 1, y: 10 };
pub const LEFT_SIDEBAR_WIDTH: i32 = 20;
pub const RIGHT_SIDEBAR_WIDTH: i32 = 20;
pub const MESSAGE_LOG_HEIGHT: i32 = 8;

pub fn draw_corners(state: &State, ctx: &mut BTerm) {
    ctx.set(
        RIGHT_SIDEBAR_WIDTH - 1,
        state.size.y - MESSAGE_LOG_HEIGHT,
        RGB::named(WHITE),
        RGB::named(BLACK),
        to_cp437('├'),
    );
    ctx.set(
        state.size.x - LEFT_SIDEBAR_WIDTH,
        state.size.y - MESSAGE_LOG_HEIGHT,
        RGB::named(WHITE),
        RGB::named(BLACK),
        to_cp437('┤'),
    );
}

pub fn draw_messages(state: &State, ctx: &mut BTerm) {
    for (i, message) in state
        .messages
        .current_messages
        .iter()
        .rev()
        .take(MESSAGE_LOG_HEIGHT as usize - 1)
        .enumerate()
    {
        ctx.print(
            RIGHT_SIDEBAR_WIDTH + 1,
            state.size.y - 1 - i as i32,
            message,
        );
    }
    for x in RIGHT_SIDEBAR_WIDTH..state.size.x - LEFT_SIDEBAR_WIDTH {
        ctx.set(
            x,
            state.size.y - MESSAGE_LOG_HEIGHT,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('─'),
        );
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

    let bar_width = LEFT_SIDEBAR_WIDTH - 3;

    ctx.draw_bar_horizontal(
        hp_x,
        hp_y + 1,
        bar_width,
        health.hp,
        health.max_hp,
        RGB::named(RED),
        RGB::named(GRAY),
    );

    for y in 0..state.size.y {
        ctx.set(
            RIGHT_SIDEBAR_WIDTH - 1,
            y,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('│'),
        );
    }
}

pub fn draw_current_blueprint(state: &State, ctx: &mut BTerm) {
    // images are 17x30
    let sidebar_x = state.size.x - RIGHT_SIDEBAR_WIDTH;
    for y in 0..state.size.y {
        ctx.set(
            sidebar_x,
            y,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('│'),
        );
    }
    let mut query = state.ecs.query_one::<&Player>(state.player_entity).unwrap();
    let bp = &query.get().unwrap().current_blueprint;
    if let Some(bp) = bp {
        ctx.render_xp_sprite(&bp.img.lookup().img, sidebar_x + 2, 1);
    } else {
        ctx.print(sidebar_x + 1, 1, "No active blueprint");
    }
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

#[derive(Debug, Clone)]
pub enum InvUIRes {
    Select(u32),
    Done,
}

pub fn update_inventory_ui(
    mut state: InvUIState,
    command: Command,
) -> (Option<InvUIRes>, InvUIState) {
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
            return (Some(InvUIRes::Done), state);
        }
        Command::Select => {
            return (Some(InvUIRes::Select(state.selection)), state);
        }
        _ => {}
    }
    return (None, state);
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
    /// relative to map display window
    pub point: Point,
}

pub fn update_examine_ui(mut ui_state: ExamineUIState, command: Command) -> (bool, ExamineUIState) {
    match command {
        Command::Move { target: offset } => {
            let n = ui_state.point + offset;
            if n.x >= 0
                && n.y >= 0
                && n.x < map::MAP_UI_DIM.width()
                && n.y < map::MAP_UI_DIM.height()
            {
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
    let mut query = state
        .ecs
        .query_one::<&Position>(state.player_entity)
        .unwrap();
    let player_pos = query.get().unwrap().0;
    let offset = player_pos - map::MAP_UI_DIM.center();
    let top_left = Point::new(map::MAP_UI_DIM.x1, map::MAP_UI_DIM.y1);
    ctx.set(
        ui_state.point.x + top_left.x,
        ui_state.point.y + top_left.y,
        RGB::named(PURPLE),
        RGB::named(BLACK),
        to_cp437('*'),
    );
    ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y, "You see:");
    let idx = state
        .map
        .point2d_to_index(ui_state.point + top_left + offset);
    if state.map.visible_tiles[idx] {
        let mut line = 0;
        for entity in state.map.tile_contents[idx].iter() {
            let mut query = state.ecs.query_one::<&Name>(*entity).unwrap();
            if let Some(name) = query.get() {
                ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1 + line, &name.0);
                line += 1;
                if state.debug {
                    let stuff = debug::get_entity_components(state.ecs.entity(*entity).unwrap());
                    for comp in stuff {
                        comp.apply(|c| {
                            ctx.print(
                                SIDEBAR_EXTRA_POS.x,
                                SIDEBAR_EXTRA_POS.y + 1 + line,
                                format!("-> {:?}", c),
                            );
                        });
                        line += 1;
                    }
                }
            }
        }
        match state.map.tiles[idx] {
            map::Tile::Wall => {
                ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1 + line, "Wall");
            }
            map::Tile::Floor => {
                ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1 + line, "Floor");
            }
            map::Tile::Stairs => {
                ctx.print(
                    SIDEBAR_EXTRA_POS.x,
                    SIDEBAR_EXTRA_POS.y + 1 + line,
                    "Stairs",
                );
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
            map::Tile::Stairs => {
                ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1, "Stairs");
            }
        }
    } else {
        ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1, "Nothing");
    }
}
