use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Offset, TimeZone};

use winapi::um::winnt::LPCWSTR;

use crate::{cstring, debug, unwrap_or_err, DEBUG};

#[no_mangle]
pub extern "stdcall" fn format(
    timestamp_ptr: LPCWSTR,
    format_ptr: LPCWSTR,
    timezone_ptr: LPCWSTR,
) -> LPCWSTR {
    let timestamp = cstring::from_widechar_ptr(timestamp_ptr);
    let timestamp: i64 = unwrap_or_err!(timestamp.parse());
    let format = cstring::from_widechar_ptr(format_ptr);
    let timezone = cstring::from_widechar_ptr(timezone_ptr);

    let timezone = match timezone.parse::<i32>() {
        Ok(secs) => FixedOffset::east(secs),
        Err(_) => {
            if timezone == "local" {
                FixedOffset::east(Local.timestamp(0, 0).offset().fix().local_minus_utc())
            } else {
                FixedOffset::east(0)
            }
        }
    };

    debug!(
        "Timestamp: {}\nFormat: {}\nTimezone: {}",
        timestamp, format, timezone
    );

    let date = NaiveDateTime::from_timestamp(timestamp, 0);
    let date: DateTime<FixedOffset> = DateTime::from_utc(date, timezone);

    debug!("Date: {}", date);

    cstring::to_widechar_ptr(date.format(&format).to_string())
}

#[no_mangle]
pub extern "stdcall" fn parse(date_ptr: LPCWSTR, format_ptr: LPCWSTR) -> LPCWSTR {
    let date_str = cstring::from_widechar_ptr(date_ptr);
    let format = cstring::from_widechar_ptr(format_ptr);

    debug!("Date string: {}\nFormat: {}", date_str, format);

    let date = unwrap_or_err!(DateTime::parse_from_str(&date_str, &format));

    debug!("Date: {}", date);

    cstring::to_widechar_ptr(date.timestamp().to_string())
}
