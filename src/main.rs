use macroquad::prelude::*;
use ::rand::random;

#[macroquad::main("basic")]
async fn main() {
    let char_size = 10.0;
    let mut width = screen_width();
    let mut height = screen_height();
    let spd = 2.0;

    let mut char_pos = vec2(width / 2.0, height / 2.0);

    loop {
        width = screen_width();
        height = screen_height();

        let mut randomX = random::<f32>() * width;
        let mut randomY = random::<f32>() * height;
        println!("{}, {}", randomX, randomY);

        draw_rectangle(randomX, randomY, 5.0, 5.0, GREEN);

        draw_rectangle(char_pos.x, char_pos.y, char_size, char_size, WHITE);

        if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) && char_pos.x > 0.0 {
            char_pos.x -= spd;
        }
        if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
            && (char_pos.x + char_size) < width
        {
            char_pos.x += spd;
        }
        if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) && char_pos.y > 0.0 {
            char_pos.y -= spd;
        }
        if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S))
            && char_pos.y + char_size < height
        {
            char_pos.y += spd;
        }
        next_frame().await
    }
}
