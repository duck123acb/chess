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
fn draw_from_atlas(texture: &Texture2D, texture_x: i32, texture_y: i32, texture_rect: Rect) { // macroquad doesn't have a built in function for drawing from an atlas
  let params = DrawTextureParams {
    dest_size: Some(vec2(texture_rect.w, texture_rect.h)),
    source: Some(Rect {x: texture_x as f32, y: texture_y as f32, w: texture_rect.w, h: texture_rect.h}),
    ..Default::default()
  };

  draw_texture_ex(texture, texture_rect.x, texture_rect.y, WHITE, params);
}

struct PieceSprite {
  rect: Rect,
  texture: Texture2D,
  piece_type: char // change to the dedicated piece enum later
}
impl PieceSprite {
  fn new(sprite_x: i32, sprite_y: i32, sprite_texture: Texture2D, sprite_type: char) -> Self {
    Self {
      rect: Rect::new(sprite_x as f32, sprite_y as f32, 130f32, 130f32),
      texture: sprite_texture,
      piece_type: sprite_type
    }
  }

  fn update(&mut self) {

  }
  fn draw(&self) {
    // depending on type, update the texture_x and texture_y
    draw_from_atlas(&self.texture, 0, 0, self.rect);
  }
}
struct Square {
  rect: Rect,
  piece: PieceSprite
}

#[macroquad::main(window_conf)]
async fn main() {
  let texture_atlas = load_texture("assets/pieces.png").await.unwrap();
  texture_atlas.set_filter(FilterMode::Nearest);

  loop {
    /* LOGIC */

    /* RENDERING */
    clear_background(BLACK);
    
    next_frame().await
  }
}