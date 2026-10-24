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

// items (type) that travel
#[derive(Debug, Clone, Copy)]
pub enum TravellerItem {
    GreenItem,
    PurpleItem,
}

// actual item that travels
#[derive(Debug, Clone, Copy)]
pub struct Traveller {
    pub itemType: TravellerItem,
    pub amount: u64,
}
