use num::{self, Complex};
use super::image;
use super::coloring;
use super::julia;
use super::utils;

pub fn run_animation(
    start: f64, 
    end: f64, 
    precision: f64, 
    color: &coloring::Coloring, 
    mut img: image::ImageConfig,
    constant_eval: impl Fn(f64) -> Complex<f64>
    ) {

    let mut counter = 0;
    let total : f64= (end- start).abs() / precision;

    let iter_vals = float_iterator(start, end, precision);

    for i in iter_vals {

        let c = constant_eval(i);

        img.zoom = i;

        let data = julia::julia(2., c, &color, &img);


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


fn float_iterator(start: f64, end: f64, precision: f64) ->impl Iterator<Item=f64>{
    let start = (start / precision) as u64;
    let end = (end / precision) as u64;

    dbg!{start}; dbg!{end};


    // (end..start).map(move |x| x as  f64 * precision).into_iter()            // end < start
    (start..end).map(move |x| x as  f64 * precision).into_iter()         // start < end

}