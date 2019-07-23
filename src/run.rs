use num::{self, Complex};
use super::image;
use super::coloring::{self, Colorify};
use super::julia;
use super::utils;



pub fn run_animation<T:Colorify + Send + Sync>(
    iterable_values: utils::Stepper<f64>,
    color: &T, 
    mut img: image::ImageConfig,
    animation_type: RunType,
    ) {

    let mut counter = 0;

    let mut julia_const;

    let total = iterable_values.max_iterations;

    let mut saver = SaveFile::new(&img.save_location);

    for i in iterable_values {

        // update variables based on what kind of animation we are creating
        match &animation_type{
            RunType::Zoom((z, jc)) => {
                img.zoom = z(i);

                //TODO use mem unint here 
                julia_const = *jc;
            },
            RunType::Function(f) => {
                julia_const = f(i)
            }
        }

        // run julia set calculation
        let data= julia::julia(2., &julia_const, color, &img);

        counter += 1;

        if counter % 10 == 0 {
            println!{"{} % done, i value {} ",100. * counter as f64 / total, i}
        }
        
        // utils::write_png(&path_, data.as_slice(), &img)
        saver.save(data, &img);
    }
}


pub enum RunType{
    Zoom((Box<dyn Fn(f64) -> f64 >, Complex<f64>)),
    Function(Box<dyn Fn(f64)-> Complex<f64>>)
}


struct SaveFile<'a>{
    root_folder: &'a str,
    current_folder_count: u64,
    current_count: u64,
    full_current_folder_path: String
}

impl <'a> SaveFile <'a>{
    fn new(img: &'a str) -> Self {
        let mut x = 
            Self {
                root_folder: img,
                current_count: 0,
                current_folder_count: 0,
                full_current_folder_path: "".to_string()
            };
        x.calc_folder_path(true);
        x
    }

    fn save(&mut self, data: Vec<u8>, img: &image::ImageConfig) {
        
        let mut sub_folder_path = self.calc_folder_path(false);;
        let file_path = format!{"step{}.png", self.calc_file_string()};

        sub_folder_path.push_str(&file_path);

        utils::write_png(&sub_folder_path, data.as_slice(), img)
    }

    fn calc_folder_path(&mut self, init: bool) -> String {
        

        if self.current_count == 9999 || init == true{
            self.current_folder_count += 1;
            self.current_count = 0;
            
            let folder = 
                format!{"sub_steps_{}\\", self.current_folder_count};

            let mut root_clone = self.root_folder.clone().to_string();
            root_clone.push_str(r"\");
            root_clone.push_str(&folder);

            std::fs::create_dir(&root_clone);
            self.full_current_folder_path = root_clone;
        }


        self.full_current_folder_path.clone()

    }

    fn calc_file_string(&mut self) -> String {
        self.current_count += 1;

        const ZEROS : usize = 4;
        
        let number_string = self.current_count.to_string();
        let number_len = number_string.len();
        
        let mut zero_str = std::iter::repeat(0).take(ZEROS-number_len).map(|x| x.to_string()).collect::<String>();

        zero_str.push_str(&number_string);

        return zero_str

    }
}



