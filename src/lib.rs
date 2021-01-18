pub use expr::Realizer;
use hashbrown::HashMap;
use rand::{
    distributions::{DistIter, Uniform},
    prelude::{Distribution, ThreadRng},
};

#[derive(Debug, Default)]
pub struct RandomRealizer {
    source: HashMap<i32, BoundedRng>,
}

impl RandomRealizer {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Realizer for RandomRealizer {
    fn next(&mut self, max: i32) -> i32 {
        self.source
            .entry(max)
            .or_insert_with(|| BoundedRng::new(max))
            .next()
    }
}

#[derive(Debug)]
struct BoundedRng(DistIter<Uniform<i32>, ThreadRng, i32>);

impl BoundedRng {
    fn new(max: i32) -> Self {
        BoundedRng(Uniform::from(1..=max).sample_iter(rand::thread_rng()))
    }

    fn next(&mut self) -> i32 {
        self.0.next().unwrap()
    }
}
