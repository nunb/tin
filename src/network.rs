use layer::Layer;

#[test]
fn network_test() {
    let layers : Vec<usize> = vec![1,2,3];
    let mut l = Network::new(&layers);
}

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(input_sizes : &Vec<usize>) -> Network {
        let mut output_sizes : Vec<usize> = input_sizes.iter().cloned().skip(1).collect();
        output_sizes.push(0);
        let inputs_outputs : Vec<(usize, usize)> = input_sizes.iter().cloned().zip(output_sizes.iter().cloned()).collect();
        println!("{:?}", inputs_outputs);
        return Network {
            layers: inputs_outputs.iter().map(|&(inputs, outputs)| Layer::new(inputs, outputs)).collect()
        }
    }
}
