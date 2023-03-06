use bracket_lib::prelude::*;

use crate::{components::*, State};

pub struct MessageLogWindow {}

impl MessageLogWindow {
    pub fn render(&self, state: &State, ctx: &mut BTerm) {
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
    pub fn update(&mut self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::Escape | VirtualKeyCode::Q => {
                return true;
            }
            _ => {}
        }
        return false;
    }
}
