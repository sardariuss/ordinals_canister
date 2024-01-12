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

#[test]
fn test_build_request() {

    use crate::types::QueryOptions;

    let service = ServiceBrc20Holders;
    let args = Args {
        function: Function::Brc20Holders{ ticker: "ordi".to_string() },
        query_options: Some( QueryOptions{ offset: 2, limit: 5 }),
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
