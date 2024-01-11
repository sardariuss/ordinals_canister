use super::super::{IsService, Args, Response, Function, BASE_URLS};

use ic_cdk::api::management_canister::http_request::HttpMethod;

use crate::types::{Provider, BitgemSatRanges};

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

    fn get_headers(&self) -> Vec<(String, String)> {
        vec![
            ("Content-Type".to_string(), "application/json".to_string()),
        ]
    }

    fn get_method(&self) -> HttpMethod {
        HttpMethod::POST
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String> {
        let sat_ranges = serde_json::from_slice::<BitgemSatRanges>(bytes)
            .map_err(|err| format!("Failed to deserialize response bytes: {:?}", err))?;
        Ok(Response::SatRange(sat_ranges))
    }
}
