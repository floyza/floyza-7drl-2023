pub mod inventory;

pub enum Window {
    None,
    Inventory { window: inventory::InventoryWindow },
}
