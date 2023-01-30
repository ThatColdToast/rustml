pub use super::layer::Layer;
use std::collections::LinkedList;

pub struct Network {
    layers: LinkedList<Layer>,
}

impl Network {
    pub fn new(sizes: Vec<i32>) -> Network {
        println!("Making Network:");

        // let mut tempLayers: Vec<Layer> = Vec::new();
        let mut tempLayers: LinkedList<Layer> = LinkedList::new();

        // First Input Layer
        tempLayers.push_back(Layer::new(sizes[0], 0));

        for i in 1..sizes.len() {
            tempLayers.push_back(Layer::new(sizes[i], sizes[i-1]));
        }
        
        return Network {
            layers: tempLayers
        }
    }

    pub fn propagate(&mut self) {
        // println!(" - Propagation -");
        for i in 0..self.layers.len()-1 {
            // println!("Layer Prop: #{}", i);
            let lastLayer = self.layers.pop_front().expect("");
            let mut thisLayer = self.layers.pop_front().expect("");

            thisLayer.propagate(&lastLayer);

            self.layers.push_front(thisLayer);
            self.layers.push_back(lastLayer);
        }

        // Move last layer to then end
        let lastLayer = self.layers.pop_front().expect("");
        self.layers.push_back(lastLayer);
    }

    pub fn cost(&self, expected: &Vec<f32>) -> f32 {
        let lastLayer = self.layers.back().expect("no layer found");
        if lastLayer.neurons.len() != expected.len() {
            panic!("expected and actual layer size does not match");
        }

        let mut sum = 0.0;

        for i in 0..lastLayer.neurons.len() {
            let neuronValue = lastLayer.neurons[i].value;
            sum += (neuronValue - expected[i]) * (neuronValue - expected[i]);
        }

        return sum;
    }

    pub fn print(&self) {
        for layer in &self.layers {
            println!("Layer #{}", layer.neurons.len());
            layer.print();
        }
    }
}
