use colorgrad::{BlendMode, Color, CustomGradient, Gradient, Interpolation};
use image::{imageops, ImageBuffer, Rgba};
use std::fs::{create_dir, File};
use std::io::BufReader;
use std::path::Path;

fn main() {
    let preset_gradients = &[
        (colorgrad::cubehelix_default(), "cubehelix_default"),
        (colorgrad::warm(), "warm"),
        (colorgrad::cool(), "cool"),
        (colorgrad::rainbow(), "rainbow"),
        (colorgrad::cividis(), "cividis"),
        (colorgrad::sinebow(), "sinebow"),
        (colorgrad::turbo(), "turbo"),
        (colorgrad::viridis(), "viridis"),
        (colorgrad::plasma(), "plasma"),
        (colorgrad::magma(), "magma"),
        (colorgrad::inferno(), "inferno"),
        (colorgrad::br_bg(), "br_bg"),
        (colorgrad::pr_gn(), "pr_gn"),
        (colorgrad::pi_yg(), "pi_yg"),
        (colorgrad::pu_or(), "pu_or"),
        (colorgrad::rd_bu(), "rd_bu"),
        (colorgrad::rd_gy(), "rd_gy"),
        (colorgrad::rd_yl_bu(), "rd_yl_bu"),
        (colorgrad::rd_yl_gn(), "rd_yl_gn"),
        (colorgrad::spectral(), "spectral"),
        (colorgrad::blues(), "blues"),
        (colorgrad::greens(), "greens"),
        (colorgrad::greys(), "greys"),
        (colorgrad::oranges(), "oranges"),
        (colorgrad::purples(), "purples"),
        (colorgrad::reds(), "reds"),
        (colorgrad::bu_gn(), "bu_gn"),
        (colorgrad::bu_pu(), "bu_pu"),
        (colorgrad::gn_bu(), "gn_bu"),
        (colorgrad::or_rd(), "or_rd"),
        (colorgrad::pu_bu_gn(), "pu_bu_gn"),
        (colorgrad::pu_bu(), "pu_bu"),
        (colorgrad::pu_rd(), "pu_rd"),
        (colorgrad::rd_pu(), "rd_pu"),
        (colorgrad::yl_gn_bu(), "yl_gn_bu"),
        (colorgrad::yl_gn(), "yl_gn"),
        (colorgrad::yl_or_br(), "yl_or_br"),
        (colorgrad::yl_or_rd(), "yl_or_rd"),
    ];

    // Custom gradients

    let grad_1 = CustomGradient::new().build().unwrap();

    let grad_2 = CustomGradient::new()
        .colors(&[
            Color::from_rgba8(0, 206, 209, 255),
            Color::from_rgba8(255, 105, 180, 255),
            Color::new(0.274, 0.5, 0.7, 1.0),
            Color::from_hsva(50.0, 1.0, 1.0, 1.0),
            Color::from_hsva(348.0, 0.9, 0.8, 1.0),
        ])
        .build()
        .unwrap();

    let grad_3 = CustomGradient::new()
        .html_colors(&["#C41189", "#00BFFF", "#FFD700"])
        .build()
        .unwrap();

    let grad_4 = CustomGradient::new()
        .html_colors(&["gold", "hotpink", "darkturquoise"])
        .build()
        .unwrap();

    let grad_5 = CustomGradient::new()
        .html_colors(&["rgb(125,110,221)", "rgb(90%,45%,97%)", "hsl(229,79%,85%)"])
        .build()
        .unwrap();

    let grad_6 = CustomGradient::new()
        .colors(&[
            Color::from_rgba8(255, 0, 0, 255),
            Color::from_rgba8(255, 0, 0, 0),
        ])
        .build()
        .unwrap();

    // Domain & color position

    let domain_1 = CustomGradient::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .build()
        .unwrap();

    let domain_2 = CustomGradient::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .domain(&[0.0, 100.0])
        .build()
        .unwrap();

    let domain_3 = CustomGradient::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .domain(&[-1.0, 1.0])
        .build()
        .unwrap();

    let color_pos_1 = CustomGradient::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .domain(&[0.0, 0.7, 1.0])
        .build()
        .unwrap();

    let color_pos_2 = CustomGradient::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .domain(&[15.0, 30.0, 80.0])
        .build()
        .unwrap();

    let color_pos_3 = CustomGradient::new()
        .html_colors(&["deeppink", "#470a5e", "red", "#ff0"])
        .domain(&[0.0, 0.7, 0.7, 1.0])
        .build()
        .unwrap();

    // Blending modes

    let colors = &["#FFF", "#00F"];

    let blend_mode_rgb = CustomGradient::new()
        .html_colors(colors)
        .mode(BlendMode::Rgb)
        .build()
        .unwrap();

    let blend_mode_linear_rgb = CustomGradient::new()
        .html_colors(colors)
        .mode(BlendMode::LinearRgb)
        .build()
        .unwrap();

    let blend_mode_oklab = CustomGradient::new()
        .html_colors(colors)
        .mode(BlendMode::Oklab)
        .build()
        .unwrap();

    let blend_mode_hsv = CustomGradient::new()
        .html_colors(colors)
        .mode(BlendMode::Hsv)
        .build()
        .unwrap();

    // Interpolation

    let colors = &["#C41189", "#00BFFF", "#FFD700"];
    let space = BlendMode::Rgb;

    let interp_linear = CustomGradient::new()
        .html_colors(colors)
        .mode(space)
        .interpolation(Interpolation::Linear)
        .build()
        .unwrap();

    let interp_catmull_rom = CustomGradient::new()
        .html_colors(colors)
        .mode(space)
        .interpolation(Interpolation::CatmullRom)
        .build()
        .unwrap();

    let interp_basis = CustomGradient::new()
        .html_colors(colors)
        .mode(space)
        .interpolation(Interpolation::Basis)
        .build()
        .unwrap();

    let custom_gradients = &[
        (&grad_1, "custom-default"),
        (&grad_2, "custom-colors"),
        (&grad_3, "custom-hex-colors"),
        (&grad_4, "custom-named-colors"),
        (&grad_5, "custom-css-colors"),
        (&grad_6, "custom-transparent"),
        (&domain_1, "domain-default"),
        (&domain_2, "domain-0-100"),
        (&domain_3, "domain-neg1-1"),
        (&color_pos_1, "color-position-1"),
        (&color_pos_2, "color-position-2"),
        (&color_pos_3, "color-position-3"),
        (&blend_mode_rgb, "blend-mode-rgb"),
        (&blend_mode_linear_rgb, "blend-mode-linear-rgb"),
        (&blend_mode_oklab, "blend-mode-oklab"),
        (&blend_mode_hsv, "blend-mode-hsv"),
        (&interp_linear, "interpolation-linear"),
        (&interp_catmull_rom, "interpolation-catmull-rom"),
        (&interp_basis, "interpolation-basis"),
    ];

    // Sharp gradients

    let grad = colorgrad::rainbow();
    let segments = 11;

    let sharp_gradients = &[
        (grad.sharp(segments, 0.0), "0.0"),
        (grad.sharp(segments, 0.1), "0.1"),
        (grad.sharp(segments, 0.2), "0.2"),
        (grad.sharp(segments, 0.3), "0.3"),
        (grad.sharp(segments, 0.4), "0.4"),
        (grad.sharp(segments, 0.5), "0.5"),
        (grad.sharp(segments, 0.6), "0.6"),
        (grad.sharp(segments, 0.7), "0.7"),
        (grad.sharp(segments, 0.8), "0.8"),
        (grad.sharp(segments, 0.9), "0.9"),
        (grad.sharp(segments, 1.0), "1.0"),
    ];

    let width = 1000;
    let height = 150;
    let padding = 10;

    let output_dir = Path::new("example_output/");

    if !output_dir.exists() {
        create_dir(output_dir).expect("Failed to create example_output/ directory.");
    }

    for (gradient, name) in preset_gradients {
        let imgbuf = grad_rgb_plot(&gradient, width, height, padding);
        let file_path = format!("example_output/preset_{}.png", name);
        println!("{}", file_path);
        imgbuf.save(file_path).unwrap();
    }

    for (gradient, name) in custom_gradients {
        let imgbuf = grad_rgb_plot(&gradient, width, height, padding);
        let file_path = format!("example_output/{}.png", name);
        println!("{}", file_path);
        imgbuf.save(file_path).unwrap();
    }

    for (gradient, name) in sharp_gradients {
        let imgbuf = grad_rgb_plot(&gradient, width, height, padding);
        let file_path = format!("example_output/sharp-smoothness-{}.png", name);
        println!("{}", file_path);
        imgbuf.save(file_path).unwrap();
    }

    // GIMP gradients

    for item in Path::new("examples/ggr/").read_dir().unwrap() {
        let path = item.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "ggr" {
                let fname = path.file_name().unwrap().to_str().unwrap();
                let (gradient, _) = parse_ggr(&path);
                let imgbuf = grad_rgb_plot(&gradient, width, height, padding);
                let file_path = format!("example_output/ggr_{fname}.png");
                println!("{}", file_path);
                imgbuf.save(file_path).unwrap();
            }
        }
    }
}

