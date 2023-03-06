use bracket_lib::prelude::*;

use crate::{components::*, State};

pub struct InventoryUI {
    pub selection: u32,
    pub length: u32,
}

impl InventoryUI {
    pub fn render(&self, state: &State, ctx: &mut BTerm) {
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
            if self.selection == idx as u32 {
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

    /// Returns `true` if we are done with this window
    pub fn update(&mut self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::K | VirtualKeyCode::Up => {
                if self.selection > 0 {
                    self.selection -= 1;
                }
            }
            VirtualKeyCode::J | VirtualKeyCode::Down => {
                if self.selection < self.length - 1 {
                    self.selection += 1;
                }
            }
            VirtualKeyCode::Escape | VirtualKeyCode::Q => {
                return true;
            }
            _ => {}
        }
        return false;
    }
}
