//  ggez
use ggez::graphics::{self, Color, DrawMode};
use ggez::mint::*;
use ggez::{Context, GameResult};

//  rand
use rand::prelude::*;

pub struct Food {
    pub position: Point2<f32>,
    pub mesh: graphics::Mesh,
}

impl Food {
    pub fn new(ctx: &mut Context) -> Self {
        let rect = graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: crate::GRID_CELL_SIZE_X,
            h: crate::GRID_CELL_SIZE_Y,
        };

        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::RED)
            .expect("error creating food mesh");

        let (mut random_x, mut random_y) = Self::generate_random_position();
        let middle_of_board = (
            crate::GRID_CELL_SIZE_X * (crate::GRID_SIZE_X / 2.0),
            crate::GRID_CELL_SIZE_Y * (crate::GRID_SIZE_Y / 2.0),
        );

        while (random_x, random_y) == middle_of_board {
            (random_x, random_y) = Self::generate_random_position();
        }

        Food {
            position: Point2 {
                x: random_x,
                y: random_y,
            },
            mesh,
        }
    }

    pub fn new_position(&mut self, snake: &crate::Player) {
        let (mut random_x, mut random_y) = Self::generate_random_position();

        let mut random_point = Point2 {
            x: random_x,
            y: random_y,
        };

        while snake.body.contains(&random_point) {
            (random_x, random_y) = Self::generate_random_position();

            random_point = Point2 {
                x: random_x,
                y: random_y,
            };
        }

        self.position = random_point;
    }

    fn generate_random_position() -> (f32, f32) {
        let mut rng = rand::thread_rng();
        let random_x: f32 =
            rng.gen_range(0.0..crate::GRID_SIZE_X).floor() * crate::GRID_CELL_SIZE_X;
        let random_y: f32 =
            rng.gen_range(0.0..crate::GRID_SIZE_Y).floor() * crate::GRID_CELL_SIZE_Y;
        (random_x, random_y)
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(
            ctx,
            &self.mesh,
            graphics::DrawParam::default().dest(self.position),
        )?;

        Ok(())
    }
}
