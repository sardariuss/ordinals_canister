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
        Ok(Response::SatInscription(sat_inscriptions))
    }
}
