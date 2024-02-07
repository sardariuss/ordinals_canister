mod http;
mod types;
mod services;
mod utils;

use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};

use services::{SERVICES, default_args, unwrap_max_response_bytes, deduce_end_point, validate_providers};
use types::{SatRanges, SatInfo, HiroSatInscription, HiroSatInscriptions, Provider, OrdFunction, Args, OrdArgs,
    ProviderOrdResult, EndPoint, Response, OrdResult, OrdError, MultiOrdResult, HiroBrc20Details, HiroBrc20Holders,
    SatRangeArgs, SatInfoArgs, SatInscriptionsArgs, InscriptionInfoArgs, InscriptionContentArgs, Brc20DetailsArgs, Brc20HoldersArgs};
use utils::from_ord_args;

use crate::http::CanisterHttpRequest;

/// Used for setting the max response bytes.
const ONE_KIB: u64 = 1_024;

// Used to approximate the real size of the HTTP request message?
// TODO: to validate, copied from the ETC-RPC canister
pub const INGRESS_OVERHEAD_BYTES: u128 = 100;

/// Cycles cost constants, based on
/// https://internetcomputer.org/docs/current/developer-docs/gas-cost#details-cost-of-compute-and-storage-transactions-on-the-internet-computer
/// Warning: This assumes the canister runs on an application subnet (13 nodes)
pub const INGRESS_MESSAGE_RECEIVED_COST: u128 = 1_200_000;
pub const INGRESS_MESSAGE_BYTE_RECEIVED_COST: u128 = 2_000;
pub const HTTP_OUTCALL_REQUEST_COST: u128 = 49_140_000;
pub const HTTP_OUTCALL_BYTE_SENT_COST: u128 = 5_200;
pub const HTTP_OUTCALL_BYTE_RECEIVED_COST: u128 = 10_400;

#[ic_cdk::update]
async fn request(args: OrdArgs) -> MultiOrdResult {

    let prepared_requests = match prepare_requests(args.clone()) {
        Ok(prepared_requests) => {
            prepared_requests
        },
        Err(err) => {
            return MultiOrdResult::Consistent(Err(err));
        }
    };

    // Early return if the caller doesn't have enough cycles to pay for all the services.
    match pay_cycles(compute_total_cost(&prepared_requests)) {
        Ok(_) => {},
        Err(err) => {
            return MultiOrdResult::Consistent(Err(err));
        }
    };

    // Execute the requests.
    // TODO: parallelize the calls
    let mut results: Vec<ProviderOrdResult> = vec![];
    for (provider, request) in prepared_requests {
        results.push(ProviderOrdResult{ provider: provider, result: execute_request(request).await });
    }

    // Sort the results.
    match results.first() {
        Some(first) => {
            let equal = results.iter().all(|other| other.result == first.result);
            if equal {
                return MultiOrdResult::Consistent(first.result.clone());
            } else {
                return MultiOrdResult::Inconsistent(results);
            }
        },
        None => {
            // This should never happen, hence the panic.
            panic!("No results");
        }
    }
}

#[ic_cdk::query]
async fn request_cost(args: OrdArgs) -> Result<u128, OrdError> {

    let prepared_requests = prepare_requests(args.clone())?;

    Ok(compute_total_cost(&prepared_requests))
}

