#![allow(non_snake_case)]
use macroquad::prelude::*;

#[macroquad::main("Little Factory")]
async fn main() {
    loop {
        clear_background(BLACK);

        let cellSize = 50.0;
        let mut currentX: f32 = 0.0;
        let mut currentY: f32 = 0.0;

        while currentX < screen_width() {
            draw_line(currentX, 0.0, currentX, screen_height(), 1.0, RED);
            currentX += cellSize;
        }

        while currentY < screen_height() {
            draw_line(0.0, currentY, screen_width(), currentY, 1.0, RED);
            currentY += cellSize;
        }

        next_frame().await;
    }
}
