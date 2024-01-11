use candid::{Deserialize, CandidType};

use lazy_static::lazy_static;

use ic_cdk::api::management_canister::http_request::HttpMethod;

use crate::ONE_KIB;
use crate::types::{Provider, EndPoint, BitgemSatRanges, HiroSatInfo, HiroSatInscription, HiroSatInscriptions,
    InscriptionContent, HiroBrc20Details, HiroBrc20Holders, Args, Function, QueryOptions};

use std::collections::BTreeMap;
use std::vec;

mod bitgem;
mod hiro;

use bitgem::sat_range::ServiceBitgemSatRange;
use hiro::sat_info::ServiceHiroSatInfo;
use hiro::sat_inscriptions::ServiceHiroSatInscriptions;
use hiro::inscription_info::ServiceHiroInscriptionInfo;
use hiro::inscription_content::ServiceHiroInscriptionContent;
use hiro::brc20_details::ServiceBrc20Details;
use hiro::brc20_holders::ServiceBrc20Holders;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum Response {
    SatRange(BitgemSatRanges),
    SatInfo(HiroSatInfo),
    SatInscriptions(HiroSatInscriptions),
    SatInscription(HiroSatInscription),
    InscriptionContent(InscriptionContent),
    Brc20Details(HiroBrc20Details),
    Brc20Holders(HiroBrc20Holders)
}

pub enum Service {
    SatRange(ServiceBitgemSatRange),
    SatInfo(ServiceHiroSatInfo),
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
            max_kb_per_item: None,
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
            max_kb_per_item: Some(2), // @todo
        },
        Function::InscriptionContent{ inscription_id: _ } => Args {
            function,
            query_options: None,
            max_kb_per_item: Some(5), // @todo
        },
        Function::Brc20Details{ ticker: _ } => Args {
            function,
            query_options: None,
            max_kb_per_item: Some(2), // @todo
        },
        Function::Brc20Holders{ ticker: _ } => Args {
            function,
            query_options: Some(QueryOptions{ offset: 0, limit: 10 }),
            max_kb_per_item: Some(2), // @todo
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

    fn get_headers(&self) -> Vec<(String, String)> {
        vec![]
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
            Service::SatInfo           (service) => service.get_url(args),
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
            Service::SatInfo           (service) => service.get_body(args),
            Service::SatInscriptions   (service) => service.get_body(args),
            Service::InscriptionInfo   (service) => service.get_body(args),
            Service::InscriptionContent(service) => service.get_body(args),
            Service::Brc20Details      (service) => service.get_body(args),
            Service::Brc20Holders      (service) => service.get_body(args),
        }
    }

    fn get_headers(&self) -> Vec<(String, String)> {
        match self {
            Service::SatRange          (service) => service.get_headers(),
            Service::SatInfo           (service) => service.get_headers(),
            Service::SatInscriptions   (service) => service.get_headers(),
            Service::InscriptionInfo   (service) => service.get_headers(),
            Service::InscriptionContent(service) => service.get_headers(),
            Service::Brc20Details      (service) => service.get_headers(),
            Service::Brc20Holders      (service) => service.get_headers(),
        }
    }

    fn get_method(&self) -> HttpMethod {
        match self {
            Service::SatRange          (service) => service.get_method(),
            Service::SatInfo           (service) => service.get_method(),
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
            Service::SatInfo           (service) => service.extract_response(bytes),
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
        map.insert((Provider::Hiro  , EndPoint::SatInfo           ), Service::SatInfo           (ServiceHiroSatInfo           ));
        map.insert((Provider::Hiro  , EndPoint::SatInscriptions   ), Service::SatInscriptions   (ServiceHiroSatInscriptions   ));
        map.insert((Provider::Hiro  , EndPoint::InscriptionInfo   ), Service::InscriptionInfo   (ServiceHiroInscriptionInfo   ));
        map.insert((Provider::Hiro  , EndPoint::InscriptionContent), Service::InscriptionContent(ServiceHiroInscriptionContent));
        map.insert((Provider::Hiro  , EndPoint::Brc20Details      ), Service::Brc20Details      (ServiceBrc20Details          ));
        map.insert((Provider::Hiro  , EndPoint::Brc20Holders      ), Service::Brc20Holders      (ServiceBrc20Holders          ));
        map
    };
}
