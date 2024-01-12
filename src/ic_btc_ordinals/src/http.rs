use candid::Func;

use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
        TransformContext, TransformFunc,
    },
    id,
};

use crate::types::HttpSendError;

/// Used to build a request to the Management Canister's `http_request` method.
pub struct CanisterHttpRequest {
    args: CanisterHttpRequestArgument,
}

impl Default for CanisterHttpRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl CanisterHttpRequest {
    /// Creates a new request to be built up by having
    pub fn new() -> Self {
        Self {
            args: CanisterHttpRequestArgument {
                url: Default::default(),
                max_response_bytes: Default::default(),
                headers: vec![HttpHeader { //@todo: add application/json here?
                    name: "User-Agent".to_string(),
                    value: "BTC Ordinals Canister".to_string(),
                }],
                body: Default::default(),
                method: HttpMethod::GET,
                transform: None,
            },
        }
    }

    /// Updates the HTTP method in the `args` field.
    pub fn method(mut self, http_method: HttpMethod) -> Self {
        self.args.method = http_method;
        self
    }

    /// Adds HTTP headers for the request
    pub fn add_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.args
            .headers
            .extend(headers.iter().map(|(name, value)| HttpHeader {
                name: name.to_string(),
                value: value.to_string(),
            }));
        self
    }

    /// Set the body of the request
    pub fn body(mut self, body: Option<Vec<u8>>) -> Self {
      self.args.body = body;
      self
    }

    /// Updates the URL in the `args` field.
    pub fn url(mut self, url: &str) -> Self {
        self.args.url = String::from(url);
        self
    }

    /// Updates the transform context of the request.
    pub fn transform_context(mut self, method: &str, context: Vec<u8>) -> Self {
        let context = TransformContext {
            function: TransformFunc(Func {
                principal: id(),
                method: method.to_string(),
            }),
            context,
        };

        self.args.transform = Some(context);
        self
    }

    /// Updates the max_response_bytes of the request.
    pub fn max_response_bytes(mut self, max_response_bytes: u64) -> Self {
        self.args.max_response_bytes = Some(max_response_bytes);
        self
    }

    /// Wraps around `http_request` to issue a request to the `http_request` endpoint.
    pub async fn send(self, cycles: u128) -> Result<HttpResponse, HttpSendError> {
        http_request(self.args, cycles)
            .await
            .map(|(response,)| response)
            .map_err(|(rejection_code, _)| HttpSendError{ rejection_code })
    }
}
