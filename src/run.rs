use num::{self, Complex};
use super::image;
use super::coloring::{self, Colorify};
use super::julia;
use super::utils;



pub fn run_animation(
    iterable_values: utils::Stepper<f64>,
    color: &coloring::Coloring, 
    mut img: image::ImageConfig,
    animation_type: RunType,
    constant_eval: impl Fn(f64) -> Complex<f64>
    ) {

    let mut counter = 0;

    // let iter_vals = utils::Stepper::new(start, end, precision);

    let mut julia_const = constant_eval(0.);
    let total = iterable_values.max_iterations;

    for i in iterable_values {

        // update variables based on what kind of animation we are creating
        match animation_type{
            RunType::Zoom => {img.zoom = i;},
            RunType::Function => {julia_const = constant_eval(i)}
        }

        // run julia set calculation
        let data = julia::julia(2., &julia_const, color, &img);

        // save intermediate data
        counter += 1;
        let path = format!{"step{}.png", counter};
        let mut path_ = img.save_location.clone().to_string();
        path_.push_str(r"\");
        path_.push_str(&path);


        if counter % 10 == 0 {
            println!{"{} % done, i value {} ",100. * counter as f64 / total, i}
        }
        
        utils::write_png(&path_, data.as_slice(), &img)
    }
}


pub enum RunType{
    Zoom,
    Function
}