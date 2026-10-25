// gridutils.rs
use crate::factories::*;
use crate::types::*;
use macroquad::experimental::animation::*;
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
    inputAnim: &mut AnimatedSprite,
    inputTexture: &Texture2D,
    outputAnim: &mut AnimatedSprite,
    outputTexture: &Texture2D,
    playerAnim: &mut AnimatedSprite,
    playerTexture: &Texture2D,
    basicResourceAnim: &mut AnimatedSprite,
    basicResourceTexture: &Texture2D,
    g2pfactoryAnim: &mut AnimatedSprite,
    g2pfactoryTexture: &Texture2D,
    transmitterAnim: &mut AnimatedSprite,
    transmitterTexture: &Texture2D,
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
            TileOccupant::Player => {
                let frame = playerAnim.frame();

                draw_texture_ex(
                    playerTexture,
                    pixelX,
                    pixelY,
                    WHITE,
                    DrawTextureParams {
                        source: Some(frame.source_rect),
                        dest_size: Some(frame.dest_size),
                        ..Default::default()
                    },
                );
                playerAnim.update();
            }
            TileOccupant::BasicResource => {
                let frame = basicResourceAnim.frame();

                draw_texture_ex(
                    basicResourceTexture,
                    pixelX,
                    pixelY,
                    WHITE,
                    DrawTextureParams {
                        source: Some(frame.source_rect),
                        dest_size: Some(frame.dest_size),
                        ..Default::default()
                    },
                );
                basicResourceAnim.update();
            }
            TileOccupant::Input => {
                let frame = inputAnim.frame();

                draw_texture_ex(
                    inputTexture,
                    pixelX,
                    pixelY,
                    WHITE,
                    DrawTextureParams {
                        source: Some(frame.source_rect),
                        dest_size: Some(frame.dest_size),
                        ..Default::default()
                    },
                );
                inputAnim.update();
            }
            TileOccupant::Output => {
                let frame = outputAnim.frame();

                draw_texture_ex(
                    outputTexture,
                    pixelX,
                    pixelY,
                    WHITE,
                    DrawTextureParams {
                        source: Some(frame.source_rect),
                        dest_size: Some(frame.dest_size),
                        ..Default::default()
                    },
                );
                outputAnim.update();
            }
            TileOccupant::G2PFactory => {
                let frame = g2pfactoryAnim.frame();

                draw_texture_ex(
                    g2pfactoryTexture,
                    pixelX,
                    pixelY,
                    WHITE,
                    DrawTextureParams {
                        source: Some(frame.source_rect),
                        dest_size: Some(frame.dest_size),
                        ..Default::default()
                    },
                );
                g2pfactoryAnim.update();
            }
            TileOccupant::TransmitterNode => {
                // draw a TransmitterNode
                let frame = transmitterAnim.frame();

                draw_texture_ex(
                    transmitterTexture,
                    pixelX,
                    pixelY,
                    WHITE,
                    DrawTextureParams {
                        source: Some(frame.source_rect),
                        dest_size: Some(frame.dest_size),
                        ..Default::default()
                    },
                );
                transmitterAnim.update();

                // step forward in each direction until exceeds limit/finds a valid node
                let searchLimit: u32 = 5;
                let mut tilesSearched: u32 = 0;
                let mut tileLookingAt: IVec2 = *pos;
                let mut locatedTransmitter: IVec2;

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
    nodeResourcesMap: &mut HashMap<IVec2, Traveller>,
    levelMap: &HashMap<IVec2, TileOccupant>,
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
            | Some(TileOccupant::Output)
            | Some(TileOccupant::G2PFactory) => {
                let mut resourcesPresentDest: u64 = 0;
                let mut resourcesPresentSource: u64 = 0;
                let mut resourceTypeDest: Option<TravellerItem> = None;
                let mut resourceTypeSource: Option<TravellerItem> = None;

                if activeTileHolder == Some(&TileOccupant::G2PFactory) {
                    convertItemType(
                        nodeResourcesMap,
                        TravellerItem::GreenItem,
                        TravellerItem::PurpleItem,
                        tileLookingAt,
                    );
                }

                // read the dest and source resources
                if let Some(RPD) = nodeResourcesMap.get(&tileLookingAt) {
                    resourcesPresentDest = RPD.amount;
                    resourceTypeDest = RPD.itemType;
                }
                if let Some(RPS) = nodeResourcesMap.get(cellWithBuilding) {
                    resourcesPresentSource = RPS.amount;
                    resourceTypeSource = RPS.itemType;
                }
                println!(
                    "source {:?}: type={:?}, amount={}; destination {:?}: type={:?}, amount={}",
                    cellWithBuilding,
                    resourceTypeSource,
                    resourcesPresentSource,
                    tileLookingAt,
                    resourceTypeDest,
                    resourcesPresentDest,
                );

                // write to the values if it can
                if resourcesPresentSource > 0
                    && (resourceTypeDest.is_none() || resourceTypeDest == resourceTypeSource)
                    && (levelMap.get(cellWithBuilding) != Some(&TileOccupant::G2PFactory)
                        || resourceTypeSource == Some(TravellerItem::PurpleItem))
                {
                    resourcesPresentSource -= 1;
                    resourcesPresentDest += 1;
                    let newDestType = resourceTypeDest.or(resourceTypeSource);

                    if let Some(RPD) = nodeResourcesMap.get_mut(&tileLookingAt) {
                        RPD.amount = resourcesPresentDest;
                        RPD.itemType = newDestType;
                    }
                    if let Some(RPS) = nodeResourcesMap.get_mut(cellWithBuilding) {
                        RPS.amount = resourcesPresentSource;
                        RPS.itemType = resourceTypeSource;
                        if RPS.amount == 0
                            && levelMap.get(cellWithBuilding) != Some(&TileOccupant::Output)
                        {
                            RPS.itemType = None;
                        }
                    }
                    println!(
                        "source: {}, dest: {}",
                        resourcesPresentSource, resourcesPresentDest
                    );
                }
            }
            Some(TileOccupant::Player)
            | Some(TileOccupant::BasicResource)
            | Some(TileOccupant::Input)
            | None => {}
        };
        tilesSearched += 1;
    }
}
