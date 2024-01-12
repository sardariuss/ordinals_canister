use super::super::{IsService, Args, Response, Function, BASE_URLS};

use crate::{types::{Provider, HiroSatInfo, SatInfo}, utils::map_str_rarity};

use std::ops::Add;

pub struct ServiceHiroSatInfo;

impl IsService for ServiceHiroSatInfo {

    fn get_url(&self, args: Args) -> String {
        let ordinal = match args.function {
            Function::SatInfo{ ordinal } => ordinal,
            _ => panic!("Invalid function: SatInfo expected"),
        };
        BASE_URLS[&Provider::Hiro]
            .clone()
            .add(
                format!("/ordinals/v1/sats/{}", ordinal)
                    .as_str()
            )
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String> {
        let hiro_sat_info = serde_json::from_slice::<HiroSatInfo>(bytes)
            .map_err(|err| format!("Failed to deserialize response bytes: {:?}", err))?;
        let rarity = map_str_rarity(&hiro_sat_info.rarity)
            .ok_or(format!("Invalid rarity: {}", hiro_sat_info.rarity))?;
        Ok(Response::SatInfo(SatInfo {
            height: hiro_sat_info.coinbase_height,
            cycle: hiro_sat_info.cycle,
            epoch: hiro_sat_info.epoch,
            period: hiro_sat_info.period,
            rarity: rarity,
        }))
    }
}

#[test]
fn test_build_request() {
    let service = ServiceHiroSatInfo;
    let args = Args {
        function: Function::SatInfo{ ordinal: 85000000000 },
        query_options: None,
        max_kb_per_item: None,
    };
    assert_eq!(service.get_url(args.clone()), "https://api.hiro.so/ordinals/v1/sats/85000000000");
    assert_eq!(service.get_body(args), None);
    assert_eq!(service.get_method(), super::super::HttpMethod::GET);
}

#[test]
fn test_extract_response() {

    use crate::types::SatoshiRarity;

    let bytes = r#"{
        "coinbase_height": 17,
        "cycle": 0,
        "decimal": "17.0",
        "degree": "0°17′17″0‴",
        "epoch": 0,
        "name": "nvsstftmsmj",
        "offset": 0,
        "percentile": "0.004047619052071431%",
        "period": 0,
        "rarity": "uncommon"
      }"#.as_bytes();
    
    let response = ServiceHiroSatInfo.extract_response(bytes).unwrap();
    assert_eq!(response, Response::SatInfo(SatInfo {
        height: 17,
        cycle: 0,
        epoch: 0,
        period: 0,
        rarity: SatoshiRarity::Uncommon,
    }));
}

