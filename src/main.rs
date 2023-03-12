use std::collections::VecDeque;

use bracket_lib::prelude::*;
use components::*;
use equipment::execute_active_target;
use hecs::{Entity, World};
use map::{item_fill_map, populate_map};
use messages::MessageLog;
use monster::monster_act;
use systems::*;
use ui::{draw_main_menu, update_main_menu, ExamineUIRes};

pub mod blueprint;
pub mod components;
pub mod components_serde;
pub mod debug;
pub mod equipment;
pub mod essence;
pub mod item;
pub mod map;
pub mod mapping;
pub mod math;
pub mod messages;
pub mod monster;
pub mod player;
pub mod raws;
pub mod skill;
pub mod systems;
pub mod ui;
pub mod util;

pub const WINDOW_WIDTH: i32 = 100;
pub const WINDOW_HEIGHT: i32 = 70;

pub struct State {
    pub ecs: World,
    pub map: map::Map,
    pub player_entity: Entity,
    pub rng: RandomNumberGenerator,
    pub messages: MessageLog,
    pub has_moved: bool,
    pub turn_order: VecDeque<Entity>,
    pub operating_mode: OperatingMode,
    pub debug: bool,
}

#[derive(Debug, PartialEq)]
pub enum OperatingMode {
    WaitingForInput,
    Ticking,
    OpenInventory(ui::InvUIState),
    OpenMessageLog,
    OpenExamine(ui::ExamineUIState),
    EquipmentTargetting {
        state: ui::ExamineUIState,
        equipment: usize,
    },
    MainMenu(ui::MainMenuState),
    GameOver,
    GameWon,
    EquipmentExamining(ui::EquipExamineState),
    HelpMenu,
}

