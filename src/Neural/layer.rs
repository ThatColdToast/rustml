#![allow(non_snake_case)]

use super::matrix::Matrix;
// use super::trainingset;

use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;
use std::option::Option;

#[derive(Debug)]
pub struct Layer {
    activations: Matrix,
    sums: Matrix,
    weights: Matrix,
    bias: Matrix,

    deltaActivations: Matrix,
    deltaSums: Matrix,
    deltaWeights: Matrix,
    deltaBias: Matrix,

    neuronCount: u32,

    prevLayer: Option<Rc<RefCell<Layer>>>,
    nextLayer: Option<Rc<RefCell<Layer>>>,
}

impl Layer {
    pub fn new(neuronCount: u32) -> Layer {
        return Layer {
            activations:      Matrix::new(0, 0),
            sums:             Matrix::new(0, 0),
            weights:          Matrix::new(0, 0),
            bias:             Matrix::new(0, 0),

            deltaActivations: Matrix::new(0, 0),
            deltaSums:        Matrix::new(0, 0),
            deltaWeights:     Matrix::new(0, 0),
            deltaBias:        Matrix::new(0, 0),

            neuronCount:      neuronCount,
            prevLayer:        None,
            nextLayer:        None,
        }
    }

    pub fn SetActivationMatrix(&mut self, neuronCount: u32, batchSize: u32) {
        self.activations = Matrix::new(neuronCount, batchSize);
    }

    pub fn SetPrevLayer(&mut self, prevLayer: Rc<RefCell<Layer>>, trainingSetBatchSize: u32) {
        self.activations = Matrix::new(self.neuronCount, trainingSetBatchSize);
        self.deltaActivations = Matrix::new(self.neuronCount, trainingSetBatchSize);

        self.sums = Matrix::new(self.neuronCount, trainingSetBatchSize);
        self.deltaSums = Matrix::new(self.neuronCount, trainingSetBatchSize);

        // self.weights = Matrix::new(self.neuronCount, Rc::clone(&prevLayer));
        // self.weights = Matrix::new(self.neuronCount, prevLayer.as_ref().borrow().neuronCount);
        self.weights = Matrix::new(self.neuronCount, prevLayer.as_ref().borrow().neuronCount);
        self.weights.Randomize();
        self.deltaWeights = Matrix::new(self.neuronCount, prevLayer.as_ref().borrow().neuronCount);

        self.bias = Matrix::new(self.neuronCount, 1);
        self.deltaBias = Matrix::new(self.neuronCount, 1);

        self.prevLayer = Some(prevLayer);
        // prevLayer.nextLayer = Some(Rc::clone(self));
    }

    pub fn SetNextLayer(&mut self, nextLayer: Rc<RefCell<Layer>>) {
        self.nextLayer = Some(nextLayer);
    }

    pub fn FeedForward(&mut self) {
        self.sums.MUL(&self.weights, &self.prevLayer.unwrap().as_ref().borrow().activations);
        self.sums.ADD_COL_VEC(&self.bias);

        match self.nextLayer {
            Some(layer) => layer.borrow_mut().FeedForward(),
            None => return
        }

        // self.nextLayer.().into_inner().FeedForward();
        // self.nextLayer.unwrap_or(return).as_ref().borrow_mut().FeedForward();
    }
    pub fn BackProp(&mut self) {
        print!("TODO - Impl BackProp");
    }
}