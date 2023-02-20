use ::rand::random;
use macroquad::prelude::*;
use std::marker::Copy;

#[derive(Copy, Clone)]
struct Item {
    x: f32,
    y: f32,
}

#[macroquad::main("basic")]
async fn main() {
    let mut time = 0.0;

    let mut char_size = 10.0;
    let mut width = screen_width();
    let mut height = screen_height();
    let mut char_spd = 2.0;

    let mut char_pos = vec2(width / 2.0, height / 2.0);

    const ITEM_LIMIT: usize = 20;

    let mut items: [Item; ITEM_LIMIT] = [Item { x: 0.0, y: 0.0 }; ITEM_LIMIT];

    for i in 0..ITEM_LIMIT {
        items[i] = Item {
            x: random::<f32>() * width,
            y: random::<f32>() * height,
        };
    }

    loop {
        width = screen_width();
        height = screen_height();

        for i in 0..ITEM_LIMIT {
            draw_rectangle(items[i].x, items[i].y, 5.0, 5.0, GREEN);
            if (char_pos.x - items[i].x).abs() < char_size
                && (char_pos.y - items[i].y).abs() < char_size
                && (char_size < width || char_size < height)
            {
                char_size += 5.0;
                char_spd += 1.0;
                items[i] = Item {
                    x: random::<f32>() * width,
                    y: random::<f32>() * height,
                };
            }
        }

        // Handling player movement on key down
        if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) && char_pos.x > 0.0 {
            char_pos.x -= char_spd;
        }
        if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
            && (char_pos.x + char_size) < width
        {
            char_pos.x += char_spd;
        }
        if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) && char_pos.y > 0.0 {
            char_pos.y -= char_spd;
        }
        if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S))
            && char_pos.y + char_size < height
        {
            char_pos.y += char_spd;
        }

        if char_size >= width && char_size >= height {
            // Drawing the player after win

            draw_rectangle(0.0, 0.0, char_size, char_size, WHITE);

            draw_text("YOU WIN!", width / 2.0, height / 2.0, 20.0, BLACK);
            draw_text(
                &format!("time: {}", time),
                width / 2.0,
                height / 2.0 + 40.0,
                20.0,
                BLACK,
            );
        } else {
            // Drawing the player after calculating the new position
            draw_rectangle(char_pos.x, char_pos.y, char_size, char_size, WHITE);

            time += get_frame_time();
            draw_text(&format!("time: {}", time), 10.0, 10.0, 20.0, WHITE);
        }

        println!("{} {} {}", char_size, width, height);

        next_frame().await
    }
}
