use std::collections::BTreeMap;

use crate::{
    components::*, monster::spawn_monster_idx, ui, OperatingMode, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use bracket_lib::prelude::*;
use hecs::{Entity, Satisfies};

use crate::State;

pub const MAP_UI_DIM: Rect = Rect {
    x1: ui::LEFT_SIDEBAR_WIDTH,
    x2: WINDOW_WIDTH - ui::RIGHT_SIDEBAR_WIDTH,
    y1: 0,
    y2: WINDOW_HEIGHT - ui::MESSAGE_LOG_HEIGHT,
};

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Wall,
    Floor,
    Stairs,
}

pub struct Map {
    pub depth: i32,
    pub tiles: Vec<Tile>,
    pub width: i32,
    pub height: i32,
    pub rooms: Vec<Rect>,
    pub visible_tiles: Vec<bool>,
    pub revealed_tiles: Vec<bool>,
    pub blocked_tiles: Vec<bool>,
    pub tile_contents: Vec<Vec<Entity>>,
}

impl Algorithm2D for Map {
    fn point2d_to_index(&self, pt: Point) -> usize {
        (pt.y as usize * self.width as usize) + pt.x as usize
    }
    fn index_to_point2d(&self, idx: usize) -> Point {
        Point::new(idx as i32 % self.width, idx as i32 / self.width)
    }
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, i: usize) -> bool {
        self.tiles[i] == Tile::Wall
    }
    fn get_available_exits(&self, i: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut vec = SmallVec::<[(usize, f32); 10]>::new();
        let Point { x, y } = self.index_to_point2d(i);
        for (new_x, new_y) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
            if self.is_available_exit(self.point2d_to_index(Point::new(new_x, new_y))) {
                let cost = 1.0;
                vec.push((self.point2d_to_index(Point::new(new_x, new_y)), cost));
            }
        }
        for (new_x, new_y) in [
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x + 1, y + 1),
        ] {
            if self.is_available_exit(self.point2d_to_index(Point::new(new_x, new_y))) {
                let cost = 1.4;
                vec.push((self.point2d_to_index(Point::new(new_x, new_y)), cost));
            }
        }
        vec
    }
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        // using DistanceAlg::Pythagoras instead of DistanceAlg::Diagonal so that monsters act more like you think
        // for example, if a monster is two tiles away, it might move towards you diagonally.
        // however, this may be the wrong solution: we only need a _tiny_ bit of diagonal penalty for that to work, not .4 or whatever
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Map {
    pub fn new(dl: i32, rng: &mut RandomNumberGenerator) -> Map {
        const WIDTH: i32 = 70;
        const HEIGHT: i32 = 60;
        const SIZE: usize = (WIDTH * HEIGHT) as usize;
        let mut map = Map {
            depth: dl,
            tiles: vec![Tile::Wall; SIZE],
            rooms: Vec::new(),
            width: WIDTH,
            height: HEIGHT,
            visible_tiles: vec![false; SIZE],
            revealed_tiles: vec![false; SIZE],
            blocked_tiles: vec![false; SIZE],
            tile_contents: vec![Vec::new(); SIZE],
        };

        let mut rooms = Vec::<Rect>::new();

        for _i in 0..15 {
            let new_room = loop {
                let room_center = (rng.range(10, WIDTH - 10), rng.range(10, HEIGHT - 10));
                let width = rng.range(5, 9);
                let height = rng.range(5, 9);
                let room = Rect::with_size(
                    room_center.0 - width / 2,
                    room_center.1 - height / 2,
                    width,
                    height,
                );

                if !rooms.iter().any(|r| room.intersect(r)) {
                    break room;
                }
            };
            rooms.push(new_room);
        }

        #[derive(Clone, Copy)]
        enum Dir {
            Horizontal,
            Vertical,
        }
        let mut halls = Vec::<(Point, Dir, i32)>::new();

        // connect each room to the next
        for i in 0..rooms.len() - 1 {
            let start = rooms[i].center();
            let end = rooms[i + 1].center();
            let diff = end - start;
            let hall_horizontal = (start, Dir::Horizontal, diff.x);
            let hall_vertical = (start + Point::new(diff.x, 0), Dir::Vertical, diff.y);
            halls.push(hall_horizontal);
            halls.push(hall_vertical);
        }

        // carve out rooms
        for room in rooms.iter() {
            room.for_each(|p| {
                let i = map.point2d_to_index(p);
                map.tiles[i] = Tile::Floor;
            });
        }

        map.rooms = rooms;

        // carve out halls
        for (base, dir, len) in halls {
            let range = if len < 0 { len..=0 } else { 0..=len };
            for i in range {
                match dir {
                    Dir::Horizontal => {
                        let i = map.point2d_to_index(base + Point::new(i, 0));
                        map.tiles[i] = Tile::Floor;
                    }
                    Dir::Vertical => {
                        let i = map.point2d_to_index(base + Point::new(0, i));
                        map.tiles[i] = Tile::Floor;
                    }
                }
            }
        }

        let idx = map.point2d_to_index(random_room_point(&map, rng));
        map.tiles[idx] = Tile::Stairs;
        map
    }

    pub fn make_last_room(_rng: &mut RandomNumberGenerator) -> Map {
        const WIDTH: i32 = 50;
        const HEIGHT: i32 = 20;
        const SIZE: usize = (WIDTH * HEIGHT) as usize;
        let mut map = Map {
            depth: 5,
            tiles: vec![Tile::Wall; SIZE],
            rooms: Vec::new(),
            width: WIDTH,
            height: HEIGHT,
            visible_tiles: vec![false; SIZE],
            revealed_tiles: vec![false; SIZE],
            blocked_tiles: vec![false; SIZE],
            tile_contents: vec![Vec::new(); SIZE],
        };

        let mut rooms = vec![];
        let starting_room = Rect::with_size(7, 7, 6, 6);
        rooms.push(starting_room);

        let grand_room = Rect::with_size(20, 5, 15, 10);
        rooms.push(grand_room);

        for x in 10..30 {
            let idx1 = map.point2d_to_index(Point::new(x, HEIGHT / 2));
            let idx2 = map.point2d_to_index(Point::new(x, HEIGHT / 2 - 1));
            map.tiles[idx1] = Tile::Floor;
            map.tiles[idx2] = Tile::Floor;
        }

        // carve out rooms
        for room in rooms.iter() {
            room.for_each(|p| {
                let i = map.point2d_to_index(p);
                map.tiles[i] = Tile::Floor;
            });
        }

        let idx = map.point2d_to_index(Point::new(35, 5));
        map.tiles[idx] = Tile::Floor;
        let idx = map.point2d_to_index(Point::new(36, 5));
        map.tiles[idx] = Tile::Stairs;

        map.rooms = rooms;

        map
    }

    pub fn is_available_exit(&self, i: usize) -> bool {
        self.in_bounds(self.index_to_point2d(i)) && !self.blocked_tiles[i]
    }
}

