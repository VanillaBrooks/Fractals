use num::complex::Complex;
use super::coloring::Colorify;
use super::image;


use rayon::{self, prelude::*};

const MAX_ITERATIONS : u32 = 1000;

pub fn julia<T:Colorify + Sync+ Send>(
    power: f64, 
    c: &Complex<f64>, 
    colors: &T,
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

            let z = Complex::new(zx, zy);

            colors.colorify(iteration, z)

            
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
