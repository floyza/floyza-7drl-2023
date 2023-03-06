use crate::{components::Health, State};
use bracket_lib::prelude::*;

pub fn draw_messages(state: &State, ctx: &mut BTerm) {
    for (i, message) in state
        .messages
        .current_messages
        .iter()
        .rev()
        .take(4)
        .enumerate()
    {
        ctx.print(0, state.size.y - 1 - i as i32, message);
    }
}

pub fn draw_side_info(state: &State, ctx: &mut BTerm) {
    let mut query = state.ecs.query_one::<&Health>(state.player_entity).unwrap();
    let health = query.get().unwrap();

    let base_x = 1;
    let base_y = 1;

    ctx.print(
        base_x,
        base_y,
        format!("Health: {}/{}", health.hp, health.max_hp),
    );

    let bar_width = 15;

    ctx.draw_bar_horizontal(
        base_x,
        base_y + 1,
        bar_width,
        health.hp,
        health.max_hp,
        RGB::named(RED),
        RGB::named(GRAY),
    );
}
