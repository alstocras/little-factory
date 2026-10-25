#![allow(non_snake_case)]
mod factories;
mod gridutils;
mod types;

use factories::*;
use gridutils::*;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;
use macroquad::ui;
use macroquad::ui::hash;
use std::collections::*;
use types::*;

#[macroquad::main("Little Factory")]
async fn main() {
    set_pc_assets_folder("assets");

    let font = load_ttf_font("fonts/font.ttf").await.unwrap();
    set_default_font(font);

    let mut levelMap: HashMap<IVec2, TileOccupant> = HashMap::new();
    let mut nodeResourcesMap: HashMap<IVec2, Traveller> = HashMap::new();
    let mut playerPos: IVec2 = IVec2::new(1, 1);

    let mut numBasicResource: u32 = 0;

    let mut gameState: GameState = GameState::MainMenu;
    let mut buildingSelected: TileOccupant = TileOccupant::TransmitterNode;

    let rows: u32 = 10;
    let cols: u32 = 10;
    let startX: f32 = 100.0;
    let startY: f32 = 100.0;
    let cellSize: f32 = 50.0;

    // spritesheets
    let inputSpritesheet = load_texture("sprites/input.png")
        .await
        .expect("unable to load input");
    let outputSpritesheet = load_texture("sprites/output.png")
        .await
        .expect("unable to load output");

    inputSpritesheet.set_filter(FilterMode::Nearest);
    outputSpritesheet.set_filter(FilterMode::Nearest);

    // building the map
    levelMap.insert(IVec2::new(5, 5), TileOccupant::BasicResource);
    levelMap.insert(IVec2::new(6, 5), TileOccupant::BasicResource);
    levelMap.insert(IVec2::new(7, 5), TileOccupant::BasicResource);
    levelMap.insert(IVec2::new(8, 5), TileOccupant::BasicResource);
    levelMap.insert(IVec2::new(0, 0), TileOccupant::Input);
    levelMap.insert(IVec2::new(9, 9), TileOccupant::Output);

    // fill the input and output boxes
    nodeResourcesMap.insert(
        IVec2::new(0, 0),
        Traveller {
            itemType: Some(TravellerItem::GreenItem),
            amount: u64::MAX,
        },
    );
    nodeResourcesMap.insert(
        IVec2::new(9, 9),
        Traveller {
            itemType: Some(TravellerItem::PurpleItem),
            amount: u64::MIN,
        },
    );

    loop {
        clear_background(BLACK);

        match gameState {
            GameState::MainMenu => {
                if is_key_down(KeyCode::Space) {
                    gameState = GameState::Game;
                }
                if is_key_down(KeyCode::Escape) {
                    std::process::exit(1);
                }
                let titleDimen = measure_text("Little Factory", None, 50, 1.0);
                let helperDimen = measure_text("space to play, esc to quit", None, 30, 1.0);
                draw_text(
                    "Little Factory",
                    screen_width() / 2.0 - titleDimen.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    GOLD,
                );
                draw_text(
                    "space to play, esc to quit",
                    screen_width() / 2.0 - helperDimen.width / 2.0,
                    screen_height() / 2.0 + titleDimen.height + 25.0,
                    30.0,
                    WHITE,
                );
            }
            GameState::Win => {
                let winDimen = measure_text("successful transfer!", None, 40, 1.0);
                draw_text(
                    "succesful transfer!",
                    screen_width() / 2.0 - winDimen.width / 2.0,
                    screen_height() / 2.0,
                    40.0,
                    GOLD,
                );
            }
            GameState::Game => {
                // handle player input
                if is_key_pressed(KeyCode::Y) || is_key_pressed(KeyCode::W) {
                    levelMap.remove(&playerPos);
                    let targetPos = IVec2::new(playerPos.x, playerPos.y - 1);
                    if targetPos.y >= 0
                        && targetPos.y < rows as i32
                        && !levelMap.contains_key(&targetPos)
                    {
                        playerPos.y -= 1;
                    }
                }
                if is_key_pressed(KeyCode::I) || is_key_pressed(KeyCode::S) {
                    levelMap.remove(&playerPos);
                    let targetPos = IVec2::new(playerPos.x, playerPos.y + 1);
                    if targetPos.y >= 0
                        && targetPos.y < rows as i32
                        && !levelMap.contains_key(&targetPos)
                    {
                        playerPos.y += 1;
                    }
                }
                if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::D) {
                    levelMap.remove(&playerPos);
                    let targetPos = IVec2::new(playerPos.x + 1, playerPos.y);
                    if targetPos.x >= 0
                        && targetPos.x < rows as i32
                        && !levelMap.contains_key(&targetPos)
                    {
                        playerPos.x += 1;
                    }
                }
                if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::A) {
                    levelMap.remove(&playerPos);
                    let targetPos = IVec2::new(playerPos.x - 1, playerPos.y);
                    if targetPos.x >= 0
                        && targetPos.x < rows as i32
                        && !levelMap.contains_key(&targetPos)
                    {
                        playerPos.x -= 1;
                    }
                }

                // click to place selected building
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouseX, mouseY) = mouse_position();

                    let mouseCellX: i32 = ((mouseX - startX) / cellSize).floor() as i32;
                    let mouseCellY: i32 = ((mouseY - startY) / cellSize).floor() as i32;

                    let mouseCellPos = IVec2::new(mouseCellX, mouseCellY);

                    let dist =
                        (playerPos.x - mouseCellPos.x).abs() + (playerPos.y - mouseCellPos.y).abs();

                    if buildingSelected == TileOccupant::G2PFactory && numBasicResource >= 2 {
                        if !levelMap.contains_key(&mouseCellPos) && dist <= 3 {
                            levelMap.insert(mouseCellPos, TileOccupant::G2PFactory);
                            nodeResourcesMap.insert(
                                mouseCellPos,
                                Traveller {
                                    itemType: Some(TravellerItem::GreenItem),
                                    amount: 0,
                                },
                            );
                            numBasicResource -= 2;
                        } else {
                            println!("occupied/too far");
                        }
                    } else if buildingSelected == TileOccupant::TransmitterNode
                        && numBasicResource >= 1
                    {
                        if !levelMap.contains_key(&mouseCellPos) && dist <= 3 {
                            levelMap.insert(mouseCellPos, TileOccupant::TransmitterNode);
                            nodeResourcesMap.insert(
                                mouseCellPos,
                                Traveller {
                                    itemType: None,
                                    amount: 0,
                                },
                            );
                            numBasicResource -= 1;
                        } else {
                            println!("occupied/too far");
                        }
                    } else {
                        println!("not enough resources/nothing selected");
                    }
                }

                // actually run the  transfer simulation
                for (cellWithBuilding, buildingType) in levelMap.iter() {
                    match buildingType {
                        TileOccupant::TransmitterNode
                        | TileOccupant::Input
                        | TileOccupant::G2PFactory => {
                            let searchLimit: u32 = 5;

                            // south (0, 1)
                            raycastAndTransferItems(
                                searchLimit,
                                cellWithBuilding,
                                &mut nodeResourcesMap,
                                &levelMap,
                                ivec2(0, 1),
                            );

                            // north (0, -1)
                            raycastAndTransferItems(
                                searchLimit,
                                cellWithBuilding,
                                &mut nodeResourcesMap,
                                &levelMap,
                                ivec2(0, -1),
                            );

                            // east (1, 0)
                            raycastAndTransferItems(
                                searchLimit,
                                cellWithBuilding,
                                &mut nodeResourcesMap,
                                &levelMap,
                                ivec2(1, 0),
                            );

                            // west (-1, 0)
                            raycastAndTransferItems(
                                searchLimit,
                                cellWithBuilding,
                                &mut nodeResourcesMap,
                                &levelMap,
                                ivec2(-1, 0),
                            );
                        }
                        TileOccupant::Player | TileOccupant::BasicResource => {}
                        TileOccupant::Output => {}
                    }
                }

                // right click to take resource / remove building
                if is_mouse_button_pressed(MouseButton::Right) {
                    let (mouseX, mouseY) = mouse_position();

                    let mouseCellX: i32 = ((mouseX - startX) / cellSize).floor() as i32;
                    let mouseCellY: i32 = ((mouseY - startY) / cellSize).floor() as i32;

                    let mouseCellPos = IVec2::new(mouseCellX, mouseCellY);

                    let dist =
                        (playerPos.x - mouseCellPos.x).abs() + (playerPos.y - mouseCellPos.y).abs();

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
                        Some(TileOccupant::TransmitterNode) => {
                            println!("transmits resources orthogonally");
                            levelMap.remove(&mouseCellPos);
                            numBasicResource += 1;
                        }
                    }
                }

                levelMap.insert(playerPos, TileOccupant::Player);
                drawGrid(cellSize, GRAY, startX, startY, rows, cols, &levelMap);

                // check for win condition
                if nodeResourcesMap.get(&IVec2::new(9, 9)).unwrap().amount >= 2000 {
                    gameState = GameState::Win;
                }

                // UI RENDERING

                // show bromine count
                let basicResLabel = format!("Br: {}", numBasicResource);
                let basicResDimen = measure_text(&basicResLabel, None, 30, 1.0);
                draw_text(
                    &basicResLabel,
                    screen_width() - basicResDimen.width - 25.0,
                    screen_height() - basicResDimen.height - 25.0,
                    30.0,
                    BLUE,
                );

                // show progress of plutonium
                let resourcesInOutput = nodeResourcesMap.get(&IVec2::new(9, 9)).unwrap().amount;
                let progLabel = format!("Transferred: {} / 2k", resourcesInOutput);
                let progLabelDimen = measure_text(&progLabel, None, 30, 1.0);
                draw_text(
                    &progLabel,
                    screen_width() - progLabelDimen.width - 25.0,
                    screen_height() - basicResDimen.height - progLabelDimen.height - 30.0,
                    30.0,
                    PURPLE,
                );

                // the build menu
                if ui::root_ui().button(None, "Transmitter") {
                    buildingSelected = TileOccupant::TransmitterNode;
                }
                if ui::root_ui().button(None, "Ge-Pu Factory") {
                    buildingSelected = TileOccupant::G2PFactory;
                }
            }
        }

        next_frame().await;
    }
}
