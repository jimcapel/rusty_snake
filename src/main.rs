//  ggez
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam};
use ggez::mint::*;
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
struct MainState {
    player: Player,
    food: Food,
    score: Score,
}

impl MainState {
    //  create new game state
    fn new(ctx: &mut Context) -> MainState {
        let player = Player::new(ctx);
        let food = Food::new(ctx);
        let score = Score::new();

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
            //  move player, false true if a collision has occurred
            if !self.player.move_player(ctx) {
                return Ok(self.new_game(ctx));
            }

            //  check for head -> food collision, if so, eat food && grow tail && spawn new food && increase score
            if self.player.head.position == self.food.position {
                // if maximum size, you win !
                if self.player.tail.len() as f32 + 1.0 == GRID_SIZE_X * GRID_SIZE_Y {
                    return Ok(self.new_game(ctx));
                }

                let mut snake_position = Vec::new();

                snake_position.push(self.player.head.position);

                for tail_segment in &self.player.tail {
                    snake_position.push(tail_segment.position);
                }

                self.food.position = Food::food_eaten(snake_position);
                self.player.add_to_tail();
                self.score.add_to_score();
            }
        }

        Ok(())
    }

    // draw code here
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        //  draw head
        graphics::draw(
            ctx,
            &self.player.mesh,
            DrawParam::default().dest(self.player.head.position),
        )?;

        //  draw tail
        for tail_segment in &self.player.tail {
            graphics::draw(
                ctx,
                &self.player.mesh,
                DrawParam::default().dest(tail_segment.position),
            )?;
        }

        //  draw food
        graphics::draw(
            ctx,
            &self.food.mesh,
            DrawParam::default().dest(self.food.position),
        )?;

        //  draw score text
        graphics::draw(
            ctx,
            &self.score.text,
            DrawParam::default().dest(Point2 {
                x: FONT_POSITION_X,
                y: FONT_POSITION_Y,
            }),
        )?;

        graphics::present(ctx)
    }
}
