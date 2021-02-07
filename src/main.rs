mod player;
pub use player::*;

mod obstacle;
mod scenery;

pub use scenery::*;

pub use obstacle::*;

use bracket_lib::prelude::*;

// Game states
enum GameMode {
    MainMenu,
    Playing,
    End,
}


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 30.0; // duration of a frame in millisecs
const TAIL_UPDATE: f32 = FRAME_DURATION * 2.3;



// A state stores the games current status, everything we
// need to preserve between frames is stored here.
// A snapshot of the current game status
struct State {
    player: Player,
    frame_time: f32,
    tail_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
    cloud: Cloud,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            tail_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::MainMenu,
            score: 0,
            cloud: Cloud::new(SCREEN_WIDTH),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(CORNFLOWER_BLUE);

        self.frame_time += ctx.frame_time_ms;
        self.tail_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if self.tail_time > TAIL_UPDATE {
            self.tail_time = 0.0;
            self.player.tail_update();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.cloud.render(ctx, self.player.x);
        self.player.render(ctx);

        ctx.draw_hollow_box_double(0, 0, 24, 6, GOLD, CORNFLOWER_BLUE);
        ctx.fill_region(Rect::with_size(1,1,22,4), 0, GOLD, PALEVIOLETRED);
        ctx.print(3, 3, "Press SPACE to flap.");

        ctx.fill_region(Rect::with_size(SCREEN_WIDTH/2 - 5,2,9,2), 0, PALE_GREEN, PALEVIOLETRED);
        ctx.draw_hollow_box_double(SCREEN_WIDTH/2 - 6, 1, 11, 4, PALE_GREEN, CORNFLOWER_BLUE);
        ctx.print_centered(3, &format!("Score: {}", self.score));


        self.obstacle.render(ctx, self.player.x);



        if self.cloud.x+self.cloud.cloud_length < self.player.x {
            self.cloud = Cloud::new(self.player.x + SCREEN_WIDTH);
        }

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT ||self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(SKY_BLUE);
        ctx.draw_box_double(6, 3, 60, 20, GOLD, SKYBLUE);
        ctx.fill_region(Rect::with_size(7, 4, 58, 18), 0, GOLD, DODGER_BLUE);
        ctx.print(11, 7, "Welcome to Flappy Ascii");

        ctx.draw_hollow_box_double(10, 10, 20, 4, GOLD, DODGER_BLUE);
        ctx.fill_region(Rect::with_size(11, 11, 18, 2), 0, PALE_GREEN, PALEVIOLETRED2);
        ctx.print(12, 12, "(P) -- Play game");

        ctx.draw_hollow_box_double(10, 16, 20, 4, GOLD, DODGER_BLUE);
        ctx.fill_region(Rect::with_size(11, 17, 18, 2), 0, PALE_GREEN, PALEVIOLETRED2);
        ctx.print(12, 18, "(Q) -- Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You died :/");
        ctx.print_centered(8, &format!("You got {} points", self.score));
        ctx.print_centered(12, "(P) -- Play again");
        ctx.print_centered(14, "(Q) -- Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    // Allows access to the State, and allows changing stuff in it
    // The bridge between the game engine and the game
    // Called every "tick", every frame
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::MainMenu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {

    let context = BTermBuilder::new()
        .with_font("../resources/haro_16x16.png", 16, 16)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/haro_16x16.png")
        .with_tile_dimensions(16, 16)
        .with_title("Flappy Ascii")
        .build()?;

    main_loop(context, State::new())
}
