use image::ColorType;
use image::codecs::png::PngEncoder;
use std::fs::File;
use num::Complex;
use crate::calc_complex;

pub fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize),
        upper_left: Complex<f64>, lower_right: Complex<f64>)
        -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64 
    }
}

pub fn render(pixels: &mut [u8], bounds: (usize, usize),
        upper_left: Complex<f64>, lower_right: Complex<f64>) {
    println!("{} {:?}", pixels.len(), bounds);
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match calc_complex::escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

pub fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) {
    let output = File::create(filename).unwrap();
    let encoder = PngEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8).unwrap();
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(image::pixel_to_point((100, 100), (25, 75), Complex {re: -1., im: 1.}, Complex {re: 1., im: -1.}), Complex {re: -0.5, im: -0.5});
}
