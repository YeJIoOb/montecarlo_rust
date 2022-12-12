use rand::{ Rng, rngs::ThreadRng };

pub struct Dice {
    pub chance: f32,
    rng: ThreadRng,
}

impl Dice {
    pub fn new_with(chance: f32, rng: ThreadRng) -> Self {
        Self {
            chance,
            rng
        }
    }
    pub fn check(&mut self) -> u8 {
        if self.rng.gen_range(0.0..1.0) < self.chance {
            return 1;
        }
        0
    }
}
