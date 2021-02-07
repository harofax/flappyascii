use bracket_lib::prelude::*;


pub struct Player {
    pub screen_x: i32,
    pub x: i32,
    pub y: i32,
    pub velocity: f32,
    tail_y: i32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            screen_x: 5,
            x,
            y,
            velocity: 0.0,
            tail_y: y,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            self.screen_x,
            self.y,
            GOLD,
            VIOLETRED,
            to_cp437('▼')
        );
        ctx.set(
            self.screen_x - 1,
            self.tail_y,
            GOLD,
            VIOLETRED,
            to_cp437('»')
        );
        ctx.set(
            self.screen_x - 2,
            self.y,
            GOLD,
            VIOLETRED,
            to_cp437('}')
        );
    }

    pub fn tail_update(&mut self) {
        self.tail_y = self.y;
    }

    pub fn gravity_and_move(&mut self) {
        if self.velocity < 3.0 {
            self.velocity += 0.6;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    pub fn flap(&mut self) {
        self.velocity = -4.8;
    }
}