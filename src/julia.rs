use num::complex::Complex;
use super::coloring::Colorify;
use super::image;
use super::utils;


use rayon::{self, prelude::*};

const MAX_ITERATIONS : u32 = 1000;

macro_rules! push {
    ($vector:ident, $($item:expr),+) => {

        $(
            $vector.push($item);
        )+

    };
}


pub fn julia<T:Colorify + Sync+ Send>(
    power: f64, 
    c: &Complex<f64>, 
    colors: &T,
    img: &image::ImageConfig
    ) -> Vec<u8>{


    let mut steps = 
    (0..img.x_dim).into_iter().collect::<Vec<_>>().par_iter().map(move |x|{
        (0..img.y_dim).into_iter().map(|y|{

            let _x = *x as f64;
            let _y = y as f64;

            let x_center = img.x_dim_f / 2.;
            let y_center = img.y_dim_f / 2.;

            let offset_x = _x- x_center;
            let offset_y = _y - y_center;

            let mut zx = offset_x * 2. / (img.x_dim_f * img.zoom);
            let mut zy =offset_y * 2. / (img.y_dim_f * img.zoom);

            let mut iteration = 0;

            while ((zx * zx) + (zy * zy))< 4. && iteration < MAX_ITERATIONS {
                
                let x_temp = (zx * zx) - (zy * zy);
                zy = (2. * zx * zy) + c.im;
                zx = x_temp + c.re;

                iteration += 1;
            }
            let z = Complex::new(zx, zy);

            utils::FractalStep::new(None, None, iteration, z)

        }).collect::<Vec<_>>()
    })
    .flatten()
    .collect::<Vec<_>>();

    let mut max = 0;
    let mut min = MAX_ITERATIONS;


    // used to find the min and max of all the items 
    for i in 0..(img.x_dim * img.y_dim) {
        let var = 
            unsafe{
                steps.get_unchecked(i)
            };

        if var.iterations > max && var.iterations != MAX_ITERATIONS{
            max = var.iterations
        }
        if var.iterations < min {
            min = var.iterations
        }
    }

    steps.into_par_iter()
        .map(|mut x| {
        
            x.max = Some(&max);
            x.min = Some(&min);

            let c = colors.colorify(x);

            vec![c.0, c.1, c.2]
        })
        .flatten()
        .collect::<Vec<_>>()


}
