use super::super::{IsService, Args, Response, OrdFunction, BASE_URLS};

use crate::{types::{Provider, HiroBrc20Details, OrdResult, Brc20DetailsArgs}, utils::deserialize_response};
use std::ops::Add;

pub struct ServiceBrc20Details;

impl IsService for ServiceBrc20Details {

    fn get_url(&self, args: Args) -> String {
        let ticker = match args.function {
            OrdFunction::Brc20Details(Brc20DetailsArgs{ ticker }) => ticker,
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

    fn extract_response(&self, bytes: &[u8]) -> OrdResult {
        let brc20_details = deserialize_response::<HiroBrc20Details>(bytes)?;
        Ok(Response::Brc20Details(brc20_details))
    }
}

#[test]
fn test_build_request() {
    let service = ServiceBrc20Details;
    let args = Args {
        function: OrdFunction::Brc20Details( Brc20DetailsArgs{ ticker: "ordi".to_string() }),
        max_kb_per_item: None,
    };
    assert_eq!(service.get_url(args.clone()), "https://api.hiro.so/ordinals/v1/brc-20/tokens/ordi");
    assert_eq!(service.get_body(args), None);
    assert_eq!(service.get_method(), super::super::HttpMethod::GET);
}

#[test]
fn test_extract_response() {

    use crate::types::{HiroBrc20Token, HiroBrc20Supply};

    let bytes = r#"{
        "token": {
          "id": "b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735i0",
          "number": 348020,
          "block_height": 779832,
          "tx_id": "b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735",
          "address": "bc1pxaneaf3w4d27hl2y93fuft2xk6m4u3wc4rafevc6slgd7f5tq2dqyfgy06",
          "ticker": "ordi",
          "max_supply": "21000000.000000000000000000",
          "mint_limit": "1000.000000000000000000",
          "decimals": 18,
          "deploy_timestamp": 1678248991000,
          "minted_supply": "21000000.000000000000000000",
          "tx_count": 225407
        },
        "supply": {
          "max_supply": "21000000.000000000000000000",
          "minted_supply": "21000000.000000000000000000",
          "holders": 15120
        }
      }"#.as_bytes();

    let response = ServiceBrc20Details.extract_response(bytes).unwrap();
    assert_eq!(response, Response::Brc20Details(HiroBrc20Details {
        token: HiroBrc20Token {
            id: "b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735i0".to_string(),
            number: 348020,
            block_height: 779832,
            tx_id: "b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735".to_string(),
            address: "bc1pxaneaf3w4d27hl2y93fuft2xk6m4u3wc4rafevc6slgd7f5tq2dqyfgy06".to_string(),
            ticker: "ordi".to_string(),
            max_supply: "21000000.000000000000000000".to_string(),
            mint_limit: "1000.000000000000000000".to_string(),
            decimals: 18,
            deploy_timestamp: 1678248991000,
            minted_supply: "21000000.000000000000000000".to_string(),
            tx_count: 225407,
        },
        supply: HiroBrc20Supply {
            max_supply: "21000000.000000000000000000".to_string(),
            minted_supply: "21000000.000000000000000000".to_string(),
            holders: 15120,
        },
    }));
}
