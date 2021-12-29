use std::collections::BTreeMap;

use chrono::{TimeZone, Utc};
use signature::{credential::AwsCredentials, Region};
use winapi::um::winnt::LPCWSTR;

use rusoto_signature as signature;

use crate::{cstring, debug, unwrap_or_err, utils::aws, DEBUG, ERR};

#[no_mangle]
pub extern "stdcall" fn sign(
    method_ptr: LPCWSTR,
    url_ptr: LPCWSTR,
    service_ptr: LPCWSTR,
    region_ptr: LPCWSTR,
    headers_ptr: LPCWSTR,
    payload_ptr: LPCWSTR,

    key_ptr: LPCWSTR,
    secret_ptr: LPCWSTR,
    token_ptr: LPCWSTR,
    expires_at_ptr: LPCWSTR,
) -> LPCWSTR {
    let method = cstring::from_widechar_ptr(method_ptr).to_ascii_uppercase();
    let url = cstring::from_widechar_ptr(url_ptr);
    let service = cstring::from_widechar_ptr(service_ptr);
    let region = cstring::from_widechar_ptr(region_ptr);
    let headers = cstring::from_widechar_ptr(headers_ptr);
    let payload = {
        let payload = cstring::from_widechar_ptr(payload_ptr);
        match base64::decode(payload) {
            Ok(payload) => Some(payload),
            Err(_) => None,
        }
    };

    let key = cstring::from_widechar_ptr(key_ptr);
    let secret = cstring::from_widechar_ptr(secret_ptr);

    let token = {
        let token = cstring::from_widechar_ptr(token_ptr);
        if token.len() > 0 {
            Some(token)
        } else {
            None
        }
    };

    let expires_at = {
        let expires_at = cstring::from_widechar_ptr(expires_at_ptr);
        match expires_at.parse::<i64>() {
            Ok(expires_at) => Some(Utc.timestamp_millis(expires_at)),
            Err(_) => None,
        }
    };

    debug!(
        "Called sign({},{},{},{},{},{:?},{},{},{:?},{:?})",
        method, url, service, region, headers, payload, key, secret, token, expires_at
    );

    let url = unwrap_or_err!(url.parse::<url::Url>());
    let region = unwrap_or_err!(region.parse::<Region>());
    let mut headers_map: BTreeMap<String, Vec<Vec<u8>>> = BTreeMap::new();
    headers.split('\n').for_each(|header| {
        let header = header.trim();
        let header = header.split_once(':');
        if let Some(header) = header {
            let name = header.0.to_string();
            let value = header.1.trim_start().as_bytes().to_vec();
            headers_map.entry(name).or_default().push(value);
        }
    });

    let mut signed_request =
        aws::create_request(&method, url, &service, region, headers_map, payload);
    debug!("Signed request: {:?}", signed_request);

    let creds = AwsCredentials::new(key, secret, token, expires_at);
    signed_request.sign(&creds);

    let auth_header = match signed_request.headers().get("authorization") {
        Some(value) => unsafe { String::from_utf8_unchecked(value[0].clone()) },
        None => {
            return cstring::to_widechar_ptr(format!("{}{}", ERR, "can't sign request"));
        }
    };

    let date = match signed_request.headers().get("x-amz-date") {
        Some(value) => unsafe { String::from_utf8_unchecked(value[0].clone()) },
        None => {
            return cstring::to_widechar_ptr(format!("{}{}", ERR, "can't sign request"));
        }
    };

    let payload_sign = match signed_request.headers().get("x-amz-content-sha256") {
        Some(value) => unsafe { String::from_utf8_unchecked(value[0].clone()) },
        None => {
            return cstring::to_widechar_ptr(format!("{}{}", ERR, "can't sign request"));
        }
    };

    let result = format!(
        r#"{{"authorization":"{}", "x-amz-date":"{}", "x-amz-content-sha256":"{}"}}"#,
        auth_header, date, payload_sign
    );

    debug!("Result: {}", result);

    cstring::to_widechar_ptr(result)
}
