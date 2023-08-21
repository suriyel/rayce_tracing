use rand::Rng;

pub const PI:f64 = std::f64::consts::PI;
pub const INFINITY:f64 = f64::INFINITY;

pub fn degrees_to_radians(degree:f64)->f64 {
    degree * std::f64::consts::PI / 180.0
}
pub fn ff_min(a: f64, b: f64) -> f64 {
    return if a <= b { a } else { b };
}

pub fn ff_max(a: f64, b: f64) -> f64 {
    return if a >= b { a } else { b };
}

pub fn get_random_double() ->f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double(min:f64,max:f64)->f64 {
    min + (max - min) * get_random_double()
}