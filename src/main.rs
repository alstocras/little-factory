#![allow(non_snake_case)]
use macroquad::prelude::*;
use macroquad::ui;
use macroquad::ui::hash;
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
            TileOccupant::G2PFactory => draw_rectangle(pixelX, pixelY, cellSize, cellSize, GRAY),
        };
    }
}

enum TileOccupant {
    Player,
    BasicResource,
    Input,
    Output,
    G2PFactory,
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

    // building the map
    levelMap.insert(IVec2::new(5, 5), TileOccupant::BasicResource);
    levelMap.insert(IVec2::new(6, 6), TileOccupant::BasicResource);
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

        // click to place (only G2PFactory for now)
        if is_mouse_button_pressed(MouseButton::Left) {
            if numBasicResource >= 2 {
                let (mouseX, mouseY) = mouse_position();

                let mouseCellX: i32 = ((mouseX - startX) / cellSize).floor() as i32;
                let mouseCellY: i32 = ((mouseY - startY) / cellSize).floor() as i32;

                let mouseCellPos = IVec2::new(mouseCellX, mouseCellY);

                let dist =
                    (playerPos.x - mouseCellPos.x).abs() + (playerPos.y - mouseCellPos.y).abs();

                if !levelMap.contains_key(&mouseCellPos) && dist <= 3 {
                    levelMap.insert(mouseCellPos, TileOccupant::G2PFactory);
                    numBasicResource -= 2;
                } else {
                    println!("occupied/too far");
                }
            } else {
                println!("not enough resources");
            }
        }

        // right click to take resource / remove building
        if is_mouse_button_pressed(MouseButton::Right) {
            let (mouseX, mouseY) = mouse_position();

            let mouseCellX: i32 = ((mouseX - startX) / cellSize).floor() as i32;
            let mouseCellY: i32 = ((mouseY - startY) / cellSize).floor() as i32;

            let mouseCellPos = IVec2::new(mouseCellX, mouseCellY);

            let dist = (playerPos.x - mouseCellPos.x).abs() + (playerPos.y - mouseCellPos.y).abs();

            match levelMap.get(&mouseCellPos) {
                None => println!("nothing here"),
                Some(TileOccupant::BasicResource) => {
                    if dist <= 3 {
                        numBasicResource += 1;
                        levelMap.remove(&mouseCellPos);
                        println!("{}", numBasicResource);
                    } else {
                        println!("too far!");
                    }
                }
                Some(TileOccupant::Input) => {}
                Some(TileOccupant::Output) => {}
                Some(TileOccupant::Player) => {}
                Some(TileOccupant::G2PFactory) => {
                    println!("converts green to purple");
                    levelMap.remove(&mouseCellPos);
                    numBasicResource += 2;
                }
            }
        }

        levelMap.insert(playerPos, TileOccupant::Player);
        drawGrid(cellSize, GRAY, startX, startY, rows, cols, &levelMap);

        // show resource count
        let basicResLabel = format!("Br: {}", numBasicResource);
        let basicResDimen = measure_text(&basicResLabel, None, 30, 1.0);
        draw_text(
            &basicResLabel,
            screen_width() - basicResDimen.width - 75.0,
            basicResDimen.height + 50.0,
            30.0,
            WHITE,
        );

        ui::widgets::Window::new(
            ui::hash!(),
            vec2(screen_width() - 75.0, 75.0),
            vec2(100.0, screen_height()),
        )
        .label("infobox")
        .ui(&mut *ui::root_ui(), |ui| {
            ui.label(vec2(20.0, 20.0), "hello");
        });

        next_frame().await;
    }
}
