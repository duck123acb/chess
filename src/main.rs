use macroquad::prelude::*;

struct Paddle {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32
}

impl Paddle {
  fn new(paddle_x: f32, paddle_y: f32) -> Paddle {
    Paddle {
      x: paddle_x,
      y: paddle_y,
      width: 25.0,
      height: 200.0
    }
  }
  fn draw(&self) {
    draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
  }
}

fn window_conf() -> Conf {
  Conf {
    window_title: "chess".to_string(),
    window_width: 800,
    window_height: 600,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let paddle = Paddle::new(0.0, 0.0);
  loop {
    clear_background(BLACK);

    paddle.draw();

    next_frame().await
  }
}