#![allow(non_snake_case)]

pub use super::Layer;
pub use super::TrainingSet;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Rc<RefCell<Layer>>>,
    // trainingSet: TrainingSet,
    trainingSetBatchSize: u32,
}

impl Network {
    pub fn new() -> Network {
        Network {
            layers: Vec::new(),
            // trainingSet: TrainingSet,
            trainingSetBatchSize: 0,
        }
    }

    pub fn Init(&mut self, inputNeuronCount: u32, trainingSetBatchSize: u32, outputNeuronSize: u32) {
        if self.layers.len() != 0 {
            panic!("neural net has an input layer already");
        }

        self.trainingSetBatchSize = trainingSetBatchSize;

        let mut inputLayer = Layer::new(inputNeuronCount);
        inputLayer.SetActivationMatrix(inputNeuronCount, trainingSetBatchSize);

        self.layers.push(Rc::new(RefCell::new(inputLayer)));
    }

    pub fn AddLayer(&mut self, neuronCount: u32) {
        if self.layers.len() < 1 {
            panic!("An input layer must be present before adding hidden layers, please call Init()")
        }

        let lastLayer = self.layers.last().unwrap();
        let hiddenLayer = &Rc::new(RefCell::new(Layer::new(neuronCount)));

        hiddenLayer.borrow_mut().SetPrevLayer(Rc::clone(lastLayer), self.trainingSetBatchSize);
        lastLayer.borrow_mut().SetNextLayer(Rc::clone(hiddenLayer));

        self.layers.push(Rc::clone(hiddenLayer));
    }

    pub fn FeedForward(&mut self) {
        if self.layers.len() < 1 {
            panic!("cannot feed forward, there are not enough layers");
        }
        self.layers[0].borrow_mut().FeedForward();
    }

    pub fn BackProp() {}
}
