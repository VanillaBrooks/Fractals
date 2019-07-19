
type float = f64;
type int = usize;

pub struct ImageConfig <'a>{
    pub x_dim: int,
    pub y_dim: int,
    pub x_dim_f: float,
    pub y_dim_f: float,
    pub zoom: float,
    pub save_location: &'a str
}

impl <'a>ImageConfig <'a> {
    pub fn new(x_dim: int, y_dim: int, zoom: float, folder: &'a str) -> Self{
        Self{
            x_dim: x_dim,
            y_dim: y_dim,
            x_dim_f: x_dim as float,
            y_dim_f: y_dim as float,
            zoom: zoom,
            save_location: folder
        }
    }
}