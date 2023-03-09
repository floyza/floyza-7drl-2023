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
}

pub fn get_command(ctx: &mut BTerm) -> Option<Command> {
    if let Some(key) = ctx.key.take() {
        match key {
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
            VirtualKeyCode::I => Some(Command::OpenInventory),
            VirtualKeyCode::M => Some(Command::OpenMessageLog),
            VirtualKeyCode::X => Some(Command::OpenExamine),
            VirtualKeyCode::Period => Some(Command::Wait),
            VirtualKeyCode::Escape | VirtualKeyCode::Q => Some(Command::Back),
            _ => None,
        }
    } else {
        None
    }
}
