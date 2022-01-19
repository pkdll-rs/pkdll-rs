use resvg::ScreenSize;
use tiny_skia::Color;
use usvg::Options;

lazy_static! {
    static ref OPT: Options = {
        let mut opt = usvg::Options::default();
        opt.fontdb.load_system_fonts();
        opt
    };
}

pub fn parse_size(width: u32, height: u32) -> Option<ScreenSize> {
    ScreenSize::new(width, height)
}

pub fn parse_color(r: f32, g: f32, b: f32, a: f32) -> Option<Color> {
    Color::from_rgba(r / 255.0, g / 255.0, b / 255.0, a / 255.0)
}

pub fn render(
    svg_data: Vec<u8>,
    size: Option<ScreenSize>,
    color: Option<Color>,
) -> Result<Vec<u8>, png::EncodingError> {
    let rtree = usvg::Tree::from_data(&svg_data, &OPT.to_ref()).unwrap();
    let pixmap_size = size.unwrap_or_else(|| rtree.svg_node().size.to_screen_size());

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    if let Some(color) = color {
        pixmap.fill(color);
    }

    let size = match size {
        Some(size) => usvg::FitTo::Size(size.width(), size.height()),
        None => usvg::FitTo::Original,
    };

    resvg::render(
        &rtree,
        size,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.encode_png()
}
