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
            TileOccupant::Input => draw_rectangle(pixelX, pixelY, cellSize, cellSize, GREEN),
            TileOccupant::Output => draw_rectangle(pixelX, pixelY, cellSize, cellSize, PURPLE),
        };
    }
}

enum TileOccupant {
    Player,
    BasicResource,
    Input,
    Output,
}

#[macroquad::main("Little Factory")]
async fn main() {
    let mut levelMap: HashMap<IVec2, TileOccupant> = HashMap::new();
    let mut playerPos: IVec2 = IVec2::new(1, 1);

    let mut numBasicResource: u32 = 0;

    let rows: u32 = 10;
    let cols: u32 = 10;
    let startX: f32 = 100.0;
    let startY: f32 = 100.0;
    let cellSize: f32 = 50.0;

    levelMap.insert(IVec2::new(5, 5), TileOccupant::BasicResource);
    levelMap.insert(IVec2::new(0, 0), TileOccupant::Input);
    levelMap.insert(IVec2::new(9, 9), TileOccupant::Output);
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

        // click to take resource
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouseX, mouseY) = mouse_position();

            let mouseCellX: i32 = ((mouseX - startX) / cellSize).floor() as i32;
            let mouseCellY: i32 = ((mouseY - startY) / cellSize).floor() as i32;

            let mouseCellPos = IVec2::new(mouseCellX, mouseCellY);

            match levelMap.get(&mouseCellPos) {
                None => println!("nothing here"),
                Some(TileOccupant::BasicResource) => {
                    numBasicResource += 1;
                    levelMap.remove(&mouseCellPos);
                    println!("{}", numBasicResource);
                }
                Some(TileOccupant::Input) => {}
                Some(TileOccupant::Output) => {}
                Some(TileOccupant::Player) => {}
            }
        }

        levelMap.insert(playerPos, TileOccupant::Player);
        drawGrid(cellSize, GRAY, startX, startY, rows, cols, &levelMap);

        next_frame().await;
    }
}
