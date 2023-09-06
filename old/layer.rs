pub use super::neuron::Neuron;
use std::rc::Rc;

pub struct LayerOld {
    pub neurons: Vec<Neuron>,
    lastLayer: Rc<Layer>,
    nextLayer: Rc<Layer>,
}

impl LayerOld {
    pub fn new(size: i32, lastLayer: Rc<Layer>, nextLayer: Rc<Layer>) -> Layer {
        // println!("LSize: {}", size);

        let mut tempNeurons: Vec<Neuron> = Vec::new();

        for i in 0..size {
            tempNeurons.push(Neuron::new(lastLayerSize));
        }
        
        return Layer {
            neurons: tempNeurons
        }
    }

    pub fn propagate(&mut self) {
        for i in 0..self.neurons.len() {
            // println!("Neuron Prop: #{}", i);
            self.neurons[i].propagate(&self.lastLayer.neurons);
        }
    }

    pub fn backpropagate(&mut self) {
        for i in 0..self.neurons.len() {
            // println!("Neuron Prop: #{}", i);
            self.neurons[i].backpropagate(&mut self.lastLayer.neurons);
        }
    }

    pub fn print(&self) {
        for neuron in &self.neurons {
            neuron.print();
        }
        println!("");
    }
}