use super::super::{IsService, Args, Response, Function, BASE_URLS};

use crate::types::{Provider, HiroSatInscription};
use std::ops::Add;

pub struct ServiceHiroInscriptionInfo;

impl IsService for ServiceHiroInscriptionInfo {

    fn get_url(&self, args: Args) -> String {
        let inscriptions_id = match args.function {
            Function::InscriptionInfo{ inscription_id } => inscription_id,
            _ => panic!("Invalid function: InscriptionInfo expected"),
        };
        BASE_URLS[&Provider::Hiro]
            .clone()
            .add(
                format!(
                    "/ordinals/v1/inscriptions/{}",
                    inscriptions_id,
                )
                .as_str(),
            )
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String> {
        let sat_inscriptions = serde_json::from_slice::<HiroSatInscription>(bytes)
            .map_err(|err| format!("Failed to deserialize response bytes: {:?}", err))?;
        Ok(Response::InscriptionInfo(sat_inscriptions))
    }
}

#[test]
fn test_build_request() {
    let service: ServiceHiroInscriptionInfo = ServiceHiroInscriptionInfo;
    let args = Args {
        function: Function::InscriptionInfo{ inscription_id: "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dci0".to_string() },
        query_options: None,
        max_kb_per_item: None,
    };
    assert_eq!(service.get_url(args.clone()), "https://api.hiro.so/ordinals/v1/inscriptions/38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dci0");
    assert_eq!(service.get_body(args), None);
    assert_eq!(service.get_headers(), vec![]);
    assert_eq!(service.get_method(), super::super::HttpMethod::GET);
}

#[test]
fn test_extract_response() {
    
    use crate::types::HiroSatInscription;

    let bytes = r#"{
        "id": "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dci0",
        "number": 19095,
        "address": "bc1p3cyx5e2hgh53w7kpxcvm8s4kkega9gv5wfw7c4qxsvxl0u8x834qf0u2td",
        "genesis_address": "bc1p3cyx5e2hgh53w7kpxcvm8s4kkega9gv5wfw7c4qxsvxl0u8x834qf0u2td",
        "genesis_block_height": 775617,
        "genesis_block_hash": "00000000000000000003e4523d5f3008bbf4deeaf8b6acca345241bfa9097d75",
        "genesis_tx_id": "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dc",
        "genesis_fee": "2805",
        "genesis_timestamp": 1675884508000,
        "tx_id": "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dc",
        "location": "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dc:0:0",
        "output": "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dc:0",
        "value": "10000",
        "offset": "0",
        "sat_ordinal": "257418248345364",
        "sat_rarity": "common",
        "sat_coinbase_height": 51483,
        "mime_type": "image/png",
        "content_type": "image/png",
        "content_length": 208,
        "timestamp": 1675884508000,
        "curse_type": null,
        "recursive": false,
        "recursion_refs": null
      }"#.as_bytes();
    
    let response = ServiceHiroInscriptionInfo.extract_response(bytes).unwrap();
    assert_eq!(response, Response::InscriptionInfo(HiroSatInscription {
        id: "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dci0".to_string(),
        number: 19095,
        address: "bc1p3cyx5e2hgh53w7kpxcvm8s4kkega9gv5wfw7c4qxsvxl0u8x834qf0u2td".to_string(),
        genesis_address: "bc1p3cyx5e2hgh53w7kpxcvm8s4kkega9gv5wfw7c4qxsvxl0u8x834qf0u2td".to_string(),
        genesis_block_height: 775617,
        genesis_block_hash: "00000000000000000003e4523d5f3008bbf4deeaf8b6acca345241bfa9097d75".to_string(),
        genesis_tx_id: "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dc".to_string(),
        genesis_fee: "2805".to_string(),
        genesis_timestamp: 1675884508000,
        tx_id: "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dc".to_string(),
        location: "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dc:0:0".to_string(),
        output: "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dc:0".to_string(),
        value: "10000".to_string(),
        offset: "0".to_string(),
        sat_ordinal: "257418248345364".to_string(),
        sat_rarity: "common".to_string(),
        sat_coinbase_height: 51483,
        mime_type: "image/png".to_string(),
        content_type: "image/png".to_string(),
        content_length: 208,
        timestamp: 1675884508000,
        curse_type: None,
        recursive: false,
        recursion_refs: None,
    }));
}