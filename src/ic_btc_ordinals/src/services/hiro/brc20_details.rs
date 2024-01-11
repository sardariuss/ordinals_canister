use super::super::{IsService, Args, Response, Function, BASE_URLS};

use crate::types::{Provider, HiroBrc20Details};
use std::ops::Add;

pub struct ServiceBrc20Details;

impl IsService for ServiceBrc20Details {

    fn get_url(&self, args: Args) -> String {
        let ticker = match args.function {
            Function::Brc20Details{ ticker } => ticker,
            _ => panic!("Invalid function: Brc20Details expected"),
        };
        BASE_URLS[&Provider::Hiro]
            .clone()
            .add(
                format!(
                    "/ordinals/v1/brc-20/tokens/{}",
                    ticker,
                )
                .as_str(),
            )
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String> {
        let brc20_details = serde_json::from_slice::<HiroBrc20Details>(bytes)
            .map_err(|err| format!("Failed to deserialize response bytes: {:?}", err))?;
        Ok(Response::Brc20Details(brc20_details))
    }
}
