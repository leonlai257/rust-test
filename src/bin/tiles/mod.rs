enum MahjongKey {
    Bamboo,
    Dot,
    Million,
    Character,
    Flower,
    Season,
}

pub struct MahjongTile {
    key: MahjongKey,
    number: u8,
}

pub fn get_tiles() -> MahjongTile {
    let mut result = MahjongTile {
        key: MahjongKey::Bamboo,
        number: 1,
    };
    result
}
