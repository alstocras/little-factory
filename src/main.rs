#![allow(non_snake_case)]
use macroquad::prelude::*;
use ndarray::*;
use std::collections::*;

use crate::TileOccupant::BasicResource;

fn drawGrid(
    cellSize: f32,
    colour: Color,
    startX: f32,
    startY: f32,
    rows: u32,
    cols: u32,
    levelMap: &HashMap<IVec2, TileOccupant>,
) {
    // draw lines
    let mut currentX: f32 = startX;
    let mut currentY: f32 = startY;

    while currentX <= rows as f32 * cellSize + startX {
        draw_line(
            currentX,
            startY,
            currentX,
            cols as f32 * cellSize + startX,
            1.0,
            colour,
        );
        currentX += cellSize;
    }

    while currentY <= cols as f32 * cellSize + startY {
        draw_line(
            startX,
            currentY,
            rows as f32 * cellSize + startY,
            currentY,
            1.0,
            colour,
        );
        currentY += cellSize;
    }

    // draw objects
    for (pos, occupant) in levelMap.iter() {
        let pixelX = (pos.x * cellSize as i32) as f32 + startX;
        let pixelY = (pos.y * cellSize as i32) as f32 + startY;

        match occupant {
            TileOccupant::Player => draw_rectangle(pixelX, pixelY, cellSize, cellSize, RED),
            TileOccupant::BasicResource => draw_rectangle(pixelX, pixelY, cellSize, cellSize, BLUE),
        };
    }
}

enum TileOccupant {
    Player,
    BasicResource,
}

#[macroquad::main("Little Factory")]
async fn main() {
    let mut levelMap: HashMap<IVec2, TileOccupant> = HashMap::new();
    let mut playerPos: IVec2 = IVec2::new(0, 0);

    let rows: u32 = 10;
    let cols: u32 = 10;

    levelMap.insert(IVec2::new(5, 5), TileOccupant::BasicResource);
    loop {
        clear_background(BLACK);

        // handle player input
        if is_key_pressed(KeyCode::Y) || is_key_pressed(KeyCode::W) {
            levelMap.remove(&playerPos);
            let targetPos = IVec2::new(playerPos.x, playerPos.y - 1);
            if targetPos.y >= 0 && targetPos.y < rows as i32 && !levelMap.contains_key(&targetPos) {
                playerPos.y -= 1;
            }
        }
        if is_key_pressed(KeyCode::I) || is_key_pressed(KeyCode::S) {
            levelMap.remove(&playerPos);
            let targetPos = IVec2::new(playerPos.x, playerPos.y + 1);
            if targetPos.y >= 0 && targetPos.y < rows as i32 && !levelMap.contains_key(&targetPos) {
                playerPos.y += 1;
            }
        }
        if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::D) {
            levelMap.remove(&playerPos);
            let targetPos = IVec2::new(playerPos.x + 1, playerPos.y);
            if targetPos.x >= 0 && targetPos.x < rows as i32 && !levelMap.contains_key(&targetPos) {
                playerPos.x += 1;
            }
        }
        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::A) {
            levelMap.remove(&playerPos);
            let targetPos = IVec2::new(playerPos.x - 1, playerPos.y);
            if targetPos.x >= 0 && targetPos.x < rows as i32 && !levelMap.contains_key(&targetPos) {
                playerPos.x -= 1;
            }
        }

        levelMap.insert(playerPos, TileOccupant::Player);
        drawGrid(50.0, GRAY, 100.0, 100.0, rows, cols, &levelMap);

        next_frame().await;
    }
}
