extern crate num;
extern crate image;
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
    utils::image::render(&mut pixels, bounds, upper_left, lower_right);
    utils::image::write_image(filename, &pixels, bounds);
}