fn parse_ggr<P: AsRef<Path>>(filepath: P) -> (Gradient, String) {
    let input = File::open(filepath).unwrap();
    let buf = BufReader::new(input);
    let fg = Color::new(0.0, 0.0, 0.0, 1.0);
    let bg = Color::new(1.0, 1.0, 1.0, 1.0);
    colorgrad::parse_ggr(buf, &fg, &bg).unwrap()
}

fn gradient_image(gradient: &Gradient, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (dmin, dmax) = gradient.domain();
    ImageBuffer::from_fn(width, height, |x, _| {
        let rgba = gradient
            .at(remap(x as f64, 0.0, width as f64, dmin, dmax))
            .to_rgba8();
        Rgba(rgba)
    })
}

fn rgb_plot(grad: &Gradient, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut imgbuf = ImageBuffer::from_pixel(
        width,
        height,
        Rgba(Color::new(0.9, 0.9, 0.9, 1.0).to_rgba8()),
    );
    let (dmin, dmax) = grad.domain();
    let fw = width as f64;
    let y1 = 0.0;
    let y2 = height as f64;

    for x in 0..width {
        let col = grad.at(remap(x as f64, 0.0, fw, dmin, dmax));
        let yr = remap(col.r, 0.0, 1.0, y2, y1);
        let yg = remap(col.g, 0.0, 1.0, y2, y1);
        let yb = remap(col.b, 0.0, 1.0, y2, y1);

        if (y1..y2).contains(&yr) {
            let pixel = imgbuf.get_pixel_mut(x, yr as u32);
            *pixel = Rgba([255, 0, 0, 255]);
        }

        if (y1..y2).contains(&yg) {
            let pixel = imgbuf.get_pixel_mut(x, yg as u32);
            *pixel = Rgba([0, 128, 0, 255]);
        }

        if (y1..y2).contains(&yb) {
            let pixel = imgbuf.get_pixel_mut(x, yb as u32);
            *pixel = Rgba([0, 0, 255, 255]);
        }
    }
    imgbuf
}

fn grad_rgb_plot(
    grad: &Gradient,
    width: u32,
    height: u32,
    padding: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let w = width + padding * 2;
    let h = height * 2 + padding * 3;
    let mut imgbuf = ImageBuffer::from_pixel(w, h, Rgba(Color::new(1.0, 1.0, 1.0, 1.0).to_rgba8()));

    let grad_img = gradient_image(grad, width, height);
    imageops::replace(&mut imgbuf, &grad_img, padding.into(), padding.into());

    let plot_img = rgb_plot(grad, width, height);
    imageops::replace(
        &mut imgbuf,
        &plot_img,
        padding.into(),
        (height + padding * 2).into(),
    );

    imgbuf
}

// Map t in range [a, b] to range [c, d]
fn remap(t: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    (t - a) * ((d - c) / (b - a)) + c
}