pub fn random_room_point(map: &Map, rng: &mut RandomNumberGenerator) -> Point {
    let room_idx = rng.range(0, map.rooms.len());
    let room = map.rooms[room_idx];
    debug_assert!(room.x1 <= room.x2);
    debug_assert!(room.y1 <= room.y2);
    let x = rng.range(room.x1, room.x2);
    let y = rng.range(room.y1, room.y2);
    Point::new(x, y)
}

pub fn populate_map_last_level(state: &mut State) {
    let fire = spawn_monster_idx(
        state,
        666,
        state.map.rooms[1].center() + Point::new(0, -4),
        0,
    );
    let air = spawn_monster_idx(state, 666, state.map.rooms[1].center(), 1);
    let water = spawn_monster_idx(
        state,
        666,
        state.map.rooms[1].center() + Point::new(0, 4),
        2,
    );
    state.turn_order.push_back(fire);
    state.turn_order.push_back(air);
    state.turn_order.push_back(water);
}

pub fn populate_map(state: &mut State) {
    if state.map.depth == 5 {
        populate_map_last_level(state);
        return;
    }
    let mut new_monsters: Vec<Point> = Vec::new();
    let mut count = 0;
    loop {
        let pt = random_room_point(&state.map, &mut state.rng);
        if state.map.depth == 0 && state.map.rooms[0].point_in_rect(pt) {
            // avoid spawning monsters in starting room
            continue;
        }
        if !new_monsters.contains(&pt) {
            new_monsters.push(pt);
            count += 1;
            if count >= 15 {
                break;
            }
        }
    }
    for pt in new_monsters {
        let edl;
        if state.map.depth > 4 {
            edl = 2;
        } else if state.map.depth % 2 == 1 {
            edl = state.map.depth / 2 + state.rng.range(0, 2);
        } else {
            edl = state.map.depth / 2;
        }
        let entity = crate::monster::spawn_monster(state, edl, pt);
        state.turn_order.push_back(entity);
    }
}

