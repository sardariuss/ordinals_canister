use super::super::{IsService, Args, Response, OrdFunction, BASE_URLS};

use crate::{types::{Provider, HiroBrc20Holders, OrdResult, Brc20HoldersArgs}, utils::deserialize_response};
use std::ops::Add;

pub struct ServiceBrc20Holders;

impl IsService for ServiceBrc20Holders {

    fn get_url(&self, args: Args) -> String {
        let (ticker, limit, offset) = match args.clone().function {
            OrdFunction::Brc20Holders(Brc20HoldersArgs{ ticker, limit, offset }) => (ticker, limit, offset),
            _ => panic!("Invalid function: Brc20Holders expected"),
        };
        BASE_URLS[&Provider::Hiro]
            .clone()
            .add(
                format!(
                    "/ordinals/v1/brc-20/tokens/{}/holders?offset={}&limit={}",
                    ticker,
                    offset,
                    limit
                )
                .as_str(),
            )
    }

    fn extract_response(&self, bytes: &[u8]) -> OrdResult {
        let brc20_details = deserialize_response::<HiroBrc20Holders>(bytes)?;
        Ok(Response::Brc20Holders(brc20_details))
    }
}

#[test]
fn test_build_request() {

    let service = ServiceBrc20Holders;
    let args = Args {
        function: OrdFunction::Brc20Holders(Brc20HoldersArgs{ ticker: "ordi".to_string(), offset: 2, limit: 5 }),
        max_kb_per_item: None,
    };
    assert_eq!(service.get_url(args.clone()), "https://api.hiro.so/ordinals/v1/brc-20/tokens/ordi/holders?offset=2&limit=5");
    assert_eq!(service.get_body(args), None);
    assert_eq!(service.get_method(), super::super::HttpMethod::GET);
}

#[test]
fn test_extract_response() {
    
    use crate::types::HiroBrc20Holder;

    let bytes = r#"{
        "limit": 5,
        "offset": 2,
        "total": 34482,
        "results": [
          {
            "address": "bc1qqd72vtqlw0nugqmzrx398x8gj03z8aqr79aexrncezqaw74dtu4qxjydq3",
            "overall_balance": "989780.514209670000000000"
          },
          {
            "address": "bc1qz7rw2atrt3e8jrywva2y8xmka8lewalx8qazlxaq8xkn2xke0yyqvpel3e",
            "overall_balance": "650111.636402850000000000"
          },
          {
            "address": "bc1q8u9thhxvkjw9t8tf0sj6k0vwmk7jstc9z0f3at0r5xunxxp9f0pqmetg7x",
            "overall_balance": "612586.442638590000000000"
          },
          {
            "address": "bc1qnw79hhts8r84gykqkctyhu3j4gckll9gqxktzqgx5a54m347zf7qxhcyn8",
            "overall_balance": "509725.062426550000000000"
          },
          {
            "address": "bc1qm07w8kvcyst7wtv3spnxj07gnxy9cxffmzsczl9vsnzxu54cx90s90knnz",
            "overall_balance": "409034.000000000000000000"
          }
        ]
      }"#.as_bytes();
    
    let response = ServiceBrc20Holders.extract_response(bytes).unwrap();
    assert_eq!(response, Response::Brc20Holders(HiroBrc20Holders {
        limit: 5,
        offset: 2,
        total: 34482,
        results: vec![
            HiroBrc20Holder {
                address: "bc1qqd72vtqlw0nugqmzrx398x8gj03z8aqr79aexrncezqaw74dtu4qxjydq3".to_string(),
                overall_balance: "989780.514209670000000000".to_string(),
            },
            HiroBrc20Holder {
                address: "bc1qz7rw2atrt3e8jrywva2y8xmka8lewalx8qazlxaq8xkn2xke0yyqvpel3e".to_string(),
                overall_balance: "650111.636402850000000000".to_string(),
            },
            HiroBrc20Holder {
                address: "bc1q8u9thhxvkjw9t8tf0sj6k0vwmk7jstc9z0f3at0r5xunxxp9f0pqmetg7x".to_string(),
                overall_balance: "612586.442638590000000000".to_string(),
            },
            HiroBrc20Holder {
                address: "bc1qnw79hhts8r84gykqkctyhu3j4gckll9gqxktzqgx5a54m347zf7qxhcyn8".to_string(),
                overall_balance: "509725.062426550000000000".to_string(),
            },
            HiroBrc20Holder {
                address: "bc1qm07w8kvcyst7wtv3spnxj07gnxy9cxffmzsczl9vsnzxu54cx90s90knnz".to_string(),
                overall_balance: "409034.000000000000000000".to_string(),
            },
        ],
    }));
}
