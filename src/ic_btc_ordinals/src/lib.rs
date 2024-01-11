mod http;
mod types;
mod services;
mod utils;

use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};

use services::{SERVICES, default_args, unwrap_max_response_bytes, Response};
use types::{Utxo, BitgemSatRanges, HiroSatInfo, HiroSatInscription, HiroSatInscriptions, Provider, Function, Args,
    EndPoint, HiroBrc20Details, HiroBrc20Holders};

use crate::http::CanisterHttpRequest;
use crate::services::IsService;

/// Used for setting the max response bytes.
const ONE_KIB: u64 = 1_024;

/// Cycles cost constants, based on
/// https://internetcomputer.org/docs/current/developer-docs/gas-cost#details-cost-of-compute-and-storage-transactions-on-the-internet-computer
pub const INGRESS_OVERHEAD_BYTES: u128 = 100;
pub const INGRESS_MESSAGE_RECEIVED_COST: u128 = 1_200_000;
pub const INGRESS_MESSAGE_BYTE_RECEIVED_COST: u128 = 2_000;
pub const HTTP_OUTCALL_REQUEST_COST: u128 = 49_140_000; // Used to be 400_000_000
pub const HTTP_OUTCALL_BYTE_RECEIVED_COST: u128 = 10_400; // Used to be 100_000

#[ic_cdk::update]
async fn request(args: Args) -> Result<Response, String> {
    
    // @todo: provider and endpoint shall not be hardcoded but deduced from the args
    // @todo: should return a MultipleResult
    call_service(Provider::Bitgem, EndPoint::SatRange, args).await
}

#[ic_cdk::update]
async fn sat_range(utxo: Utxo) -> Result<BitgemSatRanges, String> {

    call_service(Provider::Bitgem, EndPoint::SatRange, default_args(Function::SatRange { utxo })).await.map(|response| {
        match response {
            Response::SatRange(sat_ranges) => sat_ranges,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn sat_info(ordinal: u64) -> Result<HiroSatInfo, String> {

    call_service(Provider::Hiro, EndPoint::SatInfo, default_args(Function::SatInfo { ordinal })).await.map(|response| {
        match response {
            Response::SatInfo(ordinal_info) => ordinal_info,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn sat_inscriptions(ordinal: u64) -> Result<HiroSatInscriptions, String> {

    call_service(Provider::Hiro, EndPoint::SatInscriptions, default_args(Function::SatInscriptions { ordinal })).await.map(|response| {
        match response {
            Response::SatInscriptions(inscriptions) => inscriptions,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn inscription_info(inscription_id: String) -> Result<HiroSatInscription, String> {

    call_service(Provider::Hiro, EndPoint::InscriptionInfo, default_args(Function::InscriptionInfo { inscription_id })).await.map(|response| {
        match response {
            Response::SatInscription(inscription) => inscription,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn inscription_content(inscription_id: String) -> Result<Vec<u8>, String> {

    call_service(Provider::Hiro, EndPoint::InscriptionContent, default_args(Function::InscriptionContent { inscription_id })).await.map(|response| {
        match response {
            Response::InscriptionContent(content) => content,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn brc20_details(ticker: String) -> Result<HiroBrc20Details, String> {

    call_service(Provider::Hiro, EndPoint::Brc20Details, default_args(Function::Brc20Details { ticker })).await.map(|response| {
        match response {
            Response::Brc20Details(details) => details,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn brc20_holders(ticker: String) -> Result<HiroBrc20Holders, String> {

    call_service(Provider::Hiro, EndPoint::Brc20Holders, default_args(Function::Brc20Holders { ticker })).await.map(|response| {
        match response {
            Response::Brc20Holders(holders) => holders,
            _ => panic!("Unexpected response type"),
        }
    })
}

async fn call_service(
    provider: Provider,
    end_point: EndPoint,
    args: Args,
) -> Result<Response, String> {

    if let Some(service) = SERVICES.get(&(provider, end_point)) {

        let url = service.get_url(args.clone());
        let http_method = service.get_method();
        let headers = service.get_headers();
        let body = service.get_body(args.clone());
        let max_response_bytes = unwrap_max_response_bytes(args);

        let context = candid::encode_args((provider, end_point))
            .map_err(|error| format!("Failure while encoding context: {}", error))?;

        let cycles = get_http_request_cost(
            url.as_str(),
            body.clone().map(|body| body.len() as u64).unwrap_or(0),
            max_response_bytes,
        );
            
        let response = CanisterHttpRequest::new()
            .url(url.as_str())
            .method(http_method)
            .add_headers(headers)
            .body(body)
            .transform_context("transform_http_response", context)
            .max_response_bytes(max_response_bytes)
            .send(cycles)
            .await?;

        let response = candid::decode_args::<(Response,)>(response.body.as_slice())
            .map(|decoded| decoded.0)
            .map_err(|error| format!("Failure while decoding response: {}", error))?;

        return Ok(response);
    }

    Err("No service associated for given provider and endpoint".to_string())
}

#[ic_cdk::query]
fn transform_http_response(args: TransformArgs) -> HttpResponse {
    let mut sanitized = args.response;

    let context_result = candid::decode_args::<(Provider, EndPoint)>(&args.context)
        .map(|decoded| decoded);

    let (provider, end_point) = match context_result {
        Ok((provider, end_point)) => (provider, end_point),
        Err(err) => ic_cdk::trap(&format!("Failed to decode context: {}", err)),
    };

    let service = match SERVICES.get(&(provider, end_point)) {
        Some(service) => service,
        None => ic_cdk::trap(&format!("No service found for provider {:?} and endpoint {:?}", provider, end_point)),
    };

    let response = match service.extract_response(&sanitized.body) {
        Ok(response) => response,
        Err(err) => ic_cdk::trap(&format!("Failed to extract response: {}", err)),
    };

    let body = match candid::encode_args((response,)) {
        Ok(body) => body,
        Err(err) => ic_cdk::trap(&format!("Failed to encode response: {}", err)),
    };

    sanitized.body = body;  

    // Strip out the headers as these will commonly cause an error to occur.
    sanitized.headers = vec![];
    sanitized
}

/// Calculates the baseline cost of sending a request using HTTP outcalls.
fn get_http_request_cost(
    url: &str,
    payload_size_bytes: u64,
    max_response_bytes: u64,
) -> u128 {
    let ingress_bytes = payload_size_bytes as u128 + url.len() as u128 + INGRESS_OVERHEAD_BYTES;
    let base_cost = INGRESS_MESSAGE_RECEIVED_COST
        + INGRESS_MESSAGE_BYTE_RECEIVED_COST * ingress_bytes
        + HTTP_OUTCALL_REQUEST_COST
        + HTTP_OUTCALL_BYTE_RECEIVED_COST * (ingress_bytes + max_response_bytes as u128);
    base_cost as u128
}