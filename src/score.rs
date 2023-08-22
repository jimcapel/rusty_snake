use ggez::graphics;
use ggez::mint::*;
use ggez::{Context, GameResult};

pub struct Score {
    pub text: graphics::Text,
    value: u32,
}

impl Score {
    pub fn new() -> Self {
        let value = 0;
        let mut text = graphics::Text::new(format!("Score: {}", value));
        text.set_font(
            graphics::Font::default(),
            graphics::PxScale {
                x: crate::FONT_SCALE_X,
                y: crate::FONT_SCALE_Y,
            },
        );
        Score { value, text }
    }

    pub fn change_score(&mut self, value: u32) {
        self.value = value;
        self.text = graphics::Text::new(format!("Score: {}", self.value));
        self.text.set_font(
            graphics::Font::default(),
            graphics::PxScale {
                x: crate::FONT_SCALE_X,
                y: crate::FONT_SCALE_Y,
            },
        );
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam::default().dest(Point2 {
                x: crate::FONT_POSITION_X,
                y: crate::FONT_POSITION_Y,
            }),
        )?;

        Ok(())
    }
}
