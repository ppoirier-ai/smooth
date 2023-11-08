use rand;
use rand::Rng;

pub fn swap() -> f32 {
    let rng = rand::thread_rng().gen_range(0.0..10000.0);
    rng 
}