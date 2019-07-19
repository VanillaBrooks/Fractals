use num::complex::Complex;
use super::coloring::Coloring;
use super::image;


use rayon::{self, prelude::*};

const MAX_ITERATIONS : u32 = 1000;

pub fn julia(
    power: f64, 
    c: Complex<f64>, 
    colors: &Coloring,
    img: &image::ImageConfig
    ) -> Vec<u8>{

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

            if iteration == MAX_ITERATIONS{     // did not diverge
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


                let r = (colors.center * (diff * colors.red.frequency + colors.red.phase).sin()) + colors.red.delta;
                let g = (colors.center * (diff * colors.green.frequency + colors.green.phase).sin()) + colors.green.delta;
                let b = (colors.center * (diff * colors.blue.frequency + colors.blue.phase).sin()) + colors.blue.delta;

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
