use num::complex::Complex;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
// To use encoder.set()
use png::HasParameters;

use rayon::{self, prelude::*};

fn main() {

    let r = Config::new(30.116, 1.);
    let g = Config::new(3454.013, 2.);
    let b = Config::new(456.01, 5.);

    let color = Coloring::new(r, g, b);


// c = -0.54 + 0.54i
//  c = 0.355 + 0.355i 
//  c = 0.37 + 0.1i 
    let c = Complex::new(-0.4, -0.59);
    // let c = Complex::new(-0.7269 , 0.1889);
    // let c = Complex::new(0., 0.);

    let data = julia(2., c,  color);


    let path  = r".\data3.png";
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, x_dim as u32, y_dim as u32); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data.as_slice()).unwrap(); // Save
}

const x_dim : usize = 60_000;
const y_dim : usize = x_dim;

const x_dim_f: f64 = x_dim as f64;
const y_dim_f: f64 = y_dim as f64;

const zoom : f64 = 0.8;

const max_iterations: u32  = 1000;

const center: f64 = 50.;
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
    phase: f64
}
impl Config{
    fn new(freq: f64, phase: f64) -> Self{
        Config{
            frequency: freq,
            phase: phase
        }
    }
}



fn julia(power: f64, c: Complex<f64>, colors: Coloring) -> Vec<u8>{

    let mut color_vec : Vec<u8>= Vec::with_capacity(x_dim * y_dim * 3);

    (0..x_dim).into_iter().collect::<Vec<_>>().par_iter().map(move |x|{
        (0..y_dim).into_iter().map(|y|{
            // let _x = *x as f64*1.;
            let _x = *x as f64*1.0;
            let _y = y as f64*1.0;

            let x_center = x_dim_f / 2.;
            let y_center = y_dim_f / 2.;

            let offset_x = _x- x_center;
            let offset_y = _y - y_center;


            // let mut zx = (offset_x * 3.5 / x_dim_f) - 0.75;
            let mut zx = offset_x * 2. * zoom / x_dim_f;
            let mut zy =offset_y * 2. * zoom / y_dim_f;

            // assert!{zx <= 1.}
            // assert!{zx >= -2.5}
            // assert!{zy <= 1.}
            // assert!{zy >= -1.}

            let mut iteration = 0;

            while ((zx * zx) + (zy * zy))< 4. && iteration < max_iterations {
                // println!{"zx {} zy {}", zx, zy}
                
                let x_temp = (zx * zx) - (zy * zy);
                zy = (2. * zx * zy) + c.im;
                zx = x_temp + c.re;

                iteration += 1;
            }
            // println!{"exited with iterations = {}", iteration}


            // did not diverge
            if iteration == max_iterations{     // did not diverge
                (0u8 , 0u8, 0u8)
            } else {                            //diverges

                let abs_z = ((zx * zx) + (zy * zy)).sqrt();
                let diff = (iteration as f64) + 1. - ((abs_z.log(10.).log(10.))/2f64.log(10.));

                let iteration = iteration as f64;
                // let iteration = iteration + iteration.log(1.5);
                
                let r = 1.* diff;
                let g = diff * 2. ; 
                let b = diff *3.;

                // dbg!{r}; dbg!{b}; dbg![g]; dbg!{diff};

                (r as u8, g as u8, b as u8)
                // (iteration as u8, iteration as u8, iteration as u8)
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
