use ggez::*;

pub struct Lobby {
    text_field: String,
    //logo: graphics::Image,
}

impl Lobby {
    pub fn new(/*logo: graphics::Image*/) -> Self {
        Self {
            text_field: "".to_owned(),
            //logo,
        }
    }

    pub fn run(&mut self, ctx: &mut Context) -> Option<String> {
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key0) {
            self.text_field.push_str("0");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key1) {
            self.text_field.push_str("1");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key2) {
            self.text_field.push_str("2");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key3) {
            self.text_field.push_str("3");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key4) {
            self.text_field.push_str("4");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key5) {
            self.text_field.push_str("5");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key6) {
            self.text_field.push_str("6");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key7) {
            self.text_field.push_str("7");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key8) {
            self.text_field.push_str("8");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Key9) {
            self.text_field.push_str("9");
        }
        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Back) {
            let mut chars = self.text_field.chars();
            chars.next_back();
            self.text_field = chars.as_str().to_owned();
        }

        if self.text_field.len() > 4 {
            self.text_field = self.text_field[0..4].to_owned();
        }

        self.render(ctx);

        if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Return) && self.text_field.len() == 4 {
            Some(format!("macro{}", self.text_field))
        } else if ctx.keyboard.is_key_just_pressed(input::keyboard::KeyCode::Return) && self.text_field.len() == 0 {
            Some("macro?next=2".to_owned())
        } else {
            None
        }
    }

    fn render(&self, ctx: &mut Context) {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
    
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2{x: 200.0, y: 300.0},
            100.0,
            0.1,
            graphics::Color::WHITE,
        );
        if let Ok(circle) = circle {
            canvas.draw(&circle, graphics::DrawParam::default());
        }
        let _ = canvas.finish(ctx);
        /*clear_background(BLACK);
        let dest_x = screen_width() / 2.0;
        let dest_y = self.logo.height() * (dest_x / self.logo.width());
        draw_texture_ex(
            self.logo,
            screen_width() / 2. - dest_x / 2.,
            20.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_x, dest_y)),
                ..Default::default()
            },
        );
        let text_x = screen_width() / 2. + dest_x / 2. - 120.;
        draw_text("DEMO", text_x, dest_y + 30., 50., WHITE);
        draw_text(
            "- enter a lobby ID (4 digits) to play with a friend",
            20.0,
            dest_y + 60.0,
            30.0,
            WHITE,
        );
        draw_text(
            "- leave empty to get matched against a random person",
            20.0,
            dest_y + 90.0,
            30.0,
            WHITE,
        );
        draw_text(
            "- Then, press ENTER to start!",
            20.0,
            dest_y + 120.0,
            30.0,
            WHITE,
        );

        let lobby_code_str = format!("Lobby Code: {}", self.text_field);
        draw_text(&lobby_code_str, 20.0, dest_y + 200.0, 80.0, YELLOW);*/
    }
}