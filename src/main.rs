use fractals::coloring;
use fractals::image;
use fractals::run;
use fractals::julia;
use fractals::utils;

use std::f64::consts::PI;
use num::Complex;

fn main() {

    let start = 0.;
    let end =  14.2;
    let precision = 0.015;

    // let start = 0.;
    // let end = 2.*PI;
    // let precision = 0.00072722;

    let color_center = 135.;
    let zoom = 0.8;

    let r = coloring::ColorConfig::new(-0.04, 1.5, -70.);
    let g = coloring::ColorConfig::new(0.0613, 0.5, 0.);         // blue outline
    let b = coloring::ColorConfig::new(0.19075, 3.5, 170.);

    // let r = coloring::ColorConfig::new(-0.9, 0., 100.);
    // let g = coloring::ColorConfig::new(0.113, 0., 100.);           // red ish orange 
    // let b = coloring::ColorConfig::new(0.59075, -0., 100.);

    // let r = coloring::ColorConfig::new(-0.09, 1.7, 70.);
    // let g = coloring::ColorConfig::new(0.0113, 0.5, 200.);           // blue and red, probably best one
    // let b = coloring::ColorConfig::new(0.059075, -1.5, 200.);

    // let r = coloring::ColorConfig::new(-0.07, 1.7, 50.);
    // let g = coloring::ColorConfig::new(0.0113, 0.5, 200.);   
    // let b = coloring::ColorConfig::new(0.059075, -1.5, 200.);

    // let color = coloring::Coloring::new(r, g, b, color_center);
    let color = coloring::Coloring2::rbg_gradient();

    // let c = 0.7885 * Complex::new(0., 2.989).exp();
    let c = Complex::new(-0.835, -0.2321);


    let img_config = image::ImageConfig::new(1080,zoom, r"E:\fractals\steps25");


    let zoom_fn = Box::new(|x:f64| x.exp());
    let rt = run::RunType::Zoom((zoom_fn, c));

    // let const_fn = |x|  0.7885 * Complex::new(0., x).exp();          




    // let data = julia::julia(2., &c, &color, &img_config);

    // utils::write_png(r".\fractal2.png", &data, &img_config);


    let stepper = utils::Stepper::new(start, end, precision);
    run::run_animation(stepper, &color, img_config, rt);

}
