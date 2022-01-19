use crate::{cstring, debug, unwrap_or_err, utils::svg, DEBUG};
use winapi::um::winnt::LPCWSTR;

#[no_mangle]
pub extern "stdcall" fn render(
    svg_data_ptr: LPCWSTR,
    width_ptr: LPCWSTR,
    height_ptr: LPCWSTR,
    background_color_r_ptr: LPCWSTR,
    background_color_g_ptr: LPCWSTR,
    background_color_b_ptr: LPCWSTR,
    background_color_a_ptr: LPCWSTR,
) -> LPCWSTR {
    let svg_data = cstring::from_widechar_ptr(svg_data_ptr);
    let svg_data = unwrap_or_err!(base64::decode(svg_data));
    debug!("SVG: {:?}", String::from_utf8(svg_data.clone()));

    let width = cstring::from_widechar_ptr(width_ptr);
    let width: i32 = unwrap_or_err!(width.parse());

    let height = cstring::from_widechar_ptr(height_ptr);
    let height: i32 = unwrap_or_err!(height.parse());

    let background_color_r = cstring::from_widechar_ptr(background_color_r_ptr);
    let background_color_r: f32 = unwrap_or_err!(background_color_r.parse());

    let background_color_g = cstring::from_widechar_ptr(background_color_g_ptr);
    let background_color_g: f32 = unwrap_or_err!(background_color_g.parse());

    let background_color_b = cstring::from_widechar_ptr(background_color_b_ptr);
    let background_color_b: f32 = unwrap_or_err!(background_color_b.parse());

    let background_color_a = cstring::from_widechar_ptr(background_color_a_ptr);
    let background_color_a: f32 = unwrap_or_err!(background_color_a.parse());

    let width = if width >= 0 { width as u32 } else { 0 };
    let height = if height >= 0 { height as u32 } else { 0 };

    let size = svg::parse_size(width, height);
    let color = svg::parse_color(
        background_color_r,
        background_color_g,
        background_color_b,
        background_color_a,
    );

    debug!("Size: {:?}\nColor: {:?}", size, color);

    let encoded = unwrap_or_err!(svg::render(svg_data, size, color));

    cstring::to_widechar_ptr(base64::encode(encoded))
}
