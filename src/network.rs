use layer::Layer;

struct Network {
    layers: Vec<Layer>,
    input_layer: Box<Layer>,
    output_layer: Box<Layer>,
}

