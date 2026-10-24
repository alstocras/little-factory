use crate::types::*;
use macroquad::prelude::*;
use std::collections::*;

pub fn drawGrid(
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
            TileOccupant::TransmitterNode => {
                // draw a TransmitterNode
                draw_rectangle(pixelX, pixelY, cellSize, cellSize, GOLD);

                // step forward in each direction until exceeds limit/finds a valid node
                let searchLimit: u32 = 5;
                let mut tilesSearched: u32 = 0;
                let mut tileLookingAt: IVec2 = *pos;
                let mut locatedTransmitter: IVec2 = *pos;

                // north (0, -1)
                while tilesSearched < searchLimit {
                    tileLookingAt = ivec2(tileLookingAt.x, tileLookingAt.y - 1);
                    let activeTileHolder = levelMap.get(&tileLookingAt);
                    match activeTileHolder {
                        Some(TileOccupant::TransmitterNode)
                        | Some(TileOccupant::Input)
                        | Some(TileOccupant::Output)
                        | Some(TileOccupant::G2PFactory) => {
                            locatedTransmitter = tileLookingAt;
                            // draw the line
                            draw_line(
                                pixelX + cellSize / 2.0,
                                pixelY + cellSize / 2.0,
                                locatedTransmitter.x as f32 * cellSize + startX + cellSize / 2.0,
                                locatedTransmitter.y as f32 * cellSize + startY + cellSize / 2.0,
                                10.0,
                                GOLD,
                            );
                            break;
                        }
                        Some(TileOccupant::Player) | Some(TileOccupant::BasicResource) | None => {}
                    };
                    tilesSearched += 1;
                }
                tilesSearched = 0;
                tileLookingAt = *pos;

                // south (0, 1)
                while tilesSearched < searchLimit {
                    tileLookingAt = ivec2(tileLookingAt.x, tileLookingAt.y + 1);
                    let activeTileHolder = levelMap.get(&tileLookingAt);
                    match activeTileHolder {
                        Some(TileOccupant::TransmitterNode)
                        | Some(TileOccupant::Input)
                        | Some(TileOccupant::Output)
                        | Some(TileOccupant::G2PFactory) => {
                            locatedTransmitter = tileLookingAt;
                            // draw the line
                            draw_line(
                                pixelX + cellSize / 2.0,
                                pixelY + cellSize / 2.0,
                                locatedTransmitter.x as f32 * cellSize + startX + cellSize / 2.0,
                                locatedTransmitter.y as f32 * cellSize + startY + cellSize / 2.0,
                                10.0,
                                GOLD,
                            );
                            break;
                        }
                        Some(TileOccupant::Player) | Some(TileOccupant::BasicResource) | None => {}
                    };
                    tilesSearched += 1;
                }
                tilesSearched = 0;
                tileLookingAt = *pos;

                // west (-1, 0)
                while tilesSearched < searchLimit {
                    tileLookingAt = ivec2(tileLookingAt.x - 1, tileLookingAt.y);
                    let activeTileHolder = levelMap.get(&tileLookingAt);
                    match activeTileHolder {
                        Some(TileOccupant::TransmitterNode)
                        | Some(TileOccupant::Input)
                        | Some(TileOccupant::Output)
                        | Some(TileOccupant::G2PFactory) => {
                            locatedTransmitter = tileLookingAt;
                            // draw the line
                            draw_line(
                                pixelX + cellSize / 2.0,
                                pixelY + cellSize / 2.0,
                                locatedTransmitter.x as f32 * cellSize + startX + cellSize / 2.0,
                                locatedTransmitter.y as f32 * cellSize + startY + cellSize / 2.0,
                                10.0,
                                GOLD,
                            );
                            break;
                        }
                        Some(TileOccupant::Player) | Some(TileOccupant::BasicResource) | None => {}
                    };
                    tilesSearched += 1;
                }
                tilesSearched = 0;
                tileLookingAt = *pos;

                // east (1, 0)
                while tilesSearched < searchLimit {
                    tileLookingAt = ivec2(tileLookingAt.x + 1, tileLookingAt.y);
                    let activeTileHolder = levelMap.get(&tileLookingAt);
                    match activeTileHolder {
                        Some(TileOccupant::TransmitterNode)
                        | Some(TileOccupant::Input)
                        | Some(TileOccupant::Output)
                        | Some(TileOccupant::G2PFactory) => {
                            locatedTransmitter = tileLookingAt;
                            // draw the line
                            draw_line(
                                pixelX + cellSize / 2.0,
                                pixelY + cellSize / 2.0,
                                locatedTransmitter.x as f32 * cellSize + startX + cellSize / 2.0,
                                locatedTransmitter.y as f32 * cellSize + startY + cellSize / 2.0,
                                10.0,
                                GOLD,
                            );
                            break;
                        }
                        Some(TileOccupant::Player) | Some(TileOccupant::BasicResource) | None => {}
                    };
                    tilesSearched += 1;
                }
            }
        };
    }
}

// raycasting stuff thing transferrer idk
pub fn raycastAndTransferItems(
    searchLimit: u32,
    cellWithBuilding: &IVec2,
    mut nodeResourcesMap: HashMap<IVec2, Traveller>,
    levelMap: HashMap<IVec2, TileOccupant>,
    direction: IVec2,
) {
    // some essentials
    let mut tilesSearched: u32 = 0;
    let mut tileLookingAt: IVec2 = *cellWithBuilding;

    // the actual function
    while tilesSearched < searchLimit {
        tileLookingAt = ivec2(tileLookingAt.x + direction.x, tileLookingAt.y + direction.y);
        let activeTileHolder = levelMap.get(&tileLookingAt);
        match activeTileHolder {
            Some(TileOccupant::TransmitterNode)
            | Some(TileOccupant::Input)
            | Some(TileOccupant::Output)
            | Some(TileOccupant::G2PFactory) => {
                let mut resourcesPresentDest: u64 = 0;
                let mut resourcesPresentSource: u64 = 0;
                if let Some(RPD) = nodeResourcesMap.get(&tileLookingAt) {
                    resourcesPresentDest = RPD.amount;
                }
                if let Some(RPS) = nodeResourcesMap.get(cellWithBuilding) {
                    resourcesPresentSource = RPS.amount
                }

                if resourcesPresentDest < resourcesPresentSource {
                    resourcesPresentSource -= 1;
                    resourcesPresentDest += 1;

                    if let Some(RPD) = nodeResourcesMap.get_mut(&tileLookingAt) {
                        RPD.amount = resourcesPresentDest;
                    }
                    if let Some(RPS) = nodeResourcesMap.get_mut(cellWithBuilding) {
                        RPS.amount = resourcesPresentSource;
                    }
                    println!(
                        "source: {}, dest: {}",
                        resourcesPresentSource, resourcesPresentDest
                    );
                }
                break;
            }
            Some(TileOccupant::Player) | Some(TileOccupant::BasicResource) | None => {}
        };
        tilesSearched += 1;
    }
}
