pub use super::layer::Layer;
use std::{collections::LinkedList, borrow::BorrowMut};

pub struct Network {
    // layers: LinkedList<Layer>,
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(sizes: Vec<i32>) -> Network {
        println!("Making Network:");

        // let mut tempLayers: Vec<Layer> = Vec::new();
        let mut tempLayers: Vec<Layer> = Vec::new();

        // First Input Layer
        tempLayers.push(Layer::new(sizes[0], 0));

        for i in 1..sizes.len() {
            tempLayers.push(Layer::new(sizes[i], sizes[i-1]));
        }
        
        return Network {
            layers: tempLayers
        }
    }

    // pub fn propagate(&mut self) {
    //     println!(" - Propagation -");
    //     let mut iter = &self.layers.windows(2);
    //     for layers in iter.next() {
    //         layers[0].propagate(&layers[1])
    //     }
    // }

    // pub fn propagate(&mut self) {
    //     // println!(" - Propagation -");
    //     for i in 0..self.layers.len()-1 {
    //         // println!("Layer Prop: #{}", i);
    //         let lastLayer = self.layers.pop_front().expect("no layer found");
    //         let mut thisLayer = self.layers.pop_front().expect("no layer found");

    //         thisLayer.propagate(&lastLayer);

    //         self.layers.push_front(thisLayer);
    //         self.layers.push_back(lastLayer);
    //     }

    //     // Move last layer to then end
    //     let lastLayer = self.layers.pop_front().expect("no layer found");
    //     self.layers.push_back(lastLayer);
    // }

    pub fn backpropagate(&mut self) {
        println!(" - Back Propagation -");
        let mut iter1 = [1, 2, 3, 4, 5, 6].windows(3).borrow_mut();
        for num in iter1 {
            num[0] = 69;
            println!("Nums:{}, {}, {}", num[0], num[1], num[2]);
        }

        let mut iter = self.layers.windows(2).borrow_mut();
        for layers in iter {
            layers[1].backpropagate(&layers[0]) 
        }
    }

    // pub fn backpropagate(&mut self, expected: &Vec<f32>) {
    //     println!(" - Back Propagation -");
    //     let mut lastLayer = self.layers.pop_back().expect("no layer found");
    //     let mut prevLayer = self.layers.pop_back().expect("no layer found");
    // }

    // pub fn backpropagate(&mut self, expected: &Vec<f32>) {
    //     println!(" - Back Propagation -");
    //     let mut lastLayer = self.layers.pop_back().expect("no layer found");
    //     let mut prevLayer = self.layers.pop_back().expect("no layer found");

    //     for i in 0..lastLayer.neurons.len() {
    //         let desiredDelta = expected[i] - lastLayer.neurons[i].value;

    //         lastLayer.neurons[i].backpropagate(&prevLayer.neurons, desiredDelta);
    //     }

    //     self.layers.push_back(prevLayer);
    //     self.layers.push_back(lastLayer);
    // }

    // pub fn cost(&self, expected: &Vec<f32>) -> f32 {
    //     let lastLayer = self.layers.back().expect("no layer found");
    //     if lastLayer.neurons.len() != expected.len() {
    //         panic!("expected and actual layer size does not match");
    //     }

    //     let mut sum = 0.0;

    //     for i in 0..lastLayer.neurons.len() {
    //         let neuronValue = lastLayer.neurons[i].value;
    //         sum += (neuronValue - expected[i]) * (neuronValue - expected[i]);
    //     }

    //     return sum;
    // }

    pub fn print(&self) {
        for layer in &self.layers {
            println!("Layer #{}", layer.neurons.len());
            layer.print();
        }
    }
}
