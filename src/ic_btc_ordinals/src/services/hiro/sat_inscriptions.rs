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
