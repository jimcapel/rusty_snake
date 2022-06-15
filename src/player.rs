//  ggez
use ggez::graphics::{self, Color, DrawMode};
use ggez::input::keyboard::{self, KeyCode};
use ggez::mint::*;
use ggez::{Context, GameResult};

pub struct Player {
    pub body: Vec<Point2<f32>>,
    direction: Point2<f32>,
    mesh: graphics::Mesh,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Self {
        let position = Point2 {
            x: crate::GRID_CELL_SIZE_X * (crate::GRID_SIZE_X / 2.0),
            y: crate::GRID_CELL_SIZE_Y * (crate::GRID_SIZE_Y / 2.0),
        };

        let body = vec![position];
        let direction = Point2 { x: 0.0, y: -1.0 };

        let rect = graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: crate::GRID_CELL_SIZE_X,
            h: crate::GRID_CELL_SIZE_Y,
        };

        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)
            .expect("error creating mesh");

        Player {
            body,
            direction,
            mesh,
        }
    }

    pub fn grow(&mut self) {
        let position = Point2 {
            x: crate::SCREEN_SIZE_X + crate::GRID_CELL_SIZE_X,
            y: crate::SCREEN_SIZE_Y + crate::GRID_CELL_SIZE_Y,
        };

        self.body.push(position);
    }

    pub fn move_player(&mut self, ctx: &mut Context) {
        let tail_length_equals_zero = self.body.len() == 1;

        //  head
        //  check key presses to determine movement
        if keyboard::is_key_pressed(ctx, KeyCode::W)
            && (self.direction.y != 1.0 || tail_length_equals_zero)
        {
            self.direction = Point2 { x: 0.0, y: -1.0 };
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A)
            && (self.direction.x != 1.0 || tail_length_equals_zero)
        {
            self.direction = Point2 { x: -1.0, y: 0.0 };
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S)
            && (self.direction.y != -1.0 || tail_length_equals_zero)
        {
            self.direction = Point2 { x: 0.0, y: 1.0 };
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D)
            && (self.direction.x != -1.0 || tail_length_equals_zero)
        {
            self.direction = Point2 { x: 1.0, y: 0.0 };
        }

        //  update position of body
        self.body.insert(
            0,
            Point2 {
                x: self.body[0].x + (self.direction.x * crate::GRID_CELL_SIZE_X),
                y: self.body[0].y + (self.direction.y * crate::GRID_CELL_SIZE_Y),
            },
        );
        self.body.pop();

        //  wrap around screen if out of bounds
        if self.body[0].x == crate::SCREEN_SIZE_X {
            self.body[0].x = 0.0;
        } else if self.body[0].x < 0.0 {
            self.body[0].x = crate::SCREEN_SIZE_X - crate::GRID_CELL_SIZE_X;
        } else if self.body[0].y == crate::SCREEN_SIZE_Y {
            self.body[0].y = 0.0;
        } else if self.body[0].y < 0.0 {
            self.body[0].y = crate::SCREEN_SIZE_Y - crate::GRID_CELL_SIZE_Y;
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        for i in &self.body {
            graphics::draw(ctx, &self.mesh, graphics::DrawParam::default().dest(*i))?;
        }

        Ok(())
    }
}
