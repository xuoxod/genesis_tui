use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Genome {
    sequence: Vec<u8>,
}

impl Genome {
    pub fn new_random(length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let sequence = (0..length).map(|_| rng.gen::<u8>()).collect();
        Self { sequence }
    }

    pub fn sequence(&self) -> &[u8] {
        &self.sequence
    }

    pub fn mutate(&mut self, rate: f64) {
        let mut rng = rand::thread_rng();
        for byte in self.sequence.iter_mut() {
            if rng.gen::<f64>() <= rate {
                let bit_to_flip = rng.gen_range(0..8);
                *byte ^= 1 << bit_to_flip;
            }
        }
    }
}
