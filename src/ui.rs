pub mod inventory_ui;

pub enum UI {
    Playing,
    Inventory { ui: inventory_ui::InventoryUI },
}
