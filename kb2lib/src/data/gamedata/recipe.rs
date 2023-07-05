use std::ops::Mul;

use crate::prelude::Dump;


#[derive(Debug, Clone)]
pub struct Recipe {
    inputs: Vec<(usize, u64)>,
    outputs: Vec<(usize, u64)>
}

impl Mul<u64> for &Recipe {
    type Output = Recipe;

    fn mul(self, rhs: u64) -> Self::Output {
        let new_inps = self.inps().iter().map(|(id, q)| (*id, q*rhs)).collect();
        let new_outs = self.outs().iter().map(|(id, q)| (*id, q*rhs)).collect();

        Recipe::new(new_inps, new_outs)
    }
}

impl Mul<&Recipe> for u64 {
    type Output = Recipe;

    fn mul(self, rhs: &Recipe) -> Self::Output {
        rhs * self
    }
}

impl Recipe {
    pub fn new(inputs: Vec<(usize, u64)>, outputs: Vec<(usize, u64)>) -> Self {
        Self { inputs, outputs }
    }

    pub fn inps(&self) -> &[(usize, u64)] {
        &self.inputs
    }

    pub fn outs(&self) -> &[(usize, u64)] {
        &self.outputs
    }
}

impl Dump for Recipe {
    fn dump(&self) -> String {
        let inps = self.inputs.iter().map(|(id, q)| format!("{id}:{q}")).collect::<Vec<String>>().join(",");
        let outs = self.outputs.iter().map(|(id, q)| format!("{id}:{q}")).collect::<Vec<String>>().join(",");
        format!("{inps}->{outs}")
    }
}