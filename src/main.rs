use macroquad::prelude::*;

struct Paddle {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  speed: f32,
  up_key: KeyCode,
  down_key: KeyCode
}
impl Paddle {
  fn new(paddle_x: f32, paddle_y: f32, paddle_up_key: KeyCode, paddle_down_key: KeyCode) -> Self {
    Self {
      x: paddle_x,
      y: paddle_y,
      width: 25.0,
      height: 200.0,
      speed: 700.0,
      up_key: paddle_up_key,
      down_key: paddle_down_key
    }
  }
  fn draw(&self) {
    draw_rectangle(self.x - (self.width / 2.0), self.y - (self.height / 2.0), self.width, self.height, WHITE);
  }
  fn update(&mut self, dt: f32) {
    let dir;

    if is_key_down(self.up_key) && is_key_down(self.down_key) {
      dir = 0.0; // if both keys are down, dont move
    }
    else if is_key_down(self.up_key) {
      dir = -1.0;
    }
    else if is_key_down(self.down_key) {
      dir = 1.0;
    }
    else {
      dir = 0.0
    }

    self.y += self.speed * dir * dt;
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
  let mut paddle_left = Paddle::new(40.0, screen_height() / 2.0, KeyCode::W, KeyCode::S);
  let mut paddle_right = Paddle::new(screen_width() - 40.0, screen_height() / 2.0, KeyCode::I, KeyCode::K);
  
  loop {
    /* LOGIC */
    paddle_left.update(get_frame_time());
    paddle_right.update(get_frame_time());

    /* RENDERING */
    clear_background(BLACK);

    paddle_left.draw();
    paddle_right.draw();

    next_frame().await
  }
}