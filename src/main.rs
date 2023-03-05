use bracket_lib::prelude::*;
use components::*;
use hecs::{Entity, World};
use messages::MessageLog;
use ui::UI;

pub mod commands;
pub mod components;
pub mod map;
pub mod messages;
pub mod player;
pub mod tile_contents;
pub mod ui;
pub mod viewer_look;

pub struct State {
    pub ecs: World,
    pub map: map::Map,
    pub player_entity: Entity,
    pub rng: RandomNumberGenerator,
    pub messages: MessageLog,
    pub has_moved: bool,
    pub ui: ui::UI,
}

impl State {
    fn run_systems(&mut self) {
        tile_contents::system_tile_contents(self);
        viewer_look::system_calc_viewpoints(self);
        messages::handle_messages(self); // HMM TODO
    }
    fn render(&self, ctx: &mut BTerm) {
        ctx.cls();
        map::draw_map(self, ctx);
        match &self.ui {
            UI::Playing => {}
            UI::Inventory { ui } => {
                ui.render(self, ctx);
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if !self.has_moved {
            self.has_moved = true;
            self.run_systems();
            self.render(ctx);
        }
        if let Some(key) = ctx.key {
            match &mut self.ui {
                UI::Playing => {
                    let player_used_turn = player::player_act(self, key);
                    if player_used_turn { /* monsters act here */ }
                    self.run_systems();
                    self.render(ctx);
                }
                UI::Inventory { ui } => {
                    let inventory = self
                        .ecs
                        .query_one_mut::<&Inventory>(self.player_entity)
                        .unwrap();
                    match key {
                        VirtualKeyCode::K | VirtualKeyCode::Up => {
                            ui.selection = std::cmp::max(0, ui.selection - 1);
                        }
                        VirtualKeyCode::J | VirtualKeyCode::Down => {
                            ui.selection = std::cmp::min(
                                inventory.contents.len() as i32 - 1,
                                ui.selection + 1,
                            );
                        }
                        VirtualKeyCode::Escape | VirtualKeyCode::Q => {
                            self.ui = UI::Playing;
                        }
                        _ => {}
                    }
                    self.render(ctx);
                }
            }
        }
    }
}

fn main() -> BError {
    let mut world = World::new();
    let map = map::Map::new();
    let player_pos = map.rooms[0].center();
    let player_entity = world.spawn((
        Health(30),
        Position(player_pos),
        Player {},
        Viewer {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        },
        Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(GREEN),
            bg: RGB::named(BLACK),
            layer: 1,
        },
        Inventory {
            contents: Vec::new(),
        },
        Name("Bob".to_string()),
    ));

    // generate some simple stuff for testing
    world.spawn((
        Position(map.rooms[0].center() + Point::new(0, 1)),
        Item {},
        Renderable {
            glyph: to_cp437('!'),
            fg: RGB::named(RED),
            bg: RGB::named(BLACK),
            layer: 0,
        },
        Name("Potion of Redness".to_string()),
    ));
    world.spawn((
        Position(map.rooms[0].center() + Point::new(1, 1)),
        Item {},
        Renderable {
            glyph: to_cp437('!'),
            fg: RGB::named(BLUE),
            bg: RGB::named(BLACK),
            layer: 0,
        },
        Name("Potion of Blueness".to_string()),
    ));

    let state = State {
        ecs: world,
        map,
        player_entity,
        rng: RandomNumberGenerator::new(),
        messages: MessageLog {
            log: Vec::new(),
            queue: Vec::new(),
        },
        has_moved: false,
        ui: UI::Playing,
    };

    let context = BTermBuilder::simple80x50()
        .with_title("Be what you sow")
        .build()?;
    main_loop(context, state)
}
