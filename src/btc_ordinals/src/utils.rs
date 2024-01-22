use crate::types::{ SatoshiRarity, JsonError, OrdError, OrdArgs, Args };

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


pub fn deserialize_response<'a, T>(
    bytes: &'a [u8]
) 
-> Result<T, OrdError>
where
    T: candid::Deserialize<'a>, 
{
    match serde_json::from_slice::<T>(bytes) {
        Ok(res) => {
            Ok(res)
        }
        Err(_) => {
            match serde_json::from_slice::<JsonError>(bytes) {
                Ok(json_err) => {
                    Err(OrdError::ResponseError(json_err.error))
                }
                Err(_) => {
                    Err(OrdError::ResponseDecodingError(format!("Failed to deserialize response bytes: {:?}", bytes.to_vec())))
                }
            }
        }
    }
}

pub fn from_ord_args(ord_args: OrdArgs) -> Args {
    Args {
        function: ord_args.function,
        max_kb_per_item: ord_args.max_kb_per_item,
    }
}
