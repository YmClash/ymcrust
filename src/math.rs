use rand::{Rng, thread_rng};


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn random(start:i32, end:i32) ->i32 {
    let mut gen = thread_rng();
    let random = gen.gen_range(start..end);
    return random;

}