use macroquad::prelude::*;
#[macroquad::main("mahjong")]
async fn main() {
    loop {
        let mahjong_height = screen_height() / 16.0;
        let mahjong_width = mahjong_height / 1.2;
        let mahjong_gap = 10.0;
        let mahjong_offset = mahjong_width + mahjong_gap;

        clear_background(BLACK);

        for i in 0..17 {
            draw_rectangle(
                screen_width() / 20.0 + (mahjong_offset * i as f32),
                screen_height() - mahjong_height,
                mahjong_width,
                mahjong_height,
                GREEN,
            );
        }

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
