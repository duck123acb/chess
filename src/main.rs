use macroquad::prelude::*;

fn window_conf() -> Conf {
  Conf {
    window_title: "chess".to_string(),
    window_width: 800,
    window_height: 600,
    window_resizable: false,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  
  loop {
    /* LOGIC */


    /* RENDERING */
    clear_background(BLACK);


    next_frame().await
  }
}