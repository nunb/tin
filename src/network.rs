use layer::Layer;

#[test]
fn network_test() {
    let layers : Vec<usize> = vec![1,2,3];
    let l = Network::new(&layers, 2);
}

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(input_sizes : &Vec<usize>, outputs : usize) -> Network {
        let output_sizes : Vec<usize> = input_sizes.iter().skip(1).chain(vec!(outputs).iter()).cloned().collect();
        let inputs_outputs : Vec<(usize, usize)> = input_sizes.iter().cloned().zip(output_sizes.iter().cloned()).collect();
        return Network {
            layers: inputs_outputs.iter().map(|&(inputs, outputs)| Layer::new(inputs, outputs)).collect()
        }
    }
}
