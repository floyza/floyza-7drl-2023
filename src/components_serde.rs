use crate::components::*;
use hecs::{Entity, NoSuchEntity, World};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Component {
    Health(Health),
    Attack(Attack),
    Position(Position),
    Renderable(Renderable),
    Viewer(Viewer),
    Player(Player),
    Monster(Monster),
    Skilled(Skilled),
    Item(Item),
    Name(Name),
    Grower(Grower),
    Blocker(Blocker),
    Blueprint(Blueprint),
}

impl Component {
    pub fn insert(self, ecs: &mut World, entity: Entity) -> Result<(), NoSuchEntity> {
        match self {
            Component::Health(c) => ecs.insert_one(entity, c),
            Component::Attack(c) => ecs.insert_one(entity, c),
            Component::Position(c) => ecs.insert_one(entity, c),
            Component::Renderable(c) => ecs.insert_one(entity, c),
            Component::Viewer(c) => ecs.insert_one(entity, c),
            Component::Player(c) => ecs.insert_one(entity, c),
            Component::Monster(c) => ecs.insert_one(entity, c),
            Component::Skilled(c) => ecs.insert_one(entity, c),
            Component::Item(c) => ecs.insert_one(entity, c),
            Component::Name(c) => ecs.insert_one(entity, c),
            Component::Grower(c) => ecs.insert_one(entity, c),
            Component::Blocker(c) => ecs.insert_one(entity, c),
            Component::Blueprint(c) => ecs.insert_one(entity, c),
        }
    }
}

#[cfg(test)]
mod tests {
    use bracket_lib::terminal::{to_cp437, RGB};

    use super::*;

    #[test]
    fn serde_test() {
        let json = r###"[{"Health":{"max_hp":10}},{"Attack":{"damage":10}},{"Name":"Orc"},{"Renderable":{"glyph":"o","fg":"#ffffff","bg":"#000000","layer":1}}]"###;
        let parsed: Vec<Component> = serde_json::from_str(&json).unwrap();
        let and_back_again = serde_json::to_string(&parsed).unwrap();
        let and_again: Vec<Component> = serde_json::from_str(&and_back_again).unwrap();
        let data = vec![
            Component::Health(Health { max_hp: 10, hp: 10 }),
            Component::Attack(Attack { damage: 10 }),
            Component::Name(Name("Orc".to_string())),
            Component::Renderable(Renderable {
                glyph: to_cp437('o'),
                fg: RGB::from_hex("#ffffff").unwrap(),
                bg: RGB::from_hex("#000000").unwrap(),
                layer: 1,
            }),
        ];
        assert_eq!(parsed, data);
        assert_eq!(parsed, and_again);
    }
}
