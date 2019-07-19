use num::complex::Complex;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
// To use encoder.set()
use png::HasParameters;

use rayon::{self, prelude::*};

fn main() {

    let r = Config::new(-0.04, 1.5, -70.);
    let g = Config::new(0.0613, 0.5, 0.);
    let b = Config::new(0.09075, 3.5, 170.);

    let color = Coloring::new(r, g, b);


    // let start = 0.;
    // let end =  6.283185;
    // let precision = 0.001;

    let start = 1.3;
    let end = 0.01;
    let precision = 0.0001;


    // 
    // step22293    lot of colors
    // step31452    mandlebrot

    //step188

    // let zoom = 0.00001;
    let zoom = 1.;

    // zero_pad_directory(r"E:\fractals\steps");


    // dbg!{float_iterator(start, end, precision).nth(2969)};

    let c = 0.7885 * Complex::new(0., 2.989).exp();

    run_singular(c, &color, zoom);

    // run_animation(start, end, precision, &color, zoom);
}

fn run_singular(constant: Complex<f64>, color: &Coloring, zoom: f64) {
    let data = 
        julia(2., constant, &zoom, &color);

    write_png(r".\fractal.png",data)
}


fn run_animation(start: f64, end: f64, precision: f64, color: &Coloring, zoom: f64) {
    
    let mut counter = 0;
    let total : f64= end / precision;

    for i in float_iterator(start, end, precision) {

        // let c  = 0.7885 * Complex::new(0., i).exp();
        let c = 0.7885 * Complex::new(0., 2.989).exp();

        let data = julia(2., c, &i, &color);

        counter += 1;
        let path = format!{r"E:\fractals\steps3\step{}.png", counter};
        
        if counter % 10 == 0 {
            println!{"{} % done, i value {} ",100. * counter as f64 / total, i}
        }
        write_png(&path, data)
    }
}

fn write_png(path: &str, data: Vec<u8>) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, x_dim as u32, y_dim as u32); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data.as_slice()).unwrap(); // Save

}


fn float_iterator(start: f64, end: f64, precision: f64) ->impl Iterator<Item=f64>{
    let start = (start / precision) as u64;
    let end = (end / precision) as u64;

    dbg!{start}; dbg!{end};


    (end..start).map(move |x| x as  f64 * precision).into_iter()            // end < start
    // (start..end).map(move |x| x as  f64 * precision).into_iter()         // start < end

}

const x_dim : usize = 1920;
const y_dim : usize = 1080;

const x_dim_f: f64 = x_dim as f64;
const y_dim_f: f64 = y_dim as f64;

const max_iterations: u32  = 1000;

const center: f64 = 135.;
const delta : f64 = 25.;

struct Coloring {
    red: Config,
    green: Config,
    blue: Config,
}
impl Coloring {
    fn new(x: Config, y: Config, z: Config) -> Self{
        Coloring{
            red: x,
            green: y,
            blue: z
        }
    }
}


struct Config {
    frequency: f64,
    phase: f64,
    delta: f64
}
impl Config{
    fn new(freq: f64, phase: f64, center_: f64) -> Self{
        Config{
            frequency: freq,
            phase: phase,
            delta: center_
        }
    }
}



fn julia(power: f64, c: Complex<f64>, zoom: &f64, colors: &Coloring) -> Vec<u8>{

    let mut color_vec : Vec<u8>= Vec::with_capacity(x_dim * y_dim * 3);

    (0..x_dim).into_iter().collect::<Vec<_>>().par_iter().map(move |x|{
        (0..y_dim).into_iter().map(|y|{

            let _x = *x as f64;
            let _y = y as f64;

            let x_center = x_dim_f / 2.;
            let y_center = y_dim_f / 2.;

            let offset_x = _x- x_center;
            let offset_y = _y - y_center;

            let mut zx = offset_x * 2. / (x_dim_f * zoom);
            let mut zy =offset_y * 2. / (y_dim_f * zoom);

            let mut iteration = 0;

            while ((zx * zx) + (zy * zy))< 4. && iteration < max_iterations {
                
                let x_temp = (zx * zx) - (zy * zy);
                zy = (2. * zx * zy) + c.im;
                zx = x_temp + c.re;

                iteration += 1;
            }

            if iteration == max_iterations{     // did not diverge
                (0u8 , 0u8, 0u8)
            } else {                            //diverges

                /*
                 
                    Color calculation:
                 
                 */

                let iteration = iteration as f64;
                let abs_z = ((zx * zx) + (zy * zy)).sqrt();

                let z_ = Complex::new(zx, zy);

                let diff = iteration + 1. - ((abs_z.log(10.).log(10.))/2f64.log(10.));
                // let diff = iteration + 1. - z_.norm().log(2.).log(10.);
                
                // let iteration = iteration + iteration.log(1.5);
                
                let r = center- (3.*diff.sin().abs() * colors.red.delta);
                let g = center - (1.*diff.sin().abs() * colors.green.delta); 
                let b = center - (3.*diff.sin().abs() * colors.red.delta);

                let r = (center * (diff * colors.red.frequency + colors.red.phase).sin()) + colors.red.delta;
                let g = (center * (diff * colors.green.frequency + colors.green.phase).sin()) + colors.green.delta;
                let b = (center * (diff * colors.blue.frequency + colors.blue.phase).sin()) + colors.blue.delta;

                (r as u8, g as u8, b as u8)

            }
        }).collect::<Vec<_>>()
    })
    .map(|vec|{
        let mut inner:  Vec<u8> = Vec::new();
        for i in vec {
            push_vals(&mut inner, i)
        }
        inner
    })
    .flatten()
    .collect::<Vec<_>>()

}


fn push_vals<T>(vec: &mut Vec<T>, vals :(T, T, T)) {
    vec.push(vals.0);
    vec.push(vals.1);
    vec.push(vals.2);
}


fn zero_pad_directory(input_dir: &str) {
    let dir = std::fs::read_dir(input_dir);

    for file in dir{
        for f_ in file {
            let file = f_.unwrap();
            let path = file.path();
            let name = path.file_name().unwrap().to_str().unwrap();


            let max_zeros = 6;
            let len = name.len();

            let slice = name.get(4..len-4).unwrap();
            let slice_len = slice.len();
            let slice_num: u64 = slice.parse().unwrap();
            
            let actual_zeros = max_zeros - slice_len;
            // let mut zero_str = String::with_capacity(5);
            // std::iter::repeat(0).take(actual_zeros).for_each(|x| zero_str.push_str(&x.to_string()));

            let zero_str: String = std::iter::repeat(0).take(actual_zeros).map(|x| x.to_string()).collect();

            let new_path = format!{"step{}{}.png", zero_str, slice_num};
            
            let mut base_path = input_dir.clone().to_string();
            base_path.push_str(r"\");
            base_path.push_str(&new_path);

            println!{"name: {:?} new path {:?}", path, base_path}
            
            std::fs::rename(path, base_path);
        }



    }
}