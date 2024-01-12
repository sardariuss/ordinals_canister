use super::super::{IsService, Args, Response, Function, unwrap_query_options, BASE_URLS};

use crate::types::{Provider, HiroSatInscriptions};
use std::ops::Add;

pub struct ServiceHiroSatInscriptions;

impl IsService for ServiceHiroSatInscriptions {

    fn get_url(&self, args: Args) -> String {
        let ordinal = match args.clone().function {
            Function::SatInscriptions{ ordinal } => ordinal,
            _ => panic!("Invalid function: SatInscription expected"),
        };
        let query_options = unwrap_query_options(args);
        BASE_URLS[&Provider::Hiro]
            .clone()
            .add(
                format!(
                    "/ordinals/v1/sats/{}/inscriptions?offset={}&limit={}",
                    ordinal,
                    query_options.offset,
                    query_options.limit
                )
                .as_str(),
            )
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String> {
        let sat_inscriptions = serde_json::from_slice::<HiroSatInscriptions>(bytes)
            .map_err(|err| format!("Failed to deserialize response bytes: {:?}", err))?;
        Ok(Response::SatInscriptions(sat_inscriptions))
    }
}

#[test]
fn test_build_request() {

    use crate::types::QueryOptions;

    let service = ServiceHiroSatInscriptions;
    let args = Args {
        function: Function::SatInscriptions{ ordinal: 947410401228752 },
        query_options: Some( QueryOptions{ offset: 0, limit: 2 }),
        max_kb_per_item: None,
    };
    assert_eq!(service.get_url(args.clone()), "https://api.hiro.so/ordinals/v1/sats/947410401228752/inscriptions?offset=0&limit=2");
    assert_eq!(service.get_body(args), None);
    assert_eq!(service.get_headers(), vec![]);
    assert_eq!(service.get_method(), super::super::HttpMethod::GET);
}

#[test]
fn test_extract_response() {

    use crate::types::{HiroSatInscriptions, HiroSatInscription};

    let bytes = r#"{
        "limit": 2,
        "offset": 0,
        "total": 11,
        "results": [
          {
            "id": "5381836216f93e0dba4e0100fe1720ca098c62ac3ff2f229605ff2e0f16bf220i0",
            "number": 169603,
            "address": "bc1pkctwmtz5llxuu466htg2dppj4lm29wnx28n2cxwwwm9xquzres9qqshvse",
            "genesis_address": "bc1pmr9hfh7t43fh7829vmdgv0pjqaxjzme5rlgv4wkcn8u6wtt5rt2qvhkxwr",
            "genesis_block_height": 778053,
            "genesis_block_hash": "000000000000000000068bf8f0f9ded6550586eaab578c9d12263fae8683b472",
            "genesis_tx_id": "5381836216f93e0dba4e0100fe1720ca098c62ac3ff2f229605ff2e0f16bf220",
            "genesis_fee": "37844",
            "genesis_timestamp": 1677210155000,
            "tx_id": "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4",
            "location": "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4:0:0",
            "output": "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4:0",
            "value": "546",
            "offset": "0",
            "sat_ordinal": "947410401228752",
            "sat_rarity": "common",
            "sat_coinbase_height": 189482,
            "mime_type": "image/jpeg",
            "content_type": "image/jpeg",
            "content_length": 74718,
            "timestamp": 1704398076000,
            "curse_type": null,
            "recursive": false,
            "recursion_refs": null
          },
          {
            "id": "3623b227518317585ae1a0fceb2ac8bf7358208b531fbe375120a8ce96a04e17i0",
            "number": -104657,
            "address": "bc1pkctwmtz5llxuu466htg2dppj4lm29wnx28n2cxwwwm9xquzres9qqshvse",
            "genesis_address": "bc1qc27fetxkxjnd45a6eaw45ujh602w8harew4cs6",
            "genesis_block_height": 805807,
            "genesis_block_hash": "0000000000000000000475e266695760bbe64c8fd6379b4b7d4d08844d6a7654",
            "genesis_tx_id": "3623b227518317585ae1a0fceb2ac8bf7358208b531fbe375120a8ce96a04e17",
            "genesis_fee": "111960",
            "genesis_timestamp": 1693616765000,
            "tx_id": "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4",
            "location": "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4:0:0",
            "output": "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4:0",
            "value": "546",
            "offset": "0",
            "sat_ordinal": "947410401228752",
            "sat_rarity": "common",
            "sat_coinbase_height": 189482,
            "mime_type": "image/webp",
            "content_type": "image/webp",
            "content_length": 29196,
            "timestamp": 1704398076000,
            "curse_type": null,
            "recursive": false,
            "recursion_refs": null
          }
        ]
      }"#.as_bytes();
    
