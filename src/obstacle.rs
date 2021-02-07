use bracket_lib::prelude::*;
use super::{Player, SCREEN_HEIGHT};

pub struct Obstacle {
    pub x: i32,      // x pos in world space
    pub gap_y: i32,  // y pos of the gap in the wall
    pub size: i32,   // size of the gap
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score)
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x : i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x - 1,
                y,
                LIGHT_GREEN,
                DARK_CYAN,
                to_cp437('#'),
            );
            ctx.set(
                screen_x,
                y,
                LIGHT_GREEN,
                DARK_CYAN,
                to_cp437('#'),
            );
            ctx.set(
                screen_x + 1,
                y,
                LIGHT_GREEN,
                DARK_CYAN,
                to_cp437('#'),
            );
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x - 1,
                y,
                LIGHT_GREEN,
                DARK_CYAN,
                to_cp437('#'),
            );
            ctx.set(
                screen_x,
                y,
                LIGHT_GREEN,
                DARK_CYAN,
                to_cp437('#'),
            );
            ctx.set(
                screen_x + 1,
                y,
                LIGHT_GREEN,
                DARK_CYAN,
                to_cp437('#'),
            );
        }
    }

    pub fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x + player.screen_x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        does_x_match && (player_above_gap || player_below_gap)
    }
}