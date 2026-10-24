// for anything that occpias a tile
#[derive(Debug, Clone, Copy)]
pub enum TileOccupant {
    Player,
    BasicResource,
    Input,
    Output,
    G2PFactory,
    TransmitterNode,
}
