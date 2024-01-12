use crate::types::SatoshiRarity;

pub fn map_str_rarity(rarity: &str) -> Option<SatoshiRarity> {
    match rarity.to_lowercase().as_str() {
        "common"    => Some(SatoshiRarity::Common),
        "uncommon"  => Some(SatoshiRarity::Uncommon),
        "rare"      => Some(SatoshiRarity::Rare),
        "epic"      => Some(SatoshiRarity::Epic),
        "legendary" => Some(SatoshiRarity::Legendary),
        "mythic"    => Some(SatoshiRarity::Mythic),
        _           => None,
    }
}