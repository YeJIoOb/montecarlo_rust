use rand::{rngs::ThreadRng, Rng};

pub struct Dice {
    pub chance: f32,
}

pub enum ModifyResult {
    Success(u8),
    Fail,
}

impl Dice {
    pub fn new_with(chance: f32) -> Self {
        Self { chance }
    }
    pub fn check(&self, th_rng: &mut ThreadRng) -> ModifyResult {
        if th_rng.gen_range(0.0..1.0) < self.chance {
            return ModifyResult::Success(1);
        }
        ModifyResult::Fail
    }
}