#[ic_cdk::update]
async fn bitgem_sat_range(args: SatRangeArgs) -> Result<SatRanges, OrdError> {

    call_service(Provider::Bitgem, EndPoint::SatRange, default_args(OrdFunction::SatRange(args))).await.map(|response| {
        match response {
            Response::SatRange(sat_ranges) => sat_ranges,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn bitgem_sat_info(args: SatInfoArgs) -> Result<SatInfo, OrdError> {

    call_service(Provider::Bitgem, EndPoint::SatInfo, default_args(OrdFunction::SatInfo(args))).await.map(|response| {
        match response {
            Response::SatInfo(ordinal_info) => ordinal_info,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_sat_info(args: SatInfoArgs) -> Result<SatInfo, OrdError> {

    call_service(Provider::Hiro, EndPoint::SatInfo, default_args(OrdFunction::SatInfo(args))).await.map(|response| {
        match response {
            Response::SatInfo(ordinal_info) => ordinal_info,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_sat_inscriptions(args: SatInscriptionsArgs) -> Result<HiroSatInscriptions, OrdError> {

    call_service(Provider::Hiro, EndPoint::SatInscriptions, default_args(OrdFunction::SatInscriptions(args))).await.map(|response| {
        match response {
            Response::SatInscriptions(inscriptions) => inscriptions,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_inscription_info(args: InscriptionInfoArgs) -> Result<HiroSatInscription, OrdError> {

    call_service(Provider::Hiro, EndPoint::InscriptionInfo, default_args(OrdFunction::InscriptionInfo(args))).await.map(|response| {
        match response {
            Response::InscriptionInfo(inscription) => inscription,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_inscription_content(args: InscriptionContentArgs) -> Result<Vec<u8>, OrdError> {

    call_service(Provider::Hiro, EndPoint::InscriptionContent, default_args(OrdFunction::InscriptionContent(args))).await.map(|response| {
        match response {
            Response::InscriptionContent(content) => content,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_brc20_details(args: Brc20DetailsArgs) -> Result<HiroBrc20Details, OrdError> {

    call_service(Provider::Hiro, EndPoint::Brc20Details, default_args(OrdFunction::Brc20Details(args))).await.map(|response| {
        match response {
            Response::Brc20Details(details) => details,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::update]
async fn hiro_brc20_holders(args: Brc20HoldersArgs) -> Result<HiroBrc20Holders, OrdError> {

    call_service(Provider::Hiro, EndPoint::Brc20Holders, default_args(OrdFunction::Brc20Holders(args))).await.map(|response| {
        match response {
            Response::Brc20Holders(holders) => holders,
            _ => panic!("Unexpected response type"),
        }
    })
}

#[ic_cdk::query]
async fn cycles_balance() -> u64 {
    ic_cdk::api::canister_balance()
}

fn prepare_requests(args: OrdArgs) -> Result<Vec<(Provider, CanisterHttpRequest)>, OrdError> {

    // Check that the providers are available for this function.
    let end_point = deduce_end_point(args.function.clone());
    let providers = match validate_providers(args.providers.clone(), end_point){
        Ok(providers) => providers,
        Err(missing) => {
            return Err(OrdError::NoServiceError{ providers: missing, end_point });
        }
    };

    // Early return if no provider is available.
    if providers.len() == 0 {
        return Err(OrdError::NoServiceError{ providers, end_point });
    }

    let prepared_requests = providers.iter().map(|provider| {
        let request = prepare_request(provider.clone(), end_point.clone(), from_ord_args(args.clone()));
        (provider.clone(), request.clone())
    }).collect();

    Ok(prepared_requests)
}

fn prepare_request(
    provider: Provider,
    end_point: EndPoint,
    args: Args,
) -> CanisterHttpRequest {

    if let Some(service) = SERVICES.get(&(provider, end_point)) {

        let url = service.get_url(args.clone());
        let http_method = service.get_method();
        let body = service.get_body(args.clone());
        let max_response_bytes = unwrap_max_response_bytes(args);

        let context = candid::encode_args((provider, end_point))
            .map_err(|error| format!("Failure while encoding context: {}", error)).unwrap();

        let cost = get_http_request_cost(
            url.as_str(),
            body.clone().map(|body| body.len() as u64).unwrap_or(0),
            max_response_bytes,
        );

        let request = CanisterHttpRequest::new()
            .url(url.as_str())
            .method(http_method)
            .body(body.clone())
            .transform_context("transform_http_response", context)
            .max_response_bytes(max_response_bytes)
            .cycles(cost);

        return request;
    }
    
    panic!("No service for provider: {:?} and end point: {:?}", provider, end_point);
}

fn compute_total_cost(requests: &Vec<(Provider, CanisterHttpRequest)>) -> u128 {
    requests.iter().map(|request| request.1.cycles).sum()
}

fn pay_cycles(cycles_cost: u128) -> Result<(), OrdError> {
    // Check that the caller has enough cycles to pay for the request.
    let cycles_available: u128 = ic_cdk::api::call::msg_cycles_available128();
    if cycles_available < cycles_cost {
        return Err(OrdError::TooFewCycles {
            expected: cycles_cost,
            received: cycles_available,
        }
        .into());
    }
    // Pay for the request.
    ic_cdk::api::call::msg_cycles_accept128(cycles_cost);
    Ok(())
}

async fn execute_request(
    request: CanisterHttpRequest,
) -> OrdResult {
    let http_response = request
        .send()
        .await
        .map_err(|error| OrdError::HttpSendError(error))?;

    candid::decode_args::<(OrdResult,)>(http_response.body.as_slice())
        .map(|decoded| decoded.0)
        .map_err(|error| OrdError::ResponseDecodingError(format!("Failure while decoding response: {}", error)))?
}

async fn call_service(
    provider: Provider,
    end_point: EndPoint,
    args: Args,
) -> OrdResult {

    let request = prepare_request(provider, end_point, args.clone());

    pay_cycles(request.cycles)?;

    execute_request(request).await
}

#[ic_cdk::query]
fn transform_http_response(args: TransformArgs) -> HttpResponse {

    let mut sanitized = args.response;
 
    let result: OrdResult = {
        
        let context_result = candid::decode_args::<(Provider, EndPoint)>(&args.context)
            .map(|decoded| decoded);

        match context_result {
            Err(err) => Err(OrdError::ContextDecodingError(format!("Failed to decode context: {}", err))),
            Ok((provider, end_point)) => {
                match SERVICES.get(&(provider, end_point)) {
                    None => Err(OrdError::NoServiceError{ providers: vec![provider], end_point }),
                    Some(service) => service.extract_response(&sanitized.body),
                }
            }
        }
    };

    let body = match candid::encode_args((result,)) {
        Ok(body) => body,
        Err(err) => ic_cdk::trap(&format!("Failed to encode response result: {}", err)),
    };

    sanitized.body = body;  

    // Strip out the headers as these will commonly cause an error to occur.
    sanitized.headers = vec![];
    sanitized
}

/// Calculates the baseline cost of sending a request using HTTP outcalls.
fn get_http_request_cost(
    url: &str,
    body_size_bytes: u64,
    max_response_bytes: u64,
) -> u128 {
    // Assume the request size = size(body) + size(url) + ingress overhead
    let request_bytes = body_size_bytes as u128 + url.len() as u128 + INGRESS_OVERHEAD_BYTES;
    // Take the worst case scenario where the response uses the maximum number of bytes.
    let response_bytes = max_response_bytes as u128;
    
    // Add the price of receiving the message (for the call to transform context) 
    // to the price of the http outcall.
    INGRESS_MESSAGE_RECEIVED_COST
        + INGRESS_MESSAGE_BYTE_RECEIVED_COST * response_bytes
        + HTTP_OUTCALL_REQUEST_COST
        + HTTP_OUTCALL_BYTE_SENT_COST * request_bytes
        + HTTP_OUTCALL_BYTE_RECEIVED_COST * response_bytes
}