    let response = ServiceHiroSatInscriptions.extract_response(bytes).unwrap();
    assert_eq!(response, Response::SatInscriptions(HiroSatInscriptions {
        limit: 2,
        offset: 0,
        total: 11,
        results: vec![
            HiroSatInscription {
                id: "5381836216f93e0dba4e0100fe1720ca098c62ac3ff2f229605ff2e0f16bf220i0".to_string(),
                number: 169603,
                address: "bc1pkctwmtz5llxuu466htg2dppj4lm29wnx28n2cxwwwm9xquzres9qqshvse".to_string(),
                genesis_address: "bc1pmr9hfh7t43fh7829vmdgv0pjqaxjzme5rlgv4wkcn8u6wtt5rt2qvhkxwr".to_string(),
                genesis_block_height: 778053,
                genesis_block_hash: "000000000000000000068bf8f0f9ded6550586eaab578c9d12263fae8683b472".to_string(),
                genesis_tx_id: "5381836216f93e0dba4e0100fe1720ca098c62ac3ff2f229605ff2e0f16bf220".to_string(),
                genesis_fee: "37844".to_string(),
                genesis_timestamp: 1677210155000,
                tx_id: "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4".to_string(),
                location: "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4:0:0".to_string(),
                output: "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4:0".to_string(),
                value: "546".to_string(),
                offset: "0".to_string(),
                sat_ordinal: "947410401228752".to_string(),
                sat_rarity: "common".to_string(),
                sat_coinbase_height: 189482,
                mime_type: "image/jpeg".to_string(),
                content_type: "image/jpeg".to_string(),
                content_length: 74718,
                timestamp: 1704398076000,
                curse_type: None,
                recursive: false,
                recursion_refs: None,
            },
            HiroSatInscription {
                id: "3623b227518317585ae1a0fceb2ac8bf7358208b531fbe375120a8ce96a04e17i0".to_string(),
                number: -104657,
                address: "bc1pkctwmtz5llxuu466htg2dppj4lm29wnx28n2cxwwwm9xquzres9qqshvse".to_string(),
                genesis_address: "bc1qc27fetxkxjnd45a6eaw45ujh602w8harew4cs6".to_string(),
                genesis_block_height: 805807,
                genesis_block_hash: "0000000000000000000475e266695760bbe64c8fd6379b4b7d4d08844d6a7654".to_string(),
                genesis_tx_id: "3623b227518317585ae1a0fceb2ac8bf7358208b531fbe375120a8ce96a04e17".to_string(),
                genesis_fee: "111960".to_string(),
                genesis_timestamp: 1693616765000,
                tx_id: "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4".to_string(),
                location: "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4:0:0".to_string(),
                output: "5b868359ab92e242c0be0cb1a12bd7bb5450a004d8137a462cd4d171ba10c6c4:0".to_string(),
                value: "546".to_string(),
                offset: "0".to_string(),
                sat_ordinal: "947410401228752".to_string(),
                sat_rarity: "common".to_string(),
                sat_coinbase_height: 189482,
                mime_type: "image/webp".to_string(),
                content_type: "image/webp".to_string(),
                content_length: 29196,
                timestamp: 1704398076000,
                curse_type: None,
                recursive: false,
                recursion_refs: None,
            },
        ],
    }));
}