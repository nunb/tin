use std::iter;
use neuron::Neuron;

#[test]
fn layer_test() {
    let mut l = Layer::new(4, 3);
}

pub struct Layer {
    inputs : usize,
    outputs : usize,
    neurons : Vec<Neuron>,
}

impl Layer {
    pub fn new(inputs : usize, outputs : usize) -> Layer {
        return Layer{
            inputs: inputs,
            outputs: outputs,
            neurons: iter::repeat(Neuron::new(inputs)).take(outputs).collect::<Vec<_>>(),
        }
    }

    pub fn integrate(&mut self, input : Vec<f32>) -> Option<Vec<f32>> {
        if input.len() != self.inputs {
            println!("invalid input length");
            return None 
        }
        let mut output = Vec::with_capacity(self.outputs);
        for n in &self.neurons {
            output.push(n.integrate(&input))
        }
        return Some(output)
    }

}


