use crate::partial_dual::PartialDual;

pub trait Optimizable<const N: usize> {
    fn loss(&self, vars: [f64; N]) -> PartialDual<N>;
    fn report(&self, i: usize, vars: &[f64; N]) {}
    fn optimize(&self, vars: [f64; N], iterations: usize, learning_rate: f64) -> [f64; N] {
        let mut vars = vars;
        for i in 0..iterations {
            self.report(i, &vars);
            let l = self.loss(vars);
            vars = vars
                .into_iter()
                .zip(l.partial)
                .map(|(var, diff)| var - diff * learning_rate)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
        }
        vars
    }
}
