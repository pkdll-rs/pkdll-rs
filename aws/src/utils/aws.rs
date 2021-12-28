use std::collections::BTreeMap;

use rusoto_signature::{SignedRequest, Region};
use url::Url;



pub fn create_request(method: &str, url: Url, service: &str, region: Region, headers: BTreeMap<String, Vec<Vec<u8>>>, payload: Option<Vec<u8>>) -> SignedRequest {
    let mut signed_request = SignedRequest::new(
        method,
        service,
        &region,
        url.path(),
    );

    let host = match url.host_str() {
        Some(host) => Some(host.to_string()),
        None => None,
    };

    signed_request.set_hostname(host);

    url.query_pairs().for_each(|(name, value)| signed_request.add_param(name, value));

    signed_request.set_payload(payload);

    signed_request.headers = headers;

    signed_request
}