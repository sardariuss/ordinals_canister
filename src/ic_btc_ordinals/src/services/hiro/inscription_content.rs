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

#[test]
fn test_build_request() {
    let service = ServiceHiroInscriptionContent;
    let args = Args {
        function: Function::InscriptionContent{ inscription_id: "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dci0".to_string() },
        query_options: None,
        max_kb_per_item: None,
    };
    assert_eq!(service.get_url(args.clone()), "https://api.hiro.so/ordinals/v1/inscriptions/38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dci0/content");
    assert_eq!(service.get_body(args), None);
    assert_eq!(service.get_headers(), vec![]);
    assert_eq!(service.get_method(), super::super::HttpMethod::GET);
}

#[test]
fn test_extract_response() {
    let bytes = r#"whatever"#.as_bytes();
    let response = ServiceHiroInscriptionContent.extract_response(bytes).unwrap();
    assert_eq!(response, Response::InscriptionContent(bytes.to_vec()));
}

