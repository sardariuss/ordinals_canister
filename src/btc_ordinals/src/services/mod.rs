use lazy_static::lazy_static;

use ic_cdk::api::management_canister::http_request::HttpMethod;

use crate::ONE_KIB;
use crate::types::{Provider, EndPoint, Args, OrdFunction, Response, OrdError};

use std::collections::BTreeMap;

mod bitgem;
mod hiro;

use bitgem::sat_range::ServiceBitgemSatRange;
use bitgem::sat_info::ServiceBitgemSatInfo;
use hiro::sat_info::ServiceHiroSatInfo;
use hiro::sat_inscriptions::ServiceHiroSatInscriptions;
use hiro::inscription_info::ServiceHiroInscriptionInfo;
use hiro::inscription_content::ServiceHiroInscriptionContent;
use hiro::brc20_details::ServiceBrc20Details;
use hiro::brc20_holders::ServiceBrc20Holders;

pub fn default_args(function: OrdFunction) -> Args {
    match function.clone() {
        OrdFunction::SatRange(_) => Args {
            function,
            max_kb_per_item: Some(1),
        },
        OrdFunction::SatInfo(_) => Args {
            function,
            max_kb_per_item: Some(1), // 1 KiB should be enough for a single sat info, the size of the response body is approximatly 400 bytes
        },
        OrdFunction::SatInscriptions(_) => Args {
            function,
            max_kb_per_item: Some(2), // 2 KiB should be enough for a single inscription, the size of the response body is approximatly 1400 bytes
        },
        OrdFunction::InscriptionInfo(_) => Args {
            function,
            max_kb_per_item: Some(2), // 2 kiB (same as above)
        },
        OrdFunction::InscriptionContent(_) => Args {
            function,
            max_kb_per_item: Some(5), // 5 KiB, set arbitrarily because the size of the inscription content can vary
        },
        OrdFunction::Brc20Details(_) => Args {
            function,
            max_kb_per_item: Some(2), // 2 KiB should be enough for a single brc20 details, the size of the response body is approximatly 800 bytes
        },
        OrdFunction::Brc20Holders(_) => Args {
            function,
            max_kb_per_item: Some(1), // 1 Kib should be more than enough for a single brc20 holder, the size of the response body is approximatly 200 bytes
        },
    }
}

pub fn unwrap_max_response_bytes(args: Args) -> u64 {
    let num_items = match args.function.clone() {
        OrdFunction::SatInscriptions(args) => args.limit,
        OrdFunction::Brc20Holders(args) => args.limit,
        _ => 1,
    };
    args.max_kb_per_item.expect("Max kbyte per item is missing") * num_items * ONE_KIB as u64
}

pub trait IsService: Sync {
   
    fn get_url(&self, args: Args) -> String;
    
    fn get_body(&self, _: Args) -> Option<Vec<u8>> {
        None
    }

    fn get_method(&self) -> HttpMethod {
        HttpMethod::GET
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, OrdError>;
}

// TODO: Use OnceCell instead
lazy_static! {
    pub static ref BASE_URLS: BTreeMap<Provider, String> = {
        let mut map = BTreeMap::new();
        map.insert(Provider::Bitgem, "https://api.bitgem.tech".to_string());
        map.insert(Provider::Hiro,   "https://api.hiro.so"    .to_string());
        map
    };
}

lazy_static! {
    pub static ref SERVICES: BTreeMap<(Provider, EndPoint), std::sync::Arc<dyn IsService + Send + Sync>> = {
        let mut map : BTreeMap<(Provider, EndPoint), std::sync::Arc<dyn IsService + Send + Sync>> = BTreeMap::new();
        map.insert((Provider::Bitgem, EndPoint::SatRange          ), std::sync::Arc::new(ServiceBitgemSatRange        ));
        map.insert((Provider::Bitgem, EndPoint::SatInfo           ), std::sync::Arc::new(ServiceBitgemSatInfo         ));
        map.insert((Provider::Hiro  , EndPoint::SatInfo           ), std::sync::Arc::new(ServiceHiroSatInfo           ));
        map.insert((Provider::Hiro  , EndPoint::SatInscriptions   ), std::sync::Arc::new(ServiceHiroSatInscriptions   ));
        map.insert((Provider::Hiro  , EndPoint::InscriptionInfo   ), std::sync::Arc::new(ServiceHiroInscriptionInfo   ));
        map.insert((Provider::Hiro  , EndPoint::InscriptionContent), std::sync::Arc::new(ServiceHiroInscriptionContent));
        map.insert((Provider::Hiro  , EndPoint::Brc20Details      ), std::sync::Arc::new(ServiceBrc20Details          ));
        map.insert((Provider::Hiro  , EndPoint::Brc20Holders      ), std::sync::Arc::new(ServiceBrc20Holders          ));
        map
    };
}

// Return the end point associated with the given ord function
pub fn deduce_end_point(function: OrdFunction) -> EndPoint {
    match function {
        OrdFunction::SatRange(_)           => EndPoint::SatRange,
        OrdFunction::SatInfo(_)            => EndPoint::SatInfo,
        OrdFunction::SatInscriptions(_)    => EndPoint::SatInscriptions,
        OrdFunction::InscriptionInfo(_)    => EndPoint::InscriptionInfo,
        OrdFunction::InscriptionContent(_) => EndPoint::InscriptionContent,
        OrdFunction::Brc20Details(_)       => EndPoint::Brc20Details,
        OrdFunction::Brc20Holders(_)       => EndPoint::Brc20Holders,
    }
}

// Check that the given providers are available
// If the required providers argument is empty, all available services are returned
// Otherwise, if the required providers are all available, return them
// Otherwise, return an error with the providers that are not available
pub fn validate_providers(required: Vec<Provider>, end_point: EndPoint) -> Result<Vec<Provider>, Vec<Provider>> {
    let available: Vec<Provider> = SERVICES
        .iter()
        .filter(|(key, _)| key.1 == end_point)
        .map(|(key, _)| key.0.clone())
        .collect();
    if required.is_empty() {
        Ok(available)
    } else {
        let missing : Vec<_> = required
            .iter()
            .filter(|&element| !available.contains(element))
            .cloned()
            .collect();
        if missing.is_empty() {
            Ok(required)
        } else {
            Err(missing)
        }
    }
}
