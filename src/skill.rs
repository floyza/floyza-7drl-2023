use crate::components::*;
use crate::State;
use bracket_lib::prelude::*;
use hecs::Entity;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Skill {
    ShootBow,
}

impl Skill {
    /// Weight for action -- normal action has weight of 1.0
    pub fn get_prob(self) -> f32 {
        match self {
            Skill::ShootBow => 0.5,
        }
    }
    /// Returns true if succeeds. Otherwise, try a different skill
    pub fn apply(self, source: Entity, target: Entity, state: &mut State) -> Option<()> {
        debug_assert!(target != source);
        match self {
            Skill::ShootBow => {
                let mut mon_query = state
                    .ecs
                    .query_one::<(&Name, &Position, &Attack)>(source)
                    .unwrap();
                let (mon_name, mon_pos, mon_attack) = mon_query.get()?;
                let mut player_query = state
                    .ecs
                    .query_one::<(&Position, &mut Health)>(target)
                    .unwrap();
                let (player_pos, player_health) = player_query.get()?;

                let line = Bresenham::new(mon_pos.0, player_pos.0);
                for step in line.skip(1) {
                    let idx = state.map.point2d_to_index(step);
                    if !state.map.is_available_exit(idx) {
                        return None;
                    }
                }
                let damage = (mon_attack.damage as f32 * 0.5).round() as i32;
                player_health.hp -= damage;
                state.messages.enqueue_message(&format!(
                    "The {} shoots you for {} damage.",
                    mon_name.0, damage,
                ));
                return Some(());
            }
        }
    }
}
