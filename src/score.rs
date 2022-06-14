use ggez::graphics;

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

    pub fn add_to_score(&mut self) {
        self.value = self.value + 10;
        self.text = graphics::Text::new(format!("Score: {}", self.value));
        self.text.set_font(
            graphics::Font::default(),
            graphics::PxScale {
                x: crate::FONT_SCALE_X,
                y: crate::FONT_SCALE_Y,
            },
        );
    }
}
