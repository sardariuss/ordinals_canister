use super::super::{IsService, Args, Response, Function, unwrap_query_options, BASE_URLS};

use crate::types::{Provider, HiroBrc20Holders};
use std::ops::Add;

pub struct ServiceBrc20Holders;

impl IsService for ServiceBrc20Holders {

    fn get_url(&self, args: Args) -> String {
        let ticker = match args.clone().function {
            Function::Brc20Holders{ ticker } => ticker,
            _ => panic!("Invalid function: Brc20Holders expected"),
        };
        let query_options = unwrap_query_options(args);
        BASE_URLS[&Provider::Hiro]
            .clone()
            .add(
                format!(
                    "/ordinals/v1/brc-20/tokens/{}/holders?offset={}&limit={}",
                    ticker,
                    query_options.offset,
                    query_options.limit
                )
                .as_str(),
            )
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String> {
        let brc20_details = serde_json::from_slice::<HiroBrc20Holders>(bytes)
            .map_err(|err| format!("Failed to deserialize response bytes: {:?}", err))?;
        Ok(Response::Brc20Holders(brc20_details))
    }
}
