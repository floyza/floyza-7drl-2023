//! One way to the contents of an entity, as you might do for debugging. A similar pattern could
//! also be useful for serialization, or other row-oriented generic operations.

use crate::{
    components::*,
    components_serde::{self, Component},
};

pub fn format_entity(entity: hecs::EntityRef<'_>) -> String {
    let comp = get_entity_components(entity);
    serde_json::to_string(&comp).unwrap()
}

pub fn get_entity_components(entity: hecs::EntityRef<'_>) -> Vec<components_serde::Component> {
    let mut c = vec![];
    if let Some(x) = entity.get::<&Health>() {
        c.push(Component::Health((*x).clone()));
    }
    if let Some(x) = entity.get::<&Attack>() {
        c.push(Component::Attack((*x).clone()));
    }
    if let Some(x) = entity.get::<&Position>() {
        c.push(Component::Position((*x).clone()));
    }
    if let Some(x) = entity.get::<&Renderable>() {
        c.push(Component::Renderable((*x).clone()));
    }
    if let Some(x) = entity.get::<&Viewer>() {
        c.push(Component::Viewer((*x).clone()));
    }
    if let Some(x) = entity.get::<&Player>() {
        c.push(Component::Player((*x).clone()));
    }
    if let Some(x) = entity.get::<&Monster>() {
        c.push(Component::Monster((*x).clone()));
    }
    if let Some(x) = entity.get::<&Skilled>() {
        c.push(Component::Skilled((*x).clone()));
    }
    if let Some(x) = entity.get::<&Item>() {
        c.push(Component::Item((*x).clone()));
    }
    if let Some(x) = entity.get::<&Name>() {
        c.push(Component::Name((*x).clone()));
    }
    if let Some(x) = entity.get::<&Grower>() {
        c.push(Component::Grower((*x).clone()));
    }
    if let Some(x) = entity.get::<&Blocker>() {
        c.push(Component::Blocker((*x).clone()));
    }
    if let Some(x) = entity.get::<&Blueprint>() {
        c.push(Component::Blueprint((*x).clone()));
    }
    c
}
