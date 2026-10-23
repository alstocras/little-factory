#![allow(non_snake_case)]
use macroquad::prelude::*;

#[macroquad::main("Little Factory")]
async fn main() {
    loop {
        clear_background(RED);
        next_frame().await;
    }
}
