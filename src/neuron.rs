use std::num::Float;
use std::iter;

#[test]
fn neuron_test() {
    let mut n = Neuron::new(3);
    let i = vec![2.0,2.0,2.0];
    let output = n.integrate(&i);
    assert!(output < 1.0);
    assert!(output >= 0.0);
}

#[derive(Clone)]
pub struct Neuron {
    error: f32,
    weights: Vec<f32>,
    delta: Vec<f32>,
}

impl Neuron {
    pub fn new(size: usize) -> Neuron {
        return Neuron{
            error: 1.0,
            weights: iter::repeat(1.0).take(size).collect::<Vec<_>>(),
            delta: iter::repeat(0.0).take(size).collect::<Vec<_>>(),
        }
    }

    pub fn integrate(&self, input : &Vec<f32>) -> f32 {
        let iter = input.iter();
        let zipiter = iter.zip(self.weights.iter());
        return self.sigmoid(zipiter.fold(0.0, |output, (&i, &w)| output+i*w));
    }

    fn sigmoid(&self, input : f32) -> f32 {
        return 1.0/(1.0 + (-input).exp())
    }
}
