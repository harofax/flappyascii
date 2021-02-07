use bracket_lib::prelude::*;
use crate::SCREEN_HEIGHT;

pub struct Cloud {
    pub x: i32,      // x pos in world space
    pub y: i32,
    thick_factor: i32,
    pub cloud_length: i32,
    rng: RandomNumberGenerator
}

impl Cloud {
    pub fn new(x: i32) -> Self {
        let mut rngesus = RandomNumberGenerator::new();
        Cloud {
            x,
            y: rngesus.range(0, SCREEN_HEIGHT/2),
            thick_factor: rngesus.range(2, 9),
            cloud_length: rngesus.range(15, 30),
            rng: rngesus,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x : i32) {
        for mut y_thick in 0..self.thick_factor {
            for screen_x in self.x - player_x..self.x - player_x + self.cloud_length {
                ctx.set(
                    screen_x,
                    self.y + y_thick,
                    WHITE,
                    SLATE_GRAY,
                    to_cp437('≈')
                );
            }
        }


    }
}