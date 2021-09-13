extern crate num;
extern crate image;
extern crate crossbeam;

pub mod utils;
pub mod calc_complex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        println!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        println!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let bounds: (usize, usize) = utils::parser::parse_pair(&args[2], 'x').unwrap_or_else(|| {
        println!("Arg PIXELS wrong format!");
        std::process::exit(1);
    });
    let upper_left = utils::parser::parse_complex(&args[3]).unwrap_or_else(|| {
        println!("Arg UPPERLEFT wrong format!");
        std::process::exit(1);
    });
    let lower_right = utils::parser::parse_complex(&args[4]).unwrap_or_else(|| {
        println!("Arg LOWERRIGHT wrong format!");
        std::process::exit(1);
    });
    
    println!("{} {:?} {:?} {:?}", filename, bounds, upper_left, lower_right);
    let size = bounds.0 * bounds.1;
    let mut pixels: Vec<u8> = vec![0; size];
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        let _ = crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = utils::image::pixel_to_point(bounds, (0, top),
                    upper_left, lower_right);
                let band_lower_right = utils::image::pixel_to_point(bounds,
                    (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move |_| {
                    utils::image::render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap_or_else(|arg| {
            println!("crossbeam error! {:?}", arg);
            std::process::exit(1);
        });
    }
    utils::image::write_image(filename, &pixels, bounds);
}
