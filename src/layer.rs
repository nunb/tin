use std::iter;
use neuron::Neuron;

#[test]
fn layer_test() {
    let mut l = Layer::new(4, 3);
    let inputs = vec!(2,3,4);
    l.integrate(&inputs)
}

pub struct Layer {
    inputs : usize,
    outputs : usize,
    neurons : Vec<Neuron>,
}

#[derive(Debug)]
pub enum IntegrateError { InvalidInputLength }

impl Layer {
    pub fn new(inputs : usize, outputs : usize) -> Layer {
        return Layer{
            inputs: inputs,
            outputs: outputs,
            neurons: iter::repeat(Neuron::new(inputs)).take(outputs).collect::<Vec<_>>(),
        }
    }

    pub fn integrate(&mut self, input : &Vec<f32>) -> Result<Vec<f32>, IntegrateError> {
        if input.len() != self.inputs {
            return Err(IntegrateError::InvalidInputLength);
        }
        let output = self.neurons.iter().map(|n| n.integrate(&input)).collect();
        return Ok(output)
    }

}


