#![allow(non_snake_case)]
use macroquad::prelude::*;
use ndarray::*;
use std::collections::*;

fn drawGrid(
    cellSize: f32,
    colour: Color,
    startX: f32,
    startY: f32,
    rows: f32,
    cols: f32,
    levelMap: &HashMap<IVec2, TileOccupant>,
) {
    // draw lines
    let mut currentX: f32 = startX;
    let mut currentY: f32 = startY;

    while currentX < rows * cellSize {
        draw_line(currentX, startY, currentX, cols * cellSize, 1.0, colour);
        currentX += cellSize;
    }

    while currentY < cols * cellSize {
        draw_line(startX, currentY, rows * cellSize, currentY, 1.0, colour);
        currentY += cellSize;
    }

    // draw objects
    for (pos, occupant) in levelMap.iter() {
        let pixelX = (pos.x * cellSize as i32) as f32 + startX;
        let pixelY = (pos.y * cellSize as i32) as f32 + startY;

        match occupant {
            TileOccupant::PLAYER => draw_rectangle(pixelX, pixelY, cellSize, cellSize, RED),
        };
    }
}

enum TileOccupant {
    PLAYER,
}

#[macroquad::main("Little Factory")]
async fn main() {
    let mut levelMap: HashMap<IVec2, TileOccupant> = HashMap::new();
    let mut playerPos: IVec2 = IVec2::new(0, 0);

    loop {
        clear_background(BLACK);

        // handle player input
        if is_key_pressed(KeyCode::Y) || is_key_pressed(KeyCode::W) {
            levelMap.remove(&playerPos);
            playerPos.y -= 1
        }
        if is_key_pressed(KeyCode::I) || is_key_pressed(KeyCode::S) {
            levelMap.remove(&playerPos);
            playerPos.y += 1
        }
        if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::D) {
            levelMap.remove(&playerPos);
            playerPos.x += 1
        }
        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::A) {
            levelMap.remove(&playerPos);
            playerPos.x -= 1
        }

        levelMap.insert(playerPos, TileOccupant::PLAYER);
        drawGrid(50.0, GRAY, 100.0, 100.0, 10.0, 10.0, &levelMap);

        next_frame().await;
    }
}
