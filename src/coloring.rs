



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
