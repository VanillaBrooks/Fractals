use super::image;

use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;

use num;

pub fn zero_pad_directory(input_dir: &str) {
    let dir = std::fs::read_dir(input_dir);

    for file in dir{
        for f_ in file {
            let file = f_.unwrap();
            let path = file.path();
            let name = path.file_name().unwrap().to_str().unwrap();


            let max_zeros = 6;
            let len = name.len();

            let slice = name.get(4..len-4).unwrap();
            let slice_len = slice.len();
            let slice_num: u64 = slice.parse().unwrap();
            
            let actual_zeros = max_zeros - slice_len;
            // let mut zero_str = String::with_capacity(5);
            // std::iter::repeat(0).take(actual_zeros).for_each(|x| zero_str.push_str(&x.to_string()));

            let zero_str: String = std::iter::repeat(0).take(actual_zeros).map(|x| x.to_string()).collect();

            let new_path = format!{"step{}{}.png", zero_str, slice_num};
            
            let mut base_path = input_dir.clone().to_string();
            base_path.push_str(r"\");
            base_path.push_str(&new_path);

            println!{"name: {:?} new path {:?}", path, base_path}
            
            std::fs::rename(path, base_path);
        }



    }
}
pub struct Stepper <T>{
    start: T,
    end: T,
    step_size: T,
    last_val: T,
    pub max_iterations: T
}

impl <T>Stepper <T>
    where T: std::ops::Sub<Output=T> + std::ops::Div<Output=T> +  std::cmp::PartialOrd + Copy + std::ops::Neg<Output=T> + num::Signed
    {
    pub fn new(start: T, end: T, mut step_size: T) -> Self{
        if step_size.is_negative() {
            panic!{"step size should be positive"}
        }


        if end < start{
            step_size = -step_size
        }

        let m_iter = (start-end)/step_size;
        Stepper{
            start: start,
            end: end, 
            step_size: step_size,
            last_val: start,
            max_iterations: m_iter,
        }
    }
}


impl <T> Iterator for Stepper <T>
    where T: std::cmp::PartialOrd + std::ops::Add<Output=T> + Copy +  num::Signed
    {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {

        let new_val = self.last_val + self.step_size;         
        if self.step_size.is_negative(){
            if new_val < self.end {None}
            else {
                self.last_val = new_val;
                Some(new_val)
            }
        }
        else {
            if new_val > self.end {None}
            else {
                self.last_val = new_val;
                Some(new_val)
            }
        }
    
    }
}

pub fn write_png(path: &str, data: &[u8], img: &image::ImageConfig) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, img.x_dim as u32, img.y_dim as u32); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data).unwrap(); // Save

}


use num::Complex;

pub struct FractalStep <'a> {
    pub max: Option<&'a u32>,
    pub min: Option<&'a u32>,
    pub iterations: u32,
    pub current_z: Complex<f64>
}

impl <'a> FractalStep <'a> {
    pub fn new(min: Option<&'a u32>, max: Option<&'a u32>, iterations: u32, current_z: Complex<f64>) -> Self {
        FractalStep {
            max: max,
            min: min,
            iterations:iterations,
            current_z: current_z
        }
    }
}