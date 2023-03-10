use crate::{
    components::*, debug, equipment::print_desc, map, mapping::Command, State, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use bracket_lib::prelude::*;

pub const SIDEBAR_EXTRA_POS: Point = Point { x: 1, y: 30 };
pub const LEFT_SIDEBAR_WIDTH: i32 = 20;
pub const RIGHT_SIDEBAR_WIDTH: i32 = 20;
pub const MESSAGE_LOG_HEIGHT: i32 = 8;

pub fn draw_corners(ctx: &mut BTerm) {
    ctx.set(
        RIGHT_SIDEBAR_WIDTH - 1,
        WINDOW_HEIGHT - MESSAGE_LOG_HEIGHT,
        RGB::named(WHITE),
        RGB::named(BLACK),
        to_cp437('├'),
    );
    ctx.set(
        WINDOW_WIDTH - LEFT_SIDEBAR_WIDTH,
        WINDOW_HEIGHT - MESSAGE_LOG_HEIGHT,
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
            WINDOW_HEIGHT - 1 - i as i32,
            message,
        );
    }
    for x in RIGHT_SIDEBAR_WIDTH..WINDOW_WIDTH - LEFT_SIDEBAR_WIDTH {
        ctx.set(
            x,
            WINDOW_HEIGHT - MESSAGE_LOG_HEIGHT,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('─'),
        );
    }
}

