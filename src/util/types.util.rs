use crate::ml::neural_network::Neuron;

pub type SpriteSheetLoadingData<'a> = (&'a str, &'a str, &'a str);

pub type Weights = Vec<f32>;
pub type InstanceInput = Vec<f32>;
pub type ProblemSet = Vec<InstanceInput>;
pub type NeuralNetworkLayer = Vec<Neuron>;
