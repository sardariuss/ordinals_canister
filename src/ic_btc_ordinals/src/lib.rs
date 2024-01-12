mod http;
mod types;
mod services;
mod utils;

use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};

use services::{SERVICES, default_args, unwrap_max_response_bytes, get_available};
use types::{Utxo, BitgemSatRanges, SatInfo, HiroSatInscription, HiroSatInscriptions, Provider, Function, Args,
    EndPoint, Response, OrdResult, OrdError, MultiOrdResult, HiroBrc20Details, HiroBrc20Holders};

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
async fn request(args: Args) -> MultiOrdResult {

    let available = get_available(args.clone().function);

    let mut results = vec![];

    // TODO: parallelize the calls
    for (provider, end_point) in available {
        let response = call_service(provider, end_point, args.clone()).await;
        results.push((provider, response));
    }

    if let Some(first) = results.first() {
        let equal = results.iter().all(|result| result.1 == first.1);
        if equal {
            return MultiOrdResult::Consistent(first.1.clone());
        } else {
            return MultiOrdResult::Inconsistent(results);
        }
    } else {
        panic!("No service available for this function");
    }
}

#[ic_cdk::update]
async fn bitgem_sat_range(utxo: Utxo) -> Result<BitgemSatRanges, OrdError> {

    call_service(Provider::Bitgem, EndPoint::SatRange, default_args(Function::SatRange { utxo })).await.map(|response| {
        match response {
            Response::SatRange(sat_ranges) => sat_ranges,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn bitgem_sat_info(ordinal: u64) -> Result<SatInfo, OrdError> {

    call_service(Provider::Bitgem, EndPoint::SatInfo, default_args(Function::SatInfo { ordinal })).await.map(|response| {
        match response {
            Response::SatInfo(ordinal_info) => ordinal_info,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_sat_info(ordinal: u64) -> Result<SatInfo, OrdError> {

    call_service(Provider::Hiro, EndPoint::SatInfo, default_args(Function::SatInfo { ordinal })).await.map(|response| {
        match response {
            Response::SatInfo(ordinal_info) => ordinal_info,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_sat_inscriptions(ordinal: u64) -> Result<HiroSatInscriptions, OrdError> {

    call_service(Provider::Hiro, EndPoint::SatInscriptions, default_args(Function::SatInscriptions { ordinal })).await.map(|response| {
        match response {
            Response::SatInscriptions(inscriptions) => inscriptions,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_inscription_info(inscription_id: String) -> Result<HiroSatInscription, OrdError> {

    call_service(Provider::Hiro, EndPoint::InscriptionInfo, default_args(Function::InscriptionInfo { inscription_id })).await.map(|response| {
        match response {
            Response::InscriptionInfo(inscription) => inscription,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_inscription_content(inscription_id: String) -> Result<Vec<u8>, OrdError> {

    call_service(Provider::Hiro, EndPoint::InscriptionContent, default_args(Function::InscriptionContent { inscription_id })).await.map(|response| {
        match response {
            Response::InscriptionContent(content) => content,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_brc20_details(ticker: String) -> Result<HiroBrc20Details, OrdError> {

    call_service(Provider::Hiro, EndPoint::Brc20Details, default_args(Function::Brc20Details { ticker })).await.map(|response| {
        match response {
            Response::Brc20Details(details) => details,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_brc20_holders(ticker: String) -> Result<HiroBrc20Holders, OrdError> {

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
) -> OrdResult {

    if let Some(service) = SERVICES.get(&(provider, end_point)) {

        let url = service.get_url(args.clone());
        let http_method = service.get_method();
        let headers = service.get_headers();
        let body = service.get_body(args.clone());
        let max_response_bytes = unwrap_max_response_bytes(args);

        let context = candid::encode_args((provider, end_point))
            .map_err(|error| OrdError::CandidEncodingError(format!("Failure while encoding context: {}", error)))?;

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
            .await
            .map_err(|error| OrdError::HttpSendError(error))?;

        let response = candid::decode_args::<(Response,)>(response.body.as_slice())
            .map(|decoded| decoded.0)
            .map_err(|error| OrdError::CandidDecodingError(format!("Failure while decoding response: {}", error)))?;

        return Ok(response);
    }

    Err(OrdError::NoServiceError{ provider, end_point })
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