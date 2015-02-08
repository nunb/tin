use layer::Layer;

pub struct Network {
    input_layer: &Layer,
    layers: Vec<Layer>,
    output_layer: &Layer,
}

impl Network {
    pub fn new(layerSizes : &Vec<usize>) -> Network {
        let layers = Vec::with_capacity(layerSizes.len());
        for layerSize in &layerSizes.iter() {
            layers.push(Layer::new(layerSize));
        }
        return Network {
            input_layer : &layers.first(),
            layers: layers,
            output_layer: &layers.last(),
        }
    }
}
