//  ggez
use ggez::graphics::{self, Color, DrawMode};
use ggez::input::keyboard::{self, KeyCode};
use ggez::mint::*;
use ggez::Context;

pub struct PlayerPosition {
    pub position: Point2<f32>,
    pub direction: Point2<f32>,
}

pub struct Player {
    pub head: PlayerPosition,
    pub tail: Vec<PlayerPosition>,
    pub mesh: graphics::Mesh,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Self {
        let head = PlayerPosition {
            position: Point2 {
                x: crate::GRID_CELL_SIZE_X * (crate::GRID_SIZE_X / 2.0),
                y: crate::GRID_CELL_SIZE_Y * (crate::GRID_SIZE_Y / 2.0),
            },
            direction: Point2 { x: 0.0, y: -1.0 },
        };

        let tail: Vec<PlayerPosition> = Vec::new();

        let rect = graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: crate::GRID_CELL_SIZE_X,
            h: crate::GRID_CELL_SIZE_Y,
        };

        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)
            .expect("error creating mesh");

        Player { head, tail, mesh }
    }

    pub fn add_to_tail(&mut self) {
        self.tail.push(PlayerPosition {
            position: Point2 {
                x: crate::SCREEN_SIZE_X + crate::GRID_CELL_SIZE_X,
                y: crate::SCREEN_SIZE_Y + crate::GRID_CELL_SIZE_Y,
            },
            direction: Point2 { x: 0.0, y: 0.0 },
        })
    }

    //  returns true if collision
    pub fn move_player(&mut self, ctx: &mut Context) -> bool {
        let mut position_cache = self.head.position;
        let tail_length_equals_zero = self.tail.len() == 0;

        //  head
        //  check key presses to determine movement
        if keyboard::is_key_pressed(ctx, KeyCode::W)
            && (self.head.direction.y != 1.0 || tail_length_equals_zero)
        {
            self.head.direction = Point2 { x: 0.0, y: -1.0 };
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A)
            && (self.head.direction.x != 1.0 || tail_length_equals_zero)
        {
            self.head.direction = Point2 { x: -1.0, y: 0.0 };
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S)
            && (self.head.direction.y != -1.0 || tail_length_equals_zero)
        {
            self.head.direction = Point2 { x: 0.0, y: 1.0 };
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D)
            && (self.head.direction.x != -1.0 || tail_length_equals_zero)
        {
            self.head.direction = Point2 { x: 1.0, y: 0.0 };
        }

        self.head.position.x =
            self.head.position.x + (self.head.direction.x * crate::GRID_CELL_SIZE_X);
        self.head.position.y =
            self.head.position.y + (self.head.direction.y * crate::GRID_CELL_SIZE_Y);

        //  wrap around screen if out of bounds
        if self.head.position.x == crate::SCREEN_SIZE_X {
            self.head.position.x = 0.0;
        } else if self.head.position.x < 0.0 {
            self.head.position.x = crate::SCREEN_SIZE_X - crate::GRID_CELL_SIZE_X;
        } else if self.head.position.y == crate::SCREEN_SIZE_Y {
            self.head.position.y = 0.0;
        } else if self.head.position.y < 0.0 {
            self.head.position.y = crate::SCREEN_SIZE_Y - crate::GRID_CELL_SIZE_Y;
        }

        //  tail
        for tail_segment in &mut self.tail {
            //  update tail_segment position to that of the one before it
            let local_position = tail_segment.position;
            tail_segment.position = position_cache;

            position_cache = local_position;

            //  check for head -> tail_segment collision, restart game if collision has occurred
            if self.head.position == tail_segment.position {
                return false;
            }
        }

        true
    }
}
