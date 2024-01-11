use super::super::{IsService, Args, Response, Function, BASE_URLS};

use crate::types::Provider;
use std::ops::Add;

pub struct ServiceHiroInscriptionContent;

impl IsService for ServiceHiroInscriptionContent {

    fn get_url(&self, args: Args) -> String {
        let inscription_id = match args.function {
            Function::InscriptionContent{ inscription_id } => inscription_id,
            _ => panic!("Invalid function: InscriptionContent expected"),
        };
        BASE_URLS[&Provider::Hiro]
            .clone()
            .add(
                format!(
                    "/ordinals/v1/inscriptions/{}/content",
                    inscription_id,
                )
                .as_str(),
            )
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String> {
        Ok(Response::InscriptionContent(bytes.to_vec()))
    }
}
