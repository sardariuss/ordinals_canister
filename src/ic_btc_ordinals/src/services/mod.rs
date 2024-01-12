use lazy_static::lazy_static;

use ic_cdk::api::management_canister::http_request::HttpMethod;

use crate::ONE_KIB;
use crate::types::{Provider, EndPoint, Args, Function, Response, QueryOptions};

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

pub enum Service {
    SatRange(ServiceBitgemSatRange),
    BitgemSatInfo(ServiceBitgemSatInfo),
    HiroSatInfo(ServiceHiroSatInfo),
    SatInscriptions(ServiceHiroSatInscriptions),
    InscriptionInfo(ServiceHiroInscriptionInfo),
    InscriptionContent(ServiceHiroInscriptionContent),
    Brc20Details(ServiceBrc20Details),
    Brc20Holders(ServiceBrc20Holders),
}

pub fn default_args(function: Function) -> Args {
    match function.clone() {
        Function::SatRange{ utxo: _ } => Args {
            function,
            query_options: None,
            max_kb_per_item: Some(1),
        },
        Function::SatInfo{ ordinal: _ } => Args {
            function,
            query_options: None,
            max_kb_per_item: Some(1), // 1 KiB should be enough for a single sat info, the size of the response body is approximatly 400 bytes
        },
        Function::SatInscriptions{ ordinal: _ } => Args {
            function,
            query_options: Some(QueryOptions{ offset: 0, limit: 10 }),
            max_kb_per_item: Some(2), // 2 KiB should be enough for a single inscription, the size of the response body is approximatly 1400 bytes
        },
        Function::InscriptionInfo{ inscription_id: _ } => Args {
            function,
            query_options: None,
            max_kb_per_item: Some(2), // 2 kiB (same as above)
        },
        Function::InscriptionContent{ inscription_id: _ } => Args {
            function,
            query_options: None,
            max_kb_per_item: Some(5), // 5 KiB, set arbitrarily because the size of the inscription content can vary
        },
        Function::Brc20Details{ ticker: _ } => Args {
            function,
            query_options: None,
            max_kb_per_item: Some(2), // 2 KiB should be enough for a single brc20 details, the size of the response body is approximatly 800 bytes
        },
        Function::Brc20Holders{ ticker: _ } => Args {
            function,
            query_options: Some(QueryOptions{ offset: 0, limit: 10 }),
            max_kb_per_item: Some(1), // 1 Kib should be more than enough for a single brc20 holder, the size of the response body is approximatly 200 bytes
        },
    }
}

pub fn unwrap_query_options(args: Args) -> QueryOptions {
    args.query_options.expect("Query options are missing")
}

pub fn unwrap_max_response_bytes(args: Args) -> u64 {
    let number_items = args.query_options.unwrap_or(QueryOptions{ offset: 0, limit: 1 }).limit;
    args.max_kb_per_item.expect("Max kbyte per item is missing") * number_items * ONE_KIB as u64
}

pub trait IsService {
   
    fn get_url(&self, args: Args) -> String;
    
    fn get_body(&self, _: Args) -> Option<Vec<u8>> {
        None
    }

    fn get_method(&self) -> HttpMethod {
        HttpMethod::GET
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String>;
}

impl IsService for Service {
    
    fn get_url(&self, args: Args) -> String {
        match self {
            Service::SatRange          (service) => service.get_url(args),
            Service::BitgemSatInfo     (service) => service.get_url(args),
            Service::HiroSatInfo       (service) => service.get_url(args),
            Service::SatInscriptions   (service) => service.get_url(args),
            Service::InscriptionInfo   (service) => service.get_url(args),
            Service::InscriptionContent(service) => service.get_url(args),
            Service::Brc20Details      (service) => service.get_url(args),
            Service::Brc20Holders      (service) => service.get_url(args),
        }
    }

    fn get_body(&self, args: Args) -> Option<Vec<u8>> {
        match self {
            Service::SatRange          (service) => service.get_body(args),
            Service::BitgemSatInfo     (service) => service.get_body(args),
            Service::HiroSatInfo       (service) => service.get_body(args),
            Service::SatInscriptions   (service) => service.get_body(args),
            Service::InscriptionInfo   (service) => service.get_body(args),
            Service::InscriptionContent(service) => service.get_body(args),
            Service::Brc20Details      (service) => service.get_body(args),
            Service::Brc20Holders      (service) => service.get_body(args),
        }
    }

