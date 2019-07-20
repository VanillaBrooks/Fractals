
use num::Complex;

#[derive(Clone)]
pub struct Coloring {
    pub red: ColorConfig,
    pub green: ColorConfig,
    pub blue: ColorConfig,
    pub center: f64
}
impl Coloring {
    pub fn new(x: ColorConfig, y: ColorConfig, z: ColorConfig, color_center: f64) -> Self{
        Coloring{
            red: x,
            green: y,
            blue: z,
            center: color_center
        }
    }
}

#[derive(Clone)]
pub struct ColorConfig {
    pub frequency: f64,
    pub phase: f64,
    pub delta: f64
}
impl ColorConfig{
    pub fn new(freq: f64, phase: f64, center_: f64) -> Self{
        ColorConfig{
            frequency: freq,
            phase: phase,
            delta: center_
        }
    }
}

impl Colorify for Coloring {
    fn colorify(&self, iteration_count: u32, value: Complex<f64> ) -> (u8,u8,u8) {

            if iteration_count == 1000{(0, 0, 0,)}
            else{

                let iteration = iteration_count as f64;
                // let abs_z = ((zx * zx) + (zy * zy)).sqrt();
                let abs_z = value.norm();

                // let z_ = Complex::new(zx, zy);

                // let diff = iteration + 1. - ((abs_z.log(10.).log(10.))/2f64.log(10.));
                let diff : f64 = iteration + 1. - abs_z.log(2.).log(10.);

                if diff == std::f64::NAN{
                    println!{"NAN"}
                }


                let r = (self.center * (diff * self.red.frequency + self.red.phase).sin()) + self.red.delta;
                let g = (self.center * (diff * self.green.frequency + self.green.phase).sin()) + self.green.delta;
                let b = (self.center * (diff * self.blue.frequency + self.blue.phase).sin()) + self.blue.delta;

                (r as u8, g as u8, b as u8)
            }
    }
}

pub trait Colorify{
    fn colorify(&self, iteration_count: u32, value: Complex<f64>) -> (u8, u8, u8);
}
