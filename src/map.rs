use crate::components::*;
use bracket_lib::prelude::*;

use crate::State;

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: i32,
    pub height: i32,
    pub rooms: Vec<Rect>,
    pub visible_tiles: Vec<bool>,
    pub revealed_tiles: Vec<bool>,
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
        for (new_x, new_y) in [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ] {
            if self.is_available_exit(self.point2d_to_index(Point::new(new_x, new_y))) {
                let cost = 1.0;
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
    pub fn new() -> Map {
        let mut map = Map {
            tiles: vec![Tile::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            visible_tiles: vec![false; 80 * 50],
            revealed_tiles: vec![false; 80 * 50],
        };

        let mut rooms = Vec::<Rect>::new();
        let mut rng = RandomNumberGenerator::new();

        for _i in 0..11 {
            let new_room = loop {
                let room_center = (rng.range(10, 70), rng.range(10, 40));
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

        map
    }

    pub fn is_available_exit(&self, i: usize) -> bool {
        self.in_bounds(self.index_to_point2d(i)) && !(self.tiles[i] == Tile::Wall)
    }
}

pub fn draw_map(state: &State, ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;
    for (idx, tile) in state.map.tiles.iter().enumerate() {
        if state.map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                Tile::Floor => {
                    glyph = to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.5, 0.5);
                }
                Tile::Wall => {
                    glyph = to_cp437('#');
                    fg = RGB::from_f32(0., 1., 0.);
                }
            }
            if !state.map.visible_tiles[idx] {
                fg = fg.to_greyscale();
            }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }

    draw_entities(state, ctx)
}

fn draw_entities(state: &State, ctx: &mut BTerm) {
    for (_id, (pos, render)) in state.ecs.query::<(&Position, &Renderable)>().iter() {
        if state.map.visible_tiles[state.map.point2d_to_index(pos.0)] {
            ctx.set(pos.0.x, pos.0.y, render.fg, render.bg, render.glyph);
        }
    }
}
