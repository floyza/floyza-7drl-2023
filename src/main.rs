use bracket_lib::prelude::*;
use components::*;
use hecs::{Entity, Satisfies, World};

pub mod components;
pub mod map;

pub struct State {
    pub ecs: World,
    pub map: map::Map,
    pub player_entity: Entity,
    pub rng: RandomNumberGenerator,
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
        system_calc_viewpoints(self);
        map::draw_map(self, ctx);
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
    let move_pt = match key {
        VirtualKeyCode::H | VirtualKeyCode::Left => Some(Point::new(-1, 0)),
        VirtualKeyCode::L | VirtualKeyCode::Right => Some(Point::new(1, 0)),
        VirtualKeyCode::K | VirtualKeyCode::Up => Some(Point::new(0, -1)),
        VirtualKeyCode::J | VirtualKeyCode::Down => Some(Point::new(0, 1)),
        VirtualKeyCode::Y => Some(Point::new(-1, -1)),
        VirtualKeyCode::U => Some(Point::new(1, -1)),
        VirtualKeyCode::B => Some(Point::new(-1, 1)),
        VirtualKeyCode::N => Some(Point::new(1, 1)),
        _ => None,
    };
    if let Some(move_pt) = move_pt {
        let (position, viewer) = state
            .ecs
            .query_one_mut::<(&mut Position, &mut Viewer)>(state.player_entity)
            .expect("Player doesn't have expected components");
        let new_pt = position.0 + move_pt;
        let new_idx = state.map.point2d_to_index(new_pt);
        if state.map.is_available_exit(new_idx) {
            position.0 = new_pt;
            viewer.dirty = true;
        }
        return true;
    }
    return false;
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
    ));

    let state = State {
        ecs: world,
        map,
        player_entity,
        rng: RandomNumberGenerator::new(),
    };

    let context = BTermBuilder::simple80x50()
        .with_title("Be what you sow")
        .build()?;
    main_loop(context, state)
}
