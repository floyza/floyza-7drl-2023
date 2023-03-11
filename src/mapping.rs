use bracket_lib::terminal::{console, BTerm, Point, VirtualKeyCode};

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Move { target: Point },
    Grab,
    OpenInventory,
    OpenMessageLog,
    Wait,
    Back,
    OpenExamine,
    DescendStairs,
    Select,
}

pub fn get_command(ctx: &mut BTerm) -> Option<Command> {
    if let Some(key) = ctx.key.take() {
        if ctx.shift {
            match key {
                VirtualKeyCode::Period => Some(Command::DescendStairs),
                _ => None,
            }
        } else {
            match key {
                VirtualKeyCode::H | VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => {
                    Some(Command::Move {
                        target: Point::new(-1, 0),
                    })
                }
                VirtualKeyCode::L | VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => {
                    Some(Command::Move {
                        target: Point::new(1, 0),
                    })
                }
                VirtualKeyCode::K | VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => {
                    Some(Command::Move {
                        target: Point::new(0, -1),
                    })
                }
                VirtualKeyCode::J | VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => {
                    Some(Command::Move {
                        target: Point::new(0, 1),
                    })
                }
                VirtualKeyCode::Y | VirtualKeyCode::Numpad7 => Some(Command::Move {
                    target: Point::new(-1, -1),
                }),
                VirtualKeyCode::U | VirtualKeyCode::Numpad9 => Some(Command::Move {
                    target: Point::new(1, -1),
                }),
                VirtualKeyCode::B | VirtualKeyCode::Numpad1 => Some(Command::Move {
                    target: Point::new(-1, 1),
                }),
                VirtualKeyCode::N | VirtualKeyCode::Numpad3 => Some(Command::Move {
                    target: Point::new(1, 1),
                }),
                VirtualKeyCode::G => Some(Command::Grab),
                VirtualKeyCode::I => Some(Command::OpenInventory),
                VirtualKeyCode::M => Some(Command::OpenMessageLog),
                VirtualKeyCode::X => Some(Command::OpenExamine),
                VirtualKeyCode::Period => Some(Command::Wait),
                VirtualKeyCode::Escape | VirtualKeyCode::Q => Some(Command::Back),
                VirtualKeyCode::Return => Some(Command::Select),
                _ => None,
            }
        }
    } else {
        None
    }
}
