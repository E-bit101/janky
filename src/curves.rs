pub fn steering(x: f64) -> f64 {
    return x*x*x;
}

pub fn turn_speed(x: f64) -> f64{
    return 0.75*x*x + 0.25;
}