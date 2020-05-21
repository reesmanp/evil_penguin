use crate::{
    ml::neural_network::{
        util::sigmoid::{
            sigmoid,
            sigmoid_delta
        }
    },
    util::{
        constants::{
            DEFAULT_LEARNING_RATE,
            DEFAULT_NUM_OF_WEIGHTS
        },
        types::{
            InstanceInput,
            ProblemSet,
            Weights
        }
    }
};

use rand::prelude::*;

pub struct Neuron {
    inputs: Vec<f32>,

    pub weights: Weights,
    pub delta_weights: Weights,

    pub bias: f32,
    delta_bias: f32,

    pub activity: f32,

    learning_rate: f32,

    pub activation: f32,
    pub delta: f32
}

impl Neuron {
    pub fn new(weights: Weights, bias: f32, learning_rate: f32) -> Self {
        let mut delta_weights = weights.clone();
        for index in 0..weights.len() {
            delta_weights[index] = 0.0;
        }

        Self {
            inputs: vec![],
            weights,
            delta_weights,
            bias,
            delta_bias: 0.0,
            activity: 0.0,
            activation: 0.0,
            delta: 0.0,
            learning_rate
        }
    }

    pub fn calc_activity(&mut self, input_vec: InstanceInput) {
        let mut activity_value = self.bias;

        for (weight, value) in self.weights.iter().zip(input_vec.iter()) {
            activity_value += weight * value;
        }

        self.activity = activity_value;
        self.inputs = input_vec;
    }

    pub fn calc_activation(&mut self) {
        self.activation = sigmoid(self.activity);
    }

    pub fn calc_delta_weights(&mut self, error_value: f32) {
        self.delta = sigmoid_delta(
            self.activation,
            error_value
        );

        for index in 0..self.weights.len() {
            self.delta_weights[index] = self.learning_rate
                * self.delta
                * self.inputs[index];
        }
    }

    pub fn update_weights(&mut self) {
        for index in 0..self.weights.len() {
            self.weights[index] += self.delta_weights[index];
            self.delta_weights[index] = 0.0;
            self.delta = 0.0;
        }
    }

    /// Assumes delta weights has been calculated first
    pub fn calc_delta_bias(&mut self) {
        self.delta_bias = self.learning_rate * self.delta;
    }

    pub fn update_bias(&mut self) {
        self.bias += self.delta_bias;
        self.delta_bias = 0.0;
    }

    pub fn get_weights(&self) -> &Weights {
        &self.weights
    }
}

impl Default for Neuron {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let mut weight_vec = Vec::with_capacity(DEFAULT_NUM_OF_WEIGHTS);
        for _ in 0..DEFAULT_NUM_OF_WEIGHTS {
            weight_vec.push(rng.gen());
        }

        Self::new(weight_vec, rng.gen(), DEFAULT_LEARNING_RATE)
    }
}