impl State {
    fn run_systems(&mut self) {
        death::system_kill_dead(self);
        blockers::system_calc_blockers(self);
        tile_contents::system_tile_contents(self);
        viewer_look::system_calc_viewpoints(self);
    }
    fn render(&self, ctx: &mut BTerm) {
        ctx.cls();
        map::draw_map(self, ctx);
        ui::draw_messages(self, ctx);
        ui::draw_side_info(self, ctx);
        ui::draw_current_blueprint(self, ctx);
        ui::draw_corners(ctx);
        match &self.operating_mode {
            OperatingMode::Ticking => {}
            OperatingMode::WaitingForInput => {}
            OperatingMode::OpenInventory(s) => ui::draw_inventory_ui(s, self, ctx),
            OperatingMode::OpenMessageLog => ui::draw_message_log(self, ctx),
            OperatingMode::OpenExamine(s) => ui::draw_examine_ui(s, self, ctx),
            OperatingMode::EquipmentTargetting { state: s, .. } => {
                ui::draw_examine_ui(s, self, ctx)
            }
            OperatingMode::MainMenu(s) => {
                draw_main_menu(s, self, ctx);
            }
            OperatingMode::GameOver => {
                todo!();
            }
            OperatingMode::GameWon => {
                todo!();
            }
            OperatingMode::EquipmentExamining(s) => ui::draw_equip_examine(s, self, ctx),
            OperatingMode::HelpMenu => ui::draw_help(ctx),
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
            match &self.operating_mode {
                OperatingMode::Ticking => {
                    self.run_systems(); // sometimes we get here direct from ui
                    let turn = self.turn_order.front();
                    if let Some(turn) = turn {
                        if self.ecs.satisfies::<&Player>(*turn).unwrap() {
                            self.operating_mode = OperatingMode::WaitingForInput;
                        } else if self.ecs.satisfies::<&Monster>(*turn).unwrap() {
                            if let Some(slow) = self
                                .ecs
                                .query_one_mut::<Option<&mut Slowed>>(*turn)
                                .unwrap()
                            {
                                slow.duration -= 1;
                                if slow.duration % 2 == 0 {
                                    if slow.duration == 0 {
                                        self.ecs.remove_one::<Slowed>(*turn).unwrap();
                                    }
                                    self.turn_order.rotate_left(1);
                                    continue;
                                }
                            }
                            monster_act(self, *turn);
                            self.run_systems();
                            self.turn_order.rotate_left(1);
                        } else {
                            panic!("Non-actor in the actor queue");
                        }
                    }
                }
                OperatingMode::WaitingForInput => {
                    if let Some(command) = mapping::get_command(ctx) {
                        let player_used_turn = player::player_act(self, &command);
                        if player_used_turn {
                            self.turn_order.rotate_left(1);
                            if self.operating_mode == OperatingMode::WaitingForInput {
                                // if unchanged
                                self.operating_mode = OperatingMode::Ticking;
                            }
                        }
                        // self.run_systems();
                    } else {
                        break;
                    }
                }
                OperatingMode::OpenInventory(s) => {
                    if let Some(command) = mapping::get_command(ctx) {
                        let (ret, s) = ui::update_inventory_ui(s.clone(), self, command);
                        if let Some(ret) = ret {
                            match ret {
                                ui::InvUIRes::Select(idx) => {
                                    let inv = self
                                        .ecs
                                        .query_one_mut::<&Inventory>(self.player_entity)
                                        .unwrap();
                                    let item = inv.contents[idx as usize];
                                    if let Ok((bp, name)) =
                                        self.ecs.query_one_mut::<(&Blueprint, &Name)>(item)
                                    {
                                        self.messages.enqueue_message(&format!(
                                            "You attach the {}.",
                                            name.0
                                        ));
                                        let bp = bp.clone();
                                        let (p, inv) = self
                                            .ecs
                                            .query_one_mut::<(&mut Player, &mut Inventory)>(
                                                self.player_entity,
                                            )
                                            .unwrap();
                                        p.current_blueprint = Some(bp);
                                        inv.contents.remove(idx as usize);
                                        self.turn_order.rotate_left(1);
                                        self.operating_mode = OperatingMode::Ticking;
                                    } else {
                                        let name = self.ecs.query_one_mut::<&Name>(item).unwrap();
                                        self.messages.enqueue_message(&format!(
                                            "Could not attach {}: it's not a artifact.",
                                            name.0
                                        ));
                                        self.operating_mode = OperatingMode::Ticking;
                                    }
                                }
                                ui::InvUIRes::Done => {
                                    self.operating_mode = OperatingMode::Ticking;
                                }
                            }
                        } else {
                            self.operating_mode = OperatingMode::OpenInventory(s);
                        }
                    } else {
                        break;
                    }
                }
                OperatingMode::OpenMessageLog => {
                    if let Some(command) = mapping::get_command(ctx) {
                        if ui::update_message_log(command) {
                            self.operating_mode = OperatingMode::Ticking;
                        }
                    } else {
                        break;
                    }
                }
                OperatingMode::OpenExamine(s) => {
                    if let Some(command) = mapping::get_command(ctx) {
                        let (done, s) = ui::update_examine_ui(s.clone(), self, command);
                        if done == Some(ExamineUIRes::Done) {
                            self.operating_mode = OperatingMode::Ticking;
                        } else {
                            self.operating_mode = OperatingMode::OpenExamine(s);
                        }
                    } else {
                        break;
                    }
                }
                OperatingMode::EquipmentTargetting {
                    state: s,
                    equipment,
                } => {
                    let equipment = equipment.clone();
                    if let Some(command) = mapping::get_command(ctx) {
                        let (done, s) = ui::update_examine_ui(s.clone(), self, command);
                        match done {
                            Some(ExamineUIRes::Done) => {
                                self.operating_mode = OperatingMode::Ticking;
                            }
                            Some(ExamineUIRes::Select(pt)) => {
                                execute_active_target(self, equipment, pt);
                                self.turn_order.rotate_left(1);
                                self.operating_mode = OperatingMode::Ticking;
                            }
                            None => {
                                self.operating_mode = OperatingMode::EquipmentTargetting {
                                    state: s,
                                    equipment,
                                };
                            }
                        }
                    } else {
                        break;
                    }
                }
                OperatingMode::MainMenu(s) => {
                    if let Some(command) = mapping::get_command(ctx) {
                        let (res, s) = update_main_menu(s.clone(), command);
                        if res {
                            self.operating_mode = OperatingMode::Ticking;
                        } else {
                            self.operating_mode = OperatingMode::MainMenu(s);
                        }
                    } else {
                        break;
                    }
                }
                OperatingMode::GameOver => {
                    if let Some(command) = mapping::get_command(ctx) {
                        todo!();
                    } else {
                        break;
                    }
                }
                OperatingMode::GameWon => {
                    if let Some(command) = mapping::get_command(ctx) {
                        todo!();
                    } else {
                        break;
                    }
                }
                OperatingMode::EquipmentExamining(s) => {
                    if let Some(command) = mapping::get_command(ctx) {
                        let (done, s) = ui::update_equip_examine(s.clone(), self, command);
                        if done {
                            self.operating_mode = OperatingMode::Ticking;
                        } else {
                            self.operating_mode = OperatingMode::EquipmentExamining(s);
                        }
                    } else {
                        break;
                    }
                }
                OperatingMode::HelpMenu => {
                    if let Some(command) = mapping::get_command(ctx) {
                        let done = ui::update_help(command);
                        if done {
                            self.operating_mode = OperatingMode::Ticking;
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
    blueprint::load_blueprints();
    ui::load_menus_xp();

    let mut rng = RandomNumberGenerator::new();
    let mut world = World::new();
    let map = map::Map::new(0, &mut rng);
    // let map = map::Map::make_last_room(&mut rng);
    let player_pos = map.rooms[0].center();
    let bp: Blueprint = serde_json::from_str(
        r##"{ "img": "Gun", "equipment": "Gun", "filled": [[0, {"element":"Air", "power":2}]] }"##,
    )
    .unwrap();
    let equip = equipment::build_blueprint(&bp);
    let player_entity = world.spawn((
        Health { max_hp: 30, hp: 30 },
        Position(player_pos),
        Player {
            current_blueprint: None,
            passive_equipment: vec![],
            active_equipment: vec![Some(equip)],
        },
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
        Attack { damage: 3 },
    ));

    let mut state = State {
        ecs: world,
        map,
        player_entity,
        rng,
        messages: MessageLog {
            log: Vec::new(),
            current_messages: Vec::new(),
        },
        has_moved: false,
        turn_order: VecDeque::new(),
        operating_mode: OperatingMode::MainMenu(ui::MainMenuState {
            selection: 0,
            xpfile: XpFile::from_resource("../assets/main-menu.xp").unwrap(),
            looking_at_help: false,
        }),
        debug: true,
    };

    state.turn_order.push_back(player_entity);

    populate_map(&mut state);
    item_fill_map(&mut state);

    let context = BTermBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)?
        .with_title("Elemental Caverns")
        .build()?;
    main_loop(context, state)
}
