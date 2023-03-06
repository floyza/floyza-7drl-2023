use std::collections::VecDeque;

use bracket_lib::prelude::*;
use components::*;
use hecs::{Entity, World};
use map::{item_fill_map, populate_map};
use messages::MessageLog;
use monster::monster_act;
use window::Window;

pub mod blockers;
pub mod commands;
pub mod components;
pub mod components_serde;
pub mod item;
pub mod map;
pub mod messages;
pub mod monster;
pub mod player;
pub mod raws;
pub mod tile_contents;
pub mod ui;
pub mod viewer_look;
pub mod window;

pub struct State {
    pub size: Point,
    pub ecs: World,
    pub map: map::Map,
    pub player_entity: Entity,
    pub rng: RandomNumberGenerator,
    pub messages: MessageLog,
    pub has_moved: bool,
    pub window: window::Window,
    pub turn_order: VecDeque<Entity>,
    pub operating_mode: OperatingMode,
}

pub enum OperatingMode {
    WaitingForInput,
    Ticking,
}

impl State {
    fn run_systems(&mut self) {
        blockers::system_calc_blockers(self);
        tile_contents::system_tile_contents(self);
        viewer_look::system_calc_viewpoints(self);
    }
    fn render(&self, ctx: &mut BTerm) {
        ctx.cls();
        map::draw_map(self, ctx);
        ui::draw_messages(self, ctx);
        ui::draw_side_info(self, ctx);
        match &self.window {
            Window::None => {}
            Window::Inventory { window } => {
                window.render(self, ctx);
            }
            Window::MessageLog { window } => {
                window.render(self, ctx);
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
        loop {
            match self.operating_mode {
                OperatingMode::Ticking => {
                    let turn = self.turn_order.front();
                    if let Some(turn) = turn {
                        if self.ecs.satisfies::<&Player>(*turn).unwrap() {
                            self.operating_mode = OperatingMode::WaitingForInput;
                        } else if self.ecs.satisfies::<&Monster>(*turn).unwrap() {
                            monster_act(self, *turn);
                            self.run_systems();
                            self.turn_order.rotate_left(1);
                        } else {
                            panic!("Non-actor in the actor queue");
                        }
                    }
                }
                OperatingMode::WaitingForInput => {
                    if let Some(key) = ctx.key.take() {
                        match &mut self.window {
                            Window::None => {
                                let player_used_turn = player::player_act(self, key);
                                if player_used_turn {
                                    self.turn_order.rotate_left(1);
                                    self.operating_mode = OperatingMode::Ticking;
                                }
                                self.run_systems();
                            }
                            Window::Inventory { window } => {
                                if window.update(key) {
                                    self.window = Window::None;
                                }
                            }
                            Window::MessageLog { window } => {
                                if window.update(key) {
                                    self.window = Window::None;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        self.render(ctx);
    }
}

fn main() -> BError {
    raws::load_raws();

    let mut world = World::new();
    let map = map::Map::new();
    let player_pos = map.rooms[0].center();
    let player_entity = world.spawn((
        Health { max_hp: 30, hp: 30 },
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
        Grower::Empty,
        Attack { damage: 10 },
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

    let mut state = State {
        size: Point::new(80, 50),
        ecs: world,
        map,
        player_entity,
        rng: RandomNumberGenerator::new(),
        messages: MessageLog {
            log: Vec::new(),
            current_messages: Vec::new(),
        },
        has_moved: false,
        window: Window::None,
        turn_order: VecDeque::new(),
        operating_mode: OperatingMode::Ticking,
    };

    state.turn_order.push_back(player_entity);

    populate_map(&mut state);
    item_fill_map(&mut state);

    let context = BTermBuilder::simple80x50()
        .with_title("Be what you sow")
        .build()?;
    main_loop(context, state)
}
