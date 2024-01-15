use super::super::{IsService, Args, Response, Function, BASE_URLS};

use ic_cdk::api::management_canister::http_request::HttpMethod;

use crate::{types::{Provider, BitgemSatRanges, OrdResult}, utils::deserialize_response};

pub struct ServiceBitgemSatRange;

impl IsService for ServiceBitgemSatRange {

    fn get_url(&self, _: Args) -> String {
        BASE_URLS[&Provider::Bitgem].clone() + "/utxo-ranges"
    }

    fn get_body(&self, args: Args) -> Option<Vec<u8>> {
        let utxo = match args.function {
            Function::SatRange{ utxo } => utxo,
            _ => panic!("Invalid function: SatRange expected"),
        };
        let body_json = format!(r#"{{"utxos": ["{}:{}"]}}"#, utxo.txid, utxo.vout);
        Some(body_json.as_bytes().to_vec())
    }

    fn get_method(&self) -> HttpMethod {
        HttpMethod::POST
    }

    fn extract_response(&self, bytes: &[u8]) -> OrdResult {
        let sat_ranges = deserialize_response::<BitgemSatRanges>(bytes)?;
        Ok(Response::SatRange(sat_ranges))
    }
}

#[test]
fn test_build_request() {

    use crate::types::Utxo;

    let service = ServiceBitgemSatRange;
    let args = Args {
        function: Function::SatRange{ 
            utxo: Utxo { 
                txid: "3de53b46b6a2bbf38587ac3cfc055eb2e960a8d25ff1361f2f15ef2bee9168aa".to_string(), 
                vout : 0, 
            } 
        },
        query_options: None,
        max_kb_per_item: None,
    };
    assert_eq!(service.get_url(args.clone()), "https://api.bitgem.tech/utxo-ranges");
    assert_eq!(service.get_body(args), Some(r#"{"utxos": ["3de53b46b6a2bbf38587ac3cfc055eb2e960a8d25ff1361f2f15ef2bee9168aa:0"]}"#.as_bytes().to_vec()));
    assert_eq!(service.get_method(), HttpMethod::POST);
}

#[test]
fn test_extract_response() {

    use crate::types::{BitgemSatRanges, BitgemSatRange, BitgemExoticSatRange};
    
    let service = ServiceBitgemSatRange;
    let bytes = r#"{
        "ranges":[
            {
                "utxo":"1741de211d2905565bd5c07089cbae5719448fa597966e6612f76420416a0f8b:1",
                "start":1846313750000000,
                "size":2121,
                "end":1846313750002121,
                "offset":0
            }
        ],
        "exoticRanges":[
            {
                "utxo":"1741de211d2905565bd5c07089cbae5719448fa597966e6612f76420416a0f8b:1",
                "start":1846313750000000,
                "size":1,
                "end":1846313750000001,
                "offset":0,
                "satributes":["uncommon"]
            }
        ]}"#.as_bytes();
        
    let response = service.extract_response(bytes).unwrap();
    assert_eq!(response, Response::SatRange(BitgemSatRanges {
        ranges: vec![
            BitgemSatRange {
                utxo: "1741de211d2905565bd5c07089cbae5719448fa597966e6612f76420416a0f8b:1".to_string(),
                start: 1846313750000000,
                size: 2121,
                end: 1846313750002121,
                offset: 0,
            }
        ],
        exoticRanges: vec![
            BitgemExoticSatRange {
                utxo: "1741de211d2905565bd5c07089cbae5719448fa597966e6612f76420416a0f8b:1".to_string(),
                start: 1846313750000000,
                size: 1,
                end: 1846313750000001,
                offset: 0,
                satributes: vec!["uncommon".to_string()],
            }
        ],
    }));
}
