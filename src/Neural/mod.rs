pub mod network;
pub mod layer;
// pub mod neuron;
mod matrix;
mod trainingset;

pub use self::network::Network;
pub use self::layer::Layer;
// pub use self::neuron::Neuron;
pub use self::matrix::Matrix;
pub use self::trainingset::TrainingSet;