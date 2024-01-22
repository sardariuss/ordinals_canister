use super::super::{IsService, Args, Response, OrdFunction, BASE_URLS};

use ic_cdk::api::management_canister::http_request::HttpMethod;

use crate::{types::{Provider, BitgemSatRanges, ExoticSatRange, OrdResult, SatRangeArgs, SatRanges, SatoshiRarity}, utils::{deserialize_response, map_str_rarity}};

pub struct ServiceBitgemSatRange;

impl IsService for ServiceBitgemSatRange {

    fn get_url(&self, _: Args) -> String {
        BASE_URLS[&Provider::Bitgem].clone() + "/utxo-ranges"
    }

    fn get_body(&self, args: Args) -> Option<Vec<u8>> {
        let (utxos, exclude_common_ranges) = match args.function {
            OrdFunction::SatRange( SatRangeArgs { utxos, exclude_common_ranges }) => (utxos, exclude_common_ranges),
            _ => panic!("Invalid function: SatRange expected"),
        };

        // Build utxo list
        let utxo_strings: Vec<String> = utxos.iter().map(|utxo| format!(r#""{}:{}""#, utxo.txid, utxo.vout)).collect();
        let utxo_list = utxo_strings.join(",");

        let body_json = format!(r#"{{"utxos": [{}], "excludeCommonRanges": {}}}"#, utxo_list, exclude_common_ranges);
        print!("body_json: {}", body_json);
        Some(body_json.as_bytes().to_vec())
    }

    fn get_method(&self) -> HttpMethod {
        HttpMethod::POST
    }

    fn extract_response(&self, bytes: &[u8]) -> OrdResult {
        print!("response bytes: {}", String::from_utf8(bytes.to_vec()).unwrap());
        let bitgem_sat_ranges = deserialize_response::<BitgemSatRanges>(bytes)?;
        let exotic_ranges : Vec<_> = bitgem_sat_ranges.exoticRanges.unwrap_or(vec![]).iter().map(|exotic_range| {
            ExoticSatRange {
                utxo: exotic_range.utxo.clone(),
                start: exotic_range.start,
                size: exotic_range.size,
                end: exotic_range.end,
                offset: exotic_range.offset,
                satributes: exotic_range.satributes.clone(),
                rarity: exotic_range.satributes.iter().find_map(|satribute| map_str_rarity(&satribute)).unwrap_or(SatoshiRarity::Common),
            }
        }).collect();
        Ok(Response::SatRange(SatRanges{
            ranges: bitgem_sat_ranges.ranges,
            exotic_ranges: (!exotic_ranges.is_empty()).then(|| exotic_ranges),
        }))
    }
}

#[test]
fn test_build_request() {

    use crate::types::Utxo;

    let service = ServiceBitgemSatRange;
    let args = Args {
        function: OrdFunction::SatRange( SatRangeArgs{ 
            utxos: vec![
                Utxo { 
                    txid: "3de53b46b6a2bbf38587ac3cfc055eb2e960a8d25ff1361f2f15ef2bee9168aa".to_string(), 
                    vout : 0,
                },
                Utxo { 
                    txid: "3de53b46b6a2bbf38587ac3cfc055eb2e960a8d25ff1361f2f15ef2bee9168aa".to_string(), 
                    vout : 1,
                }
            ],
            exclude_common_ranges: true,
        }),
        max_kb_per_item: None,
    };
    assert_eq!(service.get_url(args.clone()), "https://api.bitgem.tech/utxo-ranges");
    assert_eq!(
        String::from_utf8(service.get_body(args).unwrap()).unwrap(), 
        r#"{"utxos": ["3de53b46b6a2bbf38587ac3cfc055eb2e960a8d25ff1361f2f15ef2bee9168aa:0","3de53b46b6a2bbf38587ac3cfc055eb2e960a8d25ff1361f2f15ef2bee9168aa:1"], "excludeCommonRanges": true}"#
    );
    assert_eq!(service.get_method(), HttpMethod::POST);
}

#[test]
fn test_extract_response_1() {
    
    let service = ServiceBitgemSatRange;
    let bytes = r#"{
        "ranges":null,
        "exotic_ranges":null
    }"#.as_bytes();
        
    let response = service.extract_response(bytes).unwrap();
    assert_eq!(response, Response::SatRange(SatRanges {
        ranges: None,
        exotic_ranges: None,
    }));
}

#[test]
fn test_extract_response_2() {

    use crate::types::SatRange;
    
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
    assert_eq!(response, Response::SatRange(SatRanges {
        ranges: Some(vec![
            SatRange {
                utxo: "1741de211d2905565bd5c07089cbae5719448fa597966e6612f76420416a0f8b:1".to_string(),
                start: 1846313750000000,
                size: 2121,
                end: 1846313750002121,
                offset: 0,
            }
        ]),
        exotic_ranges: Some(vec![
            ExoticSatRange {
                utxo: "1741de211d2905565bd5c07089cbae5719448fa597966e6612f76420416a0f8b:1".to_string(),
                start: 1846313750000000,
                size: 1,
                end: 1846313750000001,
                offset: 0,
                rarity: SatoshiRarity::Uncommon,
                satributes: vec!["uncommon".to_string()],
            }
        ]),
    }));
}

#[test]
fn test_extract_response_3() {
    
    let service = ServiceBitgemSatRange;
    let bytes = r#"{
        "ranges":null,
        "exoticRanges":[
            {
                "utxo":"0a4ae1923b59e545e82dc7067965fe02304635db665806dee76e7ead7e002d41:1",
                "start":282004319175359,
                "size":10000,
                "end":282004319185359,
                "offset":399575979,
                "satributes":["pizza"]
            },
            {
                "utxo":"0a4ae1923b59e545e82dc7067965fe02304635db665806dee76e7ead7e002d41:1",
                "start":1865887500000000,
                "size":1,
                "end":1865887500000001,
                "offset":400041360,
                "satributes":["uncommon","alpha"]
            },
            {
                "utxo":"0a4ae1923b59e545e82dc7067965fe02304635db665806dee76e7ead7e002d41:1",
                "start":1059760000000000,
                "size":1,
                "end":1059760000000001,
                "offset":506755725,
                "satributes":["uncommon","alpha"]
            }
        ]
    }"#.as_bytes();
        
    let response = service.extract_response(bytes).unwrap();
    assert_eq!(response, Response::SatRange(SatRanges {
        ranges: None,
        exotic_ranges: Some(vec![
            ExoticSatRange {
                utxo: "0a4ae1923b59e545e82dc7067965fe02304635db665806dee76e7ead7e002d41:1".to_string(),
                start: 282004319175359,
                size: 10000,
                end: 282004319185359,
                offset: 399575979,
                rarity: SatoshiRarity::Common,
                satributes: vec!["pizza".to_string()],
            },
            ExoticSatRange {
                utxo: "0a4ae1923b59e545e82dc7067965fe02304635db665806dee76e7ead7e002d41:1".to_string(),
                start: 1865887500000000,
                size: 1,
                end: 1865887500000001,
                offset: 400041360,
                rarity: SatoshiRarity::Uncommon,
                satributes: vec!["uncommon".to_string(), "alpha".to_string()],
            },
            ExoticSatRange {
                utxo: "0a4ae1923b59e545e82dc7067965fe02304635db665806dee76e7ead7e002d41:1".to_string(),
                start: 1059760000000000,
                size: 1,
                end: 1059760000000001,
                offset: 506755725,
                rarity: SatoshiRarity::Uncommon,
                satributes: vec!["uncommon".to_string(), "alpha".to_string()],
            }
        ]),
    }));
}
