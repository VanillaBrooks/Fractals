
use num::Complex;
use super::utils::FractalStep;

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
    fn colorify(&self, step: FractalStep ) -> (u8,u8,u8) {

            if step.iterations == 1000{(0, 0, 0,)}
            else{

                let iteration = step.iterations as f64;
                // let abs_z = ((zx * zx) + (zy * zy)).sqrt();
                let abs_z = step.current_z.norm();


                // let diff = iteration + 1. - ((abs_z.log(10.).log(10.))/2f64.log(10.));
                let diff : f64 = iteration + 1. - abs_z.log(2.).log(10.);

                (0, 0, 0)
            }
    }
}

#[derive(Debug, Copy, Clone)]
struct RGB{
    red: u8,
    green: u8,
    blue: u8
}
impl RGB {
    pub fn new(r: u8,g: u8,b: u8) -> Self{
        Self {
            red: r,
            green: g,
            blue: b
        }
    }
}


pub struct Coloring2{
    color_map: Vec<RGB> 
}

impl Coloring2{
    pub fn rbg_gradient() -> Self {
        let mut r = 255;
        let mut g = 0;
        let mut b = 0;

        let mut color_vec = Vec::with_capacity(255);


        for i in 0..256{

            let s = RGB::new(r, g, b);
            color_vec.push(s);

            r -= 1;
            b += 1;
            
            if i < 127{
                g+=1;
            }
            else{
                g -= 1;
            }
        }

        Self{
            color_map: color_vec
        }

    }

    
    
    fn interpolate(&self, index: f64) -> &RGB{

        let len = self.color_map.len() as f64;
        let index = self.wrap(len, index);


        let whole = index as u64 as f64;
        let fraction = index - whole;

        if fraction > 0.5 {
            self.color_map.get(whole as usize +1).unwrap()
        } else{ 
            self.color_map.get(whole as usize).unwrap()
        }


        // return self.color_map.get(k)
    }

    fn wrap(&self, len: f64, index: f64) -> f64 {
        if index < len{
            index
        }
        else{
            self.wrap(len, index- len)
        }

    }
}

impl Colorify for Coloring2{
    fn colorify(&self, step: FractalStep) -> (u8, u8, u8) {

        if step.iterations == 10000{
            return (0, 0, 0)
        }


        let iteration = step.iterations as f64;
        let abs_z = step.current_z.norm();


        let diff : f64 = iteration + 1. - abs_z.log(2.).log(10.);

        let colors : &RGB = self.interpolate(diff);

        (colors.red, colors.green, colors.blue)




    }
}


pub trait Colorify{
    fn colorify(&self, step: FractalStep) -> (u8, u8, u8);
}
