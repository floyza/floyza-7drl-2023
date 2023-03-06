use bracket_lib::terminal::Point;

pub enum Command {
    Move { target: Point },
    Grab,
    OpenInventory,
    Wait,
}
