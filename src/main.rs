use colors_transform::Color;

mod complex;

#[derive(serde::Deserialize)]
struct Config {
    size: (u32, u32),
    x_range: (f64, f64),
    y_range: (f64, f64),
    max_iteration_count: u32,
    max_distance: f64,
    scale: f64,
}

fn main() {
    println!("Hello, world!");

    let content = std::fs::read_to_string("./config.json").unwrap();
    let config: Config = serde_json::from_slice(content.as_bytes()).unwrap();

    let Config {
        size: (width, height),
        x_range: (x_min, x_max),
        y_range: (y_min, y_max),
        max_distance,
        max_iteration_count,
        scale,
    } = config;

    const TOTAL_STEPS: u64 = 100;
    let bar = indicatif::ProgressBar::new(TOTAL_STEPS);
    bar.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}% ETA: {eta}",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    let mut image = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let (x, y) = (x as f64, y as f64);

        let pixel_progress = x + y * width as f64;

        let ratio = pixel_progress / (width * height) as f64;
        let progress_bar_position = (ratio * TOTAL_STEPS as f64) as u64;

        bar.set_position(progress_bar_position);

        let mapped_x = x_min + (x_max - x_min) * x / width as f64;
        let mapped_y = y_min + (y_max - y_min) * y / height as f64;

        let mut z = complex::Complex::from(0.0, 0.0);
        let c = complex::Complex::from(scale * mapped_x, scale * mapped_y);

        let mut iteration = 0;
        while iteration < max_iteration_count && z.norm() <= max_distance {
            z = z * z + c;
            iteration += 1;
        }
        let rgb = colors_transform::Hsl::from(
            iteration as f32 / 2.0 + 245.0,
            100.0,
            if iteration < max_iteration_count {
                iteration as f32 / max_iteration_count as f32 * 50.0
            } else {
                0.0
            },
        )
        .to_rgb();
        let r = rgb.get_red() as u8;
        let g = rgb.get_green() as u8;
        let b = rgb.get_blue() as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    image.save("fractal.png").unwrap();
}
