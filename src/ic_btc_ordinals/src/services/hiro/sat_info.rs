use super::super::{IsService, Args, Response, Function, BASE_URLS};

use crate::types::{Provider, HiroSatInfo};

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
        let ordinal_info = serde_json::from_slice::<HiroSatInfo>(bytes)
            .map_err(|err| format!("Failed to deserialize response bytes: {:?}", err))?;
        Ok(Response::SatInfo(ordinal_info))
    }
}

