use bracket_lib::prelude::*;
use commands::Command;
use components::*;
use hecs::{Entity, Satisfies, World};
use messages::MessageLog;

pub mod commands;
pub mod components;
pub mod map;
pub mod messages;

pub struct State {
    pub ecs: World,
    pub map: map::Map,
    pub player_entity: Entity,
    pub rng: RandomNumberGenerator,
    pub messages: MessageLog,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let player_acted = if let Some(key) = ctx.key {
            player_act(self, key)
        } else {
            false
        };
        if player_acted {
            // make monsters act
        }
        system_tile_contents(self);
        system_calc_viewpoints(self);
        map::draw_map(self, ctx);
        messages::handle_messages(self);
    }
}

fn system_tile_contents(state: &mut State) {
    for tile in state.map.tile_contents.iter_mut() {
        tile.clear();
    }
    for (id, pos) in state.ecs.query_mut::<&Position>() {
        let idx = state.map.point2d_to_index(pos.0);
        state.map.tile_contents[idx].push(id);
    }
}

fn system_calc_viewpoints(state: &mut State) {
    for (_id, (viewer, position, is_player)) in
        state
            .ecs
            .query_mut::<(&mut Viewer, &Position, Satisfies<&Player>)>()
    {
        if viewer.dirty {
            viewer.dirty = false;
            viewer.visible_tiles.clear();
            viewer.visible_tiles = field_of_view(position.0, viewer.range, &state.map);
            viewer.visible_tiles.retain(|p| {
                p.x >= 0 && p.x < state.map.width && p.y >= 0 && p.y < state.map.height
            });
        }
        if is_player {
            for t in state.map.visible_tiles.iter_mut() {
                *t = false;
            }
            for vis in viewer.visible_tiles.iter() {
                let idx = state.map.point2d_to_index(*vis);
                state.map.revealed_tiles[idx] = true;
                state.map.visible_tiles[idx] = true;
            }
        }
    }
}

fn player_act(state: &mut State, key: VirtualKeyCode) -> bool {
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
        _ => None,
    };
    match act {
        Some(Command::Move { target: move_pt }) => {
            let (position, viewer) = state
                .ecs
                .query_one_mut::<(&mut Position, &mut Viewer)>(state.player_entity)
                .unwrap();
            let new_pt = position.0 + move_pt;
            let new_idx = state.map.point2d_to_index(new_pt);
            if state.map.is_available_exit(new_idx) {
                position.0 = new_pt;
                viewer.dirty = true;
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
        None => false,
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

    let state = State {
        ecs: world,
        map,
        player_entity,
        rng: RandomNumberGenerator::new(),
        messages: MessageLog {
            log: Vec::new(),
            queue: Vec::new(),
        },
    };

    let context = BTermBuilder::simple80x50()
        .with_title("Be what you sow")
        .build()?;
    main_loop(context, state)
}