    fn get_method(&self) -> HttpMethod {
        match self {
            Service::SatRange          (service) => service.get_method(),
            Service::BitgemSatInfo     (service) => service.get_method(),
            Service::HiroSatInfo       (service) => service.get_method(),
            Service::SatInscriptions   (service) => service.get_method(),
            Service::InscriptionInfo   (service) => service.get_method(),
            Service::InscriptionContent(service) => service.get_method(),
            Service::Brc20Details      (service) => service.get_method(),
            Service::Brc20Holders      (service) => service.get_method(),
        }
    }

    fn extract_response(&self, bytes: &[u8]) -> Result<Response, String> {
        match self {
            Service::SatRange          (service) => service.extract_response(bytes),
            Service::BitgemSatInfo     (service) => service.extract_response(bytes),
            Service::HiroSatInfo       (service) => service.extract_response(bytes),
            Service::SatInscriptions   (service) => service.extract_response(bytes),
            Service::InscriptionInfo   (service) => service.extract_response(bytes),
            Service::InscriptionContent(service) => service.extract_response(bytes),
            Service::Brc20Details      (service) => service.extract_response(bytes),
            Service::Brc20Holders      (service) => service.extract_response(bytes),
        }
    }
}

lazy_static! {
    pub static ref BASE_URLS: BTreeMap<Provider, String> = {
        let mut map = BTreeMap::new();
        map.insert(Provider::Bitgem, "https://api.bitgem.tech".to_string());
        map.insert(Provider::Hiro,   "https://api.hiro.so"    .to_string());
        map
    };
}

lazy_static! {
    pub static ref SERVICES: BTreeMap<(Provider, EndPoint), Service> = {
        let mut map = BTreeMap::new();
        map.insert((Provider::Bitgem, EndPoint::SatRange          ), Service::SatRange          (ServiceBitgemSatRange        ));
        map.insert((Provider::Bitgem, EndPoint::SatInfo           ), Service::BitgemSatInfo     (ServiceBitgemSatInfo         ));
        map.insert((Provider::Hiro  , EndPoint::SatInfo           ), Service::HiroSatInfo       (ServiceHiroSatInfo           ));
        map.insert((Provider::Hiro  , EndPoint::SatInscriptions   ), Service::SatInscriptions   (ServiceHiroSatInscriptions   ));
        map.insert((Provider::Hiro  , EndPoint::InscriptionInfo   ), Service::InscriptionInfo   (ServiceHiroInscriptionInfo   ));
        map.insert((Provider::Hiro  , EndPoint::InscriptionContent), Service::InscriptionContent(ServiceHiroInscriptionContent));
        map.insert((Provider::Hiro  , EndPoint::Brc20Details      ), Service::Brc20Details      (ServiceBrc20Details          ));
        map.insert((Provider::Hiro  , EndPoint::Brc20Holders      ), Service::Brc20Holders      (ServiceBrc20Holders          ));
        map
    };
}

// Return the services that are available for the given function
pub fn get_available(function: Function) -> Vec<(Provider, EndPoint)> {
    let end_point = match function {
        Function::SatRange{ utxo: _ }                     => EndPoint::SatRange,
        Function::SatInfo{ ordinal: _ }                   => EndPoint::SatInfo,
        Function::SatInscriptions{ ordinal: _ }           => EndPoint::SatInscriptions,
        Function::InscriptionInfo{ inscription_id: _ }    => EndPoint::InscriptionInfo,
        Function::InscriptionContent{ inscription_id: _ } => EndPoint::InscriptionContent,
        Function::Brc20Details{ ticker: _ }               => EndPoint::Brc20Details,
        Function::Brc20Holders{ ticker: _ }               => EndPoint::Brc20Holders,
    };
    SERVICES
        .iter()
        .filter(|(key, _)| key.1 == end_point)
        .map(|(key, _)| key.clone())
        .collect()
}
