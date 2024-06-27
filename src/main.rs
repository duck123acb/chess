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

struct Piece {

}
struct Square {
  rect: Rect,
  piece: Piece
}

fn draw_from_atlas(texture: &Texture2D, texture_x: i32, texture_y: i32, width: i32, height: i32, dest_x: f32, dest_y: f32) {
  let params = DrawTextureParams {
    dest_size: Some(vec2(width as f32, height as f32)),
    source: Some(Rect {x: texture_x as f32, y: texture_y as f32, w: width as f32, h: height as f32}),
    ..Default::default()
  };

  draw_texture_ex(texture, dest_x, dest_y, WHITE, params);
}

#[macroquad::main(window_conf)]
async fn main() {
  let texture_atlas = load_texture("assets/pieces.png").await.unwrap();
  texture_atlas.set_filter(FilterMode::Nearest);



  loop {
    /* LOGIC */
    

    /* RENDERING */
    clear_background(BLACK);
    
    draw_from_atlas(&texture_atlas, 0, 0, 130, 130, 0.0, 0.0); // Draw the first sprite

    next_frame().await
  }
}