use fractals::coloring;
use fractals::image;
use fractals::run;
use fractals::julia;
use fractals::utils;

use std::f64::consts::PI;
use num::Complex;

fn main() {

    // let start = 0.;
    // let end =  6.283185;
    // let precision = 0.001;

    let start = 0.;
    let end = 2.*PI;
    let precision = 0.01;

    let color_center = 135.;
    let zoom = 0.8;

    let r = coloring::ColorConfig::new(-0.04, 1.5, -70.);
    let g = coloring::ColorConfig::new(0.0613, 0.5, 0.);
    let b = coloring::ColorConfig::new(0.09075, 3.5, 170.);

    let color = coloring::Coloring::new(r, g, b, color_center);

    let img_config = image::ImageConfig::new(1080,1080, zoom, r"E:\fractals\steps6");

    let stepper = utils::Stepper::new(0., 2.*PI, 0.01);

    let rt = run::RunType::Function;

    let const_fn = |x|  0.7885 * Complex::new(0., x).exp();
    // let const_fn = |x| 0.7885 * Complex::new(0., 2.989).exp();



    // let c = 0.7885 * Complex::new(0., 2.989).exp();

    // let data = julia::julia(2., c, &color, &img_config);

    // utils::write_png(r".\fractal.png", &data, &img_config);


    run::run_animation(stepper, &color, img_config, rt, const_fn);

}
