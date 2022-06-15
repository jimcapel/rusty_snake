//  ggez
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{conf, timer, Context, ContextBuilder, GameResult};

//  declare modules
pub mod food;
pub mod player;
pub mod score;

//  use modules
use crate::food::Food;
use crate::player::Player;
use crate::score::Score;

//  constants
const GRID_SIZE_X: f32 = 32.0;
const GRID_SIZE_Y: f32 = 20.0;
const FONT_SCALE_X: f32 = 64.0;
const FONT_SCALE_Y: f32 = 64.0;
const FONT_POSITION_X: f32 = 5.0;
const FONT_POSITION_Y: f32 = 5.0;
const GRID_CELL_SIZE_X: f32 = 32.0;
const GRID_CELL_SIZE_Y: f32 = 32.0;
const FPS: u32 = 8;
const SCREEN_SIZE_X: f32 = GRID_SIZE_X * GRID_CELL_SIZE_X;
const SCREEN_SIZE_Y: f32 = GRID_SIZE_Y * GRID_CELL_SIZE_Y;

fn main() {
    let context_builder = ContextBuilder::new("rusty_snake", "jimcapello")
        .window_setup(conf::WindowSetup::default().title("rusty_snake"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE_X, SCREEN_SIZE_Y));

    let (mut ctx, event_loop) = context_builder.build().expect("problem building context!");

    let my_game = MainState::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

//   struct holding game state
pub struct MainState {
    food: Food,
    score: Score,
    player: Player,
}

impl MainState {
    //  create new game state
    fn new(ctx: &mut Context) -> MainState {
        // let player = Player::new(ctx);
        let food = Food::new(ctx);
        let score = Score::new();

        let player = Player::new(ctx);

        MainState {
            player,
            food,
            score,
        }
    }

    fn new_game(&mut self, ctx: &mut Context) {
        self.player = Player::new(ctx);
        self.food = Food::new(ctx);
        self.score = Score::new();
    }
}

impl EventHandler for MainState {
    //  update code here
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, FPS) {
            self.player.move_player(ctx);

            if collision_check(&self.player) {
                self.new_game(ctx);
            }

            //  check for head -> food collision, if so, eat food && grow tail && spawn new food && increase score
            if self.player.body[0] == self.food.position {
                // if maximum size, you win !
                if self.player.body.len() as f32 == GRID_SIZE_X * GRID_SIZE_Y {
                    return Ok(self.new_game(ctx));
                }

                self.food.new_position(&self.player);
                self.score.change_score(10);
                self.player.grow();
            }
        }

        Ok(())
    }

    // draw code here
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        //  render snake
        self.player.render(ctx)?;

        //  render food
        self.food.render(ctx)?;

        //  draw score text
        self.score.render(ctx)?;

        graphics::present(ctx)
    }
}

fn collision_check(player: &Player) -> bool {
    if player.body.len() == 1 {
        return false;
    };

    if player.body[1..].contains(&player.body[0]) {
        return true;
    }

    false
}
