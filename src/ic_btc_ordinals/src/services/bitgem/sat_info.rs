use super::super::{IsService, Args, Response, Function, BASE_URLS};

use crate::{types::{Provider, BitgemSatInfo, SatInfo}, utils::map_str_rarity};

use std::ops::Add;

pub struct ServiceBitgemSatInfo;

impl IsService for ServiceBitgemSatInfo {

    fn get_url(&self, args: Args) -> String {
        let ordinal = match args.function {
            Function::SatInfo{ ordinal } => ordinal,
            _ => panic!("Invalid function: SatInfo expected"),
        };
        BASE_URLS[&Provider::Bitgem]
            .clone()
            .add(
                format!("/sat/{}", ordinal)
                    .as_str()
            )
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String> {
        let bitgem_sat_info = serde_json::from_slice::<BitgemSatInfo>(bytes)
            .map_err(|err| format!("Failed to deserialize response bytes: {:?}", err))?;
        let rarities : Vec<_> = bitgem_sat_info.satributes
            .iter()
            .filter_map(|satribute| map_str_rarity(&satribute))
            .collect();
        if rarities.len() != 1 {
            return Err(format!("Cannot find rarity in Bitgem satributes: {}", bitgem_sat_info.satributes.join(",")));
        }
        Ok(Response::SatInfo(SatInfo {
            height: bitgem_sat_info.height,
            cycle: bitgem_sat_info.cycle,
            epoch: bitgem_sat_info.epoch,
            period: bitgem_sat_info.period,
            rarity: rarities[0].clone(),
        }))
    }
}

#[test]
fn test_build_request() {
    let service = ServiceBitgemSatInfo;
    let args = Args {
        function: Function::SatInfo{ ordinal: 85000000000 },
        query_options: None,
        max_kb_per_item: None,
    };
    assert_eq!(service.get_url(args.clone()), "https://api.bitgem.tech/sat/85000000000");
    assert_eq!(service.get_body(args), None);
    assert_eq!(service.get_method(), super::super::HttpMethod::GET);
}

#[test]
fn test_extract_response() {
    let service = ServiceBitgemSatInfo;
    let bytes = r#"{
        "sat":85000000000,
        "height":17,
        "cycle":0,
        "epoch":0,
        "period":0,
        "satributes":["uncommon","alpha","vintage"]
    }"#.as_bytes();
    let response = service.extract_response(bytes).unwrap();
    assert_eq!(response, Response::SatInfo(SatInfo {
        height: 17,
        cycle: 0,
        epoch: 0,
        period: 0,
        rarity: crate::types::SatoshiRarity::Uncommon,
    }));
}

