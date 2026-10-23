#![allow(non_snake_case)]
use macroquad::prelude::*;

fn drawGrid(cellSize: f32, colour: Color) {
    let mut currentX: f32 = 0.0;
    let mut currentY: f32 = 0.0;

    while currentX < screen_width() {
        draw_line(currentX, 0.0, currentX, screen_height(), 1.0, colour);
        currentX += cellSize;
    }

    while currentY < screen_height() {
        draw_line(0.0, currentY, screen_width(), currentY, 1.0, colour);
        currentY += cellSize;
    }
}

#[macroquad::main("Little Factory")]
async fn main() {
    loop {
        clear_background(BLACK);

        drawGrid(50.0, GRAY);

        next_frame().await;
    }
}