pub fn draw_side_info(state: &State, ctx: &mut BTerm) {
    let mut query = state
        .ecs
        .query_one::<(&Health, &Player)>(state.player_entity)
        .unwrap();
    let (health, player) = query.get().unwrap();

    ctx.print(1, 1, format!("Depth: {}", state.map.depth + 1));
    ctx.print(1, 2, format!("Health: {}/{}", health.hp, health.max_hp));

    let bar_width = LEFT_SIDEBAR_WIDTH - 3;

    ctx.draw_bar_horizontal(
        1,
        3,
        bar_width,
        health.hp,
        health.max_hp,
        RGB::named(RED),
        RGB::named(GRAY),
    );

    let mut line = 5;
    ctx.print(1, line, "Actives:");
    line += 1;
    for (i, eq) in player.active_equipment.iter().enumerate() {
        let eq = eq.as_ref().unwrap(); // we are not ever rendering while executing effects
        ctx.print(1, line, format!("{}){:?}", i + 1, eq.ingredients.0));
        line += 1;
    }
    line += 1;
    ctx.print(1, line, "Passives:");
    line += 1;
    for eq in player.passive_equipment.iter() {
        let eq = eq.as_ref().unwrap(); // we are not ever rendering while executing effects
        ctx.print(1, line, format!("{:?}", eq.ingredients.0));
        line += 1;
    }

    for y in 0..WINDOW_HEIGHT {
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
    let sidebar_x = WINDOW_WIDTH - RIGHT_SIDEBAR_WIDTH;
    for y in 0..WINDOW_HEIGHT {
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
        let offset_x = 2;
        let offset_y = 1;
        let bpi = bp.img.lookup();
        ctx.render_xp_sprite(&bpi.img, sidebar_x + offset_x, offset_y);
        for slot in bp.filled.iter() {
            let gem = bpi.gem_spots[slot.0];
            let color = match slot.1.element {
                Elemental::Air => RGB::named(SKYBLUE),
                Elemental::Water => RGB::named(BLUE3),
                Elemental::Fire => RGB::named(RED),
            };
            ctx.set(
                sidebar_x + offset_x + gem.x,
                offset_y + gem.y,
                color,
                RGB::named(BLACK),
                to_cp437('☼'),
            );
        }
        {
            let mut ess = vec![];
            for i in 0..bpi.gem_spots.len() {
                if let Some(essence) = bp.filled.iter().find(|p| p.0 == i) {
                    ess.push(Some(essence.1.clone()));
                } else {
                    ess.push(None);
                }
            }
            let mut builder = TextBuilder::empty();
            print_desc(bp.equipment, &ess, &mut builder);
            let mut block =
                TextBlock::new(sidebar_x + 1, offset_y + 30, RIGHT_SIDEBAR_WIDTH - 1, 5);
            block
                .print(&builder)
                .expect("Description text was too long");
            let mut draw_batch = DrawBatch::new();
            block.render_to_draw_batch(&mut draw_batch);
            draw_batch.submit(0).unwrap();
            render_draw_buffer(ctx).unwrap();
        }
        if bp.filled.len() == bpi.gem_spots.len() {
            ctx.print(sidebar_x + 1, offset_y + 30 + 5, "Artifact ready!");
            ctx.print(sidebar_x + 1, offset_y + 30 + 5 + 1, "Press 'a' to forge!");
        }
    } else {
        ctx.print(sidebar_x + 1, 1, "No active artifact");
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
    let x = 20;
    let y = 5;
    let w = 50;
    let h = 50;
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

pub fn update_game_over(command: Command) -> bool {
    match command {
        Command::Select => true,
        _ => false,
    }
}

pub fn draw_game_over(ctx: &mut BTerm) {
    let x = 4;
    let y = 4;
    let w = 92;
    let h = 62;
    ctx.draw_box(x, y, w, h, RGB::named(WHITE), RGB::named(BLACK));
    let xpfile = XpFile::from_resource("../assets/game-over.xp").unwrap();
    ctx.render_xp_sprite(&xpfile, x + 1, y + 1);
}

pub fn draw_game_won(ctx: &mut BTerm) {
    let x = 4;
    let y = 4;
    let w = 92;
    let h = 62;
    ctx.draw_box(x, y, w, h, RGB::named(WHITE), RGB::named(BLACK));
    let xpfile = XpFile::from_resource("../assets/you-won.xp").unwrap();
    ctx.render_xp_sprite(&xpfile, x + 1, y + 1);
}

pub fn update_help(command: Command) -> bool {
    match command {
        Command::Back => return true,
        _ => return false,
    }
}

const HELP_CONTENTS: &str = include_str!("../raws/help.txt");

pub fn draw_help(ctx: &mut BTerm) {
    let x = 4;
    let y = 4;
    let w = 92;
    let h = 62;
    ctx.draw_box(x, y, w, h, RGB::named(WHITE), RGB::named(BLACK));
    let mut builder = TextBuilder::empty();
    {
        let stuff: Vec<&str> = HELP_CONTENTS.split('\n').collect();
        for line in stuff {
            builder.line_wrap(line).ln();
        }
    }
    let mut block = TextBlock::new(x + 1, y + 1, w - 2, h - 2);
    block.print(&builder).expect("Too long of help text");
    let mut draw_batch = DrawBatch::new();
    block.render_to_draw_batch(&mut draw_batch);
    draw_batch.submit(0).unwrap();
    render_draw_buffer(ctx).unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct MainMenuState {
    pub selection: i32,
    pub xpfile: XpFile,
    pub looking_at_help: bool,
}

pub fn update_main_menu(mut ui_state: MainMenuState, command: Command) -> (bool, MainMenuState) {
    if ui_state.looking_at_help {
        let done = update_help(command);
        if done {
            ui_state.looking_at_help = false;
            return (false, ui_state);
        }
    }
    match command {
        Command::Move {
            target: Point { x: 0, y: -1 },
        } => {
            if ui_state.selection > 0 {
                ui_state.selection -= 1;
            };
        }
        Command::Move {
            target: Point { x: 0, y: 1 },
        } => {
            if ui_state.selection + 1 < 2 {
                ui_state.selection += 1;
            };
        }
        Command::Select => {
            if ui_state.selection == 0 {
                return (true, ui_state);
            } else {
                ui_state.looking_at_help = true;
                return (false, ui_state);
            }
        }
        _ => {}
    }
    return (false, ui_state);
}

pub fn draw_main_menu(ui_state: &MainMenuState, _state: &State, ctx: &mut BTerm) {
    if ui_state.looking_at_help {
        draw_help(ctx);
        return;
    }
    let x = 4;
    let y = 4;
    let w = 92;
    let h = 62;
    ctx.draw_box(x, y, w, h, RGB::named(WHITE), RGB::named(BLACK));
    ctx.render_xp_sprite(&ui_state.xpfile, x + 1, y + 1);
    ctx.set(
        x + 1 + 29,
        y + 1 + 20 + ui_state.selection * 2,
        RGB::from_hex("#69008C").unwrap(),
        RGB::named(BLACK),
        to_cp437('>'),
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct EquipExamineState {
    pub selection: i32,
    pub length: i32,
}

pub fn draw_equip_examine(ui_state: &EquipExamineState, state: &State, ctx: &mut BTerm) {
    let mut query = state.ecs.query_one::<&Player>(state.player_entity).unwrap();
    let player = query.get().unwrap();

    let equip;
    if ui_state.selection < player.active_equipment.len() as i32 {
        equip = &player.active_equipment[ui_state.selection as usize];
        ctx.set(
            0,
            6 + ui_state.selection,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('>'),
        );
    } else {
        let sel = ui_state.selection - player.active_equipment.len() as i32;
        if !(sel < player.passive_equipment.len() as i32) {
            return;
        }
        equip = &player.passive_equipment[sel as usize];
        ctx.set(
            0,
            8 + ui_state.selection as i32,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437('>'),
        );
    }
    ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y, "Equipped:");
    let Some(equip) = equip else {return};
    let bpi = equip.img.lookup();
    ctx.render_xp_sprite(&bpi.img, SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1);
    for (i, slot) in equip.ingredients.1.iter().enumerate() {
        let gem = bpi.gem_spots[i];
        let color = match slot.element {
            Elemental::Air => RGB::named(SKYBLUE),
            Elemental::Water => RGB::named(BLUE3),
            Elemental::Fire => RGB::named(RED),
        };
        ctx.set(
            SIDEBAR_EXTRA_POS.x + gem.x,
            SIDEBAR_EXTRA_POS.y + gem.y + 1,
            color,
            RGB::named(BLACK),
            to_cp437('☼'),
        );
        let mut builder = TextBuilder::empty();
        let ess = equip
            .ingredients
            .1
            .iter()
            .map(|x| Some(x.clone()))
            .collect();
        print_desc(equip.ingredients.0, &ess, &mut builder);
        let mut block = TextBlock::new(
            SIDEBAR_EXTRA_POS.x,
            SIDEBAR_EXTRA_POS.y + 30,
            LEFT_SIDEBAR_WIDTH - 2,
            5,
        );
        block
            .print(&builder)
            .expect("Description text was too long");
        let mut draw_batch = DrawBatch::new();
        block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(0).unwrap();
        render_draw_buffer(ctx).unwrap();
    }
}

pub fn update_equip_examine(
    mut ui_state: EquipExamineState,
    _state: &mut State,
    command: Command,
) -> (bool, EquipExamineState) {
    match command {
        Command::Move {
            target: Point { x: 0, y: -1 },
        } => {
            if ui_state.selection > 0 {
                ui_state.selection -= 1;
            };
        }
        Command::Move {
            target: Point { x: 0, y: 1 },
        } => {
            if ui_state.selection + 1 < ui_state.length {
                ui_state.selection += 1;
            };
        }
        Command::Back => {
            return (true, ui_state);
        }
        _ => {}
    }
    return (false, ui_state);
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfUIState {
    pub query: String,
    pub selection: bool,
}

#[derive(Debug, Clone)]
pub enum ConfUIRes {
    Yes,
    No,
}

pub fn update_confirmation_ui(
    mut ui_state: ConfUIState,
    _state: &mut State,
    command: Command,
) -> (Option<ConfUIRes>, ConfUIState) {
    match command {
        Command::Move {
            target: Point { x: 0, y: -1 },
        } => {
            ui_state.selection = true;
        }
        Command::Move {
            target: Point { x: 0, y: 1 },
        } => {
            ui_state.selection = false;
        }
        Command::Select => {
            if ui_state.selection {
                return (Some(ConfUIRes::Yes), ui_state);
            } else {
                return (Some(ConfUIRes::No), ui_state);
            }
        }
        Command::Back => {
            return (Some(ConfUIRes::No), ui_state);
        }
        _ => {}
    }
    (None, ui_state)
}

pub fn draw_confirmation_ui(ui_state: &ConfUIState, ctx: &mut BTerm) {
    let width = ui_state.query.len() as i32 + 4;
    let height = 4;
    let x = (WINDOW_WIDTH - width) / 2;
    let y = (WINDOW_HEIGHT - height) / 2;
    ctx.draw_box(x, y, width, height, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print(x + 2, y + 1, &ui_state.query);
    if ui_state.selection {
        ctx.print(x + 1, y + 2, ">yes");
        ctx.print(x + 1, y + 3, " no");
    } else {
        ctx.print(x + 1, y + 2, " yes");
        ctx.print(x + 1, y + 3, ">no");
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvUIState {
    pub selection: u32,
    pub length: u32,
    pub confirming: Option<ConfUIState>,
}

#[derive(Debug, Clone)]
pub enum InvUIRes {
    Select(u32),
    Done,
}

pub fn update_inventory_ui(
    mut ui_state: InvUIState,
    state: &mut State,
    command: Command,
) -> (Option<InvUIRes>, InvUIState) {
    if let Some(confirming) = ui_state.confirming {
        let (res, state2) = update_confirmation_ui(confirming, state, command);
        match res {
            Some(ConfUIRes::Yes) => {
                ui_state.confirming = None;
                return (Some(InvUIRes::Select(ui_state.selection)), ui_state);
            }
            Some(ConfUIRes::No) => {
                ui_state.confirming = None;
                return (None, ui_state);
            }
            None => {
                ui_state.confirming = Some(state2);
                return (None, ui_state);
            }
        }
    }
    match command {
        Command::Move {
            target: Point { x: 0, y: -1 },
        } => {
            if ui_state.selection > 0 {
                ui_state.selection -= 1;
            }
        }
        Command::Move {
            target: Point { x: 0, y: 1 },
        } => {
            if ui_state.selection + 1 < ui_state.length {
                ui_state.selection += 1;
            }
        }
        Command::Back => {
            return (Some(InvUIRes::Done), ui_state);
        }
        Command::Select => {
            let p = state
                .ecs
                .query_one_mut::<&Player>(state.player_entity)
                .unwrap();
            if p.current_blueprint.is_some() {
                ui_state.confirming = Some(ConfUIState {
                    query: "Are you sure? This will delete the existing artifact.".to_owned(),
                    selection: false,
                });
            } else {
                return (Some(InvUIRes::Select(ui_state.selection)), ui_state);
            }
        }
        _ => {}
    }
    return (None, ui_state);
}

pub fn draw_inventory_ui(ui_state: &InvUIState, state: &State, ctx: &mut BTerm) {
    let x = 20;
    let y = 5;
    let w = 30;
    let h = 20;
    ctx.draw_box(x, y, w, h, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print_centered_at(x + w / 2, y, "Artifacts");
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
    if let Some(confirming) = &ui_state.confirming {
        draw_confirmation_ui(confirming, ctx);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExamineUIState {
    /// relative to map display window
    pub point: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExamineUIRes {
    Select(Point),
    Done,
}

pub fn update_examine_ui(
    mut ui_state: ExamineUIState,
    state: &mut State,
    command: Command,
) -> (Option<ExamineUIRes>, ExamineUIState) {
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
            return (Some(ExamineUIRes::Done), ui_state);
        }
        Command::Select => {
            let player_pos = state
                .ecs
                .query_one_mut::<&Position>(state.player_entity)
                .unwrap()
                .0;
            let offset = player_pos - map::MAP_UI_DIM.center();
            let top_left = Point::new(map::MAP_UI_DIM.x1, map::MAP_UI_DIM.y1);
            return (
                Some(ExamineUIRes::Select(ui_state.point + top_left + offset)),
                ui_state,
            );
        }
        _ => {}
    }
    return (None, ui_state);
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
    let mut line = 0;
    if state.debug {
        ctx.print(
            SIDEBAR_EXTRA_POS.x,
            SIDEBAR_EXTRA_POS.y + 1 + line,
            format!("@pos: {:?}", ui_state.point + top_left + offset),
        );
        line += 1;
    }
    if state.map.in_bounds(ui_state.point + top_left + offset) {
        let idx = state
            .map
            .point2d_to_index(ui_state.point + top_left + offset);
        if state.map.visible_tiles[idx] {
            for entity in state.map.tile_contents[idx].iter() {
                let mut query = state.ecs.query_one::<&Name>(*entity).unwrap();
                if let Some(name) = query.get() {
                    ctx.print(SIDEBAR_EXTRA_POS.x, SIDEBAR_EXTRA_POS.y + 1 + line, &name.0);
                    line += 1;
                    if state.debug {
                        let stuff =
                            debug::get_entity_components(state.ecs.entity(*entity).unwrap());
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
        } else {
            ctx.print(
                SIDEBAR_EXTRA_POS.x,
                SIDEBAR_EXTRA_POS.y + 1 + line,
                "Nothing",
            );
        }
    } else {
        ctx.print(
            SIDEBAR_EXTRA_POS.x,
            SIDEBAR_EXTRA_POS.y + 1 + line,
            "Nothing",
        );
    }
}

embedded_resource!(RES_MAIN_MENU, "../assets/main-menu.xp");
embedded_resource!(RES_GAME_OVER, "../assets/game-over.xp");
embedded_resource!(RES_YOU_WON, "../assets/you-won.xp");

pub fn load_menus_xp() {
    link_resource!(RES_MAIN_MENU, "../assets/main-menu.xp");
    link_resource!(RES_GAME_OVER, "../assets/game-over.xp");
    link_resource!(RES_YOU_WON, "../assets/you-won.xp");
}
