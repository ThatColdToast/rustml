pub use super::neuron::Neuron;

pub struct Layer {
    pub neurons: Vec<Neuron>
}

impl Layer {
    pub fn new(size: i32, lastLayerSize: i32) -> Layer {
        // println!("LSize: {}", size);

        let mut tempNeurons: Vec<Neuron> = Vec::new();

        for i in 0..size {
            tempNeurons.push(Neuron::new(lastLayerSize));
        }
        
        return Layer {
            neurons: tempNeurons
        }
    }

    pub fn propagate(&mut self, lastLayer: &Layer) {
        for i in 0..self.neurons.len() {
            // println!("Neuron Prop: #{}", i);
            self.neurons[i].propagate(&lastLayer.neurons)
        }
    }

    pub fn print(&self) {
        for neuron in &self.neurons {
            neuron.print();
        }
        println!("");
    }
}