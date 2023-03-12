use bracket_lib::terminal::{BTerm, Point, VirtualKeyCode};

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
    CreateItem,
    UseActive(i32),
    EquipExamine,
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
                VirtualKeyCode::E => Some(Command::EquipExamine),
                VirtualKeyCode::D => Some(Command::DescendStairs),
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
                VirtualKeyCode::X | VirtualKeyCode::Slash => Some(Command::OpenExamine),
                VirtualKeyCode::A => Some(Command::CreateItem),
                VirtualKeyCode::Period => Some(Command::Wait),
                VirtualKeyCode::Escape | VirtualKeyCode::Q => Some(Command::Back),
                VirtualKeyCode::Return => Some(Command::Select),
                VirtualKeyCode::Key1 => Some(Command::UseActive(1)),
                VirtualKeyCode::Key2 => Some(Command::UseActive(2)),
                VirtualKeyCode::Key3 => Some(Command::UseActive(3)),
                VirtualKeyCode::Key4 => Some(Command::UseActive(4)),
                VirtualKeyCode::Key5 => Some(Command::UseActive(5)),
                VirtualKeyCode::Key6 => Some(Command::UseActive(6)),
                VirtualKeyCode::Key7 => Some(Command::UseActive(7)),
                VirtualKeyCode::Key8 => Some(Command::UseActive(8)),
                VirtualKeyCode::Key9 => Some(Command::UseActive(9)),
                _ => None,
            }
        }
    } else {
        None
    }
}