pub fn item_fill_map(state: &mut State) {
    if state.map.depth == 5 {
        return;
    }
    let mut new_items: Vec<Point> = Vec::new();
    let mut count = 0;
    if state.map.depth == 0 {
        // guarantee a item in the starting room
        let room = state.map.rooms[0];
        debug_assert!(room.x1 <= room.x2);
        debug_assert!(room.y1 <= room.y2);
        let item_x = state.rng.range(room.x1, room.x2);
        let item_y = state.rng.range(room.y1, room.y2);
        let pt = Point::new(item_x, item_y);
        new_items.push(pt);
    }
    loop {
        let pt = random_room_point(&state.map, &mut state.rng);
        if !new_items.contains(&pt) {
            new_items.push(pt);
            count += 1;
            if count >= 3 {
                break;
            }
        }
    }
    for pt in new_items {
        let edl;
        if state.map.depth > 4 {
            edl = 2;
        } else if state.map.depth % 2 == 1 {
            edl = state.map.depth / 2 + state.rng.range(0, 2);
        } else {
            edl = state.map.depth / 2;
        }
        crate::item::spawn_item(state, edl, pt);
    }
}

pub fn draw_map(state: &State, ctx: &mut BTerm) {
    let mut query = state
        .ecs
        .query_one::<&Position>(state.player_entity)
        .unwrap();
    let player_pos = query.get().unwrap().0;
    let offset = player_pos - MAP_UI_DIM.center();
    MAP_UI_DIM.for_each(|pt| {
        if !state.map.in_bounds(pt + offset) {
            return;
        }
        let idx = state.map.point2d_to_index(pt + offset);
        if let Some(tile) = state.map.tiles.get(idx) {
            if state.map.revealed_tiles[idx] {
                let glyph;
                let mut fg;
                match tile {
                    Tile::Floor => {
                        glyph = to_cp437('.');
                        fg = RGB::from_hex("#f5f5dc").unwrap();
                    }
                    Tile::Wall => {
                        glyph = to_cp437('#');
                        if state.map.depth == 5 {
                            fg = RGB::from_hex("#8a2be2").unwrap();
                        } else {
                            fg = RGB::from_hex("#838b83").unwrap();
                        }
                    }
                    Tile::Stairs => {
                        glyph = to_cp437('>');
                        fg = RGB::from_hex("#da2c43").unwrap();
                    }
                }
                if !state.map.visible_tiles[idx] {
                    fg = fg.to_greyscale();
                }
                ctx.set(pt.x, pt.y, fg, RGB::from_f32(0., 0., 0.), glyph);
            }
        }
    });

    draw_entities(state, ctx, offset)
}

fn draw_entities(state: &State, ctx: &mut BTerm, offset: Point) {
    let mut renderings: BTreeMap<i32, Vec<(Renderable, Point)>> = BTreeMap::new();
    for (_id, (pos, render, slowed)) in state
        .ecs
        .query::<(&Position, &Renderable, Satisfies<&Slowed>)>()
        .iter()
    {
        if state.map.visible_tiles[state.map.point2d_to_index(pos.0)] {
            if !renderings.contains_key(&render.layer) {
                renderings.insert(render.layer, Vec::new());
            }
            let mut drawing = render.clone();
            if slowed {
                drawing.bg = RGB::named(DARKBLUE);
            }
            renderings
                .get_mut(&render.layer)
                .unwrap()
                .push((drawing, pos.0.clone()));
        }
    }
    for (_layer_id, layer) in renderings {
        for (render, pos) in layer {
            let pos = pos - offset;
            if MAP_UI_DIM.point_in_rect(pos) {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

pub fn new_floor(state: &mut State) {
    let new_map;
    if state.map.depth == 5 {
        // we beat the game!!
        state.operating_mode = OperatingMode::GameWon;
        return;
    } else if state.map.depth == 4 {
        new_map = Map::make_last_room(&mut state.rng);
    } else {
        new_map = Map::new(state.map.depth + 1, &mut state.rng);
    }
    state.map = new_map;
    let (position, viewer, health) = state
        .ecs
        .query_one_mut::<(&mut Position, &mut Viewer, &mut Health)>(state.player_entity)
        .unwrap();
    position.0 = state.map.rooms[0].center();
    viewer.dirty = true;
    health.hp = health.max_hp;
    let mut gone = vec![];
    for (ent, _i) in state.ecs.query_mut::<&Ephermal>() {
        gone.push(ent);
    }
    for ent in gone {
        state.ecs.despawn(ent).unwrap();
    }
    state.turn_order.clear();
    state.turn_order.push_back(state.player_entity);
    populate_map(state);
    item_fill_map(state);
}
