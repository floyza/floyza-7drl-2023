use bracket_lib::prelude::*;

use crate::{components::*, State};

pub struct MessageLogWindow {}

impl MessageLogWindow {
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
