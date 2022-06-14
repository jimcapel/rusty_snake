//  ggez
use ggez::graphics::{self, Color, DrawMode};
use ggez::mint::*;
use ggez::Context;

//  rand
use rand::prelude::*;

pub struct Food {
    pub position: Point2<f32>,
    pub mesh: graphics::Mesh,
}

impl Food {
    //  to_do: stop food from spawning on snake, when no tiles free, reset game
    pub fn new(ctx: &mut Context) -> Self {
        let rect = graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: crate::GRID_CELL_SIZE_X,
            h: crate::GRID_CELL_SIZE_Y,
        };
        // could reduce calls to this
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

    pub fn food_eaten(current_player_positions: Vec<Point2<f32>>) -> Point2<f32> {
        let (mut random_x, mut random_y) = Self::generate_random_position();

        let mut random_point = Point2 {
            x: random_x,
            y: random_y,
        };

        while current_player_positions.contains(&random_point) {
            (random_x, random_y) = Self::generate_random_position();

            random_point = Point2 {
                x: random_x,
                y: random_y,
            };
        }

        random_point
    }

    fn generate_random_position() -> (f32, f32) {
        let mut rng = rand::thread_rng();
        let random_x: f32 =
            rng.gen_range(0.0..crate::GRID_SIZE_X).floor() * crate::GRID_CELL_SIZE_X;
        let random_y: f32 =
            rng.gen_range(0.0..crate::GRID_SIZE_Y).floor() * crate::GRID_CELL_SIZE_Y;
        (random_x, random_y)
    }
}
