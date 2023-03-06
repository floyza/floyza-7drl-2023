pub mod inventory;
pub mod message_log;

pub enum Window {
    None,
    Inventory {
        window: inventory::InventoryWindow,
    },
    MessageLog {
        window: message_log::MessageLogWindow,
    },
}
