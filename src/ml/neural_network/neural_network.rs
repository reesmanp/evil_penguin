use crate::{
    ml::neural_network::Neuron,
    util::types::{
        InstanceInput,
        NeuralNetworkLayer,
        ProblemSet
    }
};

use rand::Rng;

pub struct NeuralNetwork {
    pub layers: Vec<NeuralNetworkLayer>,
    test_instances: usize,
    tests_correct: usize,
    pub update_bias: bool,
    pub mse: f32
}

impl NeuralNetwork {
    pub fn new(layout: Vec<usize>, learning_rate: f32, feature_amount: usize) -> Self {
        let mut layers = Vec::with_capacity(layout.len());
        let mut rng = rand::thread_rng();
        // The initial input is the features themselves
        let mut next_input_size = feature_amount;

        // Find the size of each layer
        for node_amount in layout {
            // Auto generate a bias [0.0, 1.0]
            let bias = rng.gen();
            let mut layer = Vec::with_capacity(node_amount);

            // Instantiate the correct amount of neurons for this layer
            for _ in 0..node_amount {
                // Make sure the amount of weights equals the size of the input to this layer
                let mut weights = Vec::with_capacity(next_input_size);

                // Auto generate weights [0.0, 1.0]
                for _ in 0..next_input_size {
                    weights.push(rng.gen());
                }

                // Instantiate neuron
                let node = Neuron::new(weights, bias, learning_rate);
                layer.push(node);
            }

            // Add layer to network along with its associated generated bias value
            layers.push(layer);
            // The size of the next input is the size of this layers output
            next_input_size = node_amount;
        }

        Self {
            layers,
            test_instances: 0,
            tests_correct: 0,
            update_bias: true,
            mse: 0.0
        }
    }

    fn feed_forward(&mut self, instance: InstanceInput) {
        let mut this_input = instance;

        for nodes in &mut self.layers {
            let mut next_input = Vec::with_capacity(nodes.len());

            for node in nodes {
                node.calc_activity(this_input.clone());
                node.calc_activation();

                // The output of this node is an input to the next layer
                next_input.push(node.activation);
            }

            // The outputs of this layer is the inputs of the next layer
            this_input = next_input;
        }
    }

    fn back_propagation(&mut self, expected_output: f32) {
        let mut is_output_layer = true;
        let mut error_sum_vec: Vec<f32>;
        let mut next_error_sum_vec: Vec<f32> = vec![];

        // For every layer, working backwards
        for layer in &mut self.layers.iter_mut().rev() {
            error_sum_vec = next_error_sum_vec;
            next_error_sum_vec = Vec::with_capacity(layer.len());

            // Make sure the vector has enough elements
            for _ in 0..layer[0].get_weights().len() {
                next_error_sum_vec.push(0.0);
            }

            for node in layer {
                if is_output_layer {
                    node.calc_delta_weights(expected_output - node.activation);
                } else {
                    node.calc_delta_weights(error_sum_vec.remove(0));
                }

                // Determine the appropriate error value for the next iteration
                for index in 0..next_error_sum_vec.len() {
                    next_error_sum_vec[index] += node.weights[index] * node.delta;
                }

                // Find the delta for the weight associated with the bias
                node.calc_delta_bias()
            }

            is_output_layer = false;
        }
    }

    /// All the weights need to be updated at the same time
    /// So here we update the weights based off of the calculated deltas
    fn update_weights(&mut self) {
        for layer in &mut self.layers {
            for node in layer {
                node.update_weights();

                if self.update_bias {
                    node.update_bias();
                }
            }
        }
    }

    // TODO: Figure out later
    // pub fn train(&mut self, training_set: ProblemSet, epochs: usize, is_online: bool) {
    //     for _ in 0..epochs {
    //         for (training_instance, expected_output) in training_set.clone() {
    //             // Feed Forward
    //             self.feed_forward(training_instance);
    //
    //             // Back Propagation
    //             self.back_propagation(expected_output);
    //
    //             // Update weights
    //             if is_online {
    //                 self.update_weights();
    //             }
    //         }
    //
    //         if !is_online {
    //             self.update_weights();
    //         }
    //     }
    // }

    pub fn test(&mut self, test_instance: Vec<f32>, maybe_threshold: Option<f32>) -> Vec<f32> {
        self.feed_forward(test_instance);

        self.layers
            .last()
            .unwrap()
            .iter()
            .map(|neuron| neuron.activation)
            .collect::<Vec<f32>>()
    }

    pub fn print(&self) {
        for layer in &self.layers {
            println!("********************");
            for neuron in layer {
                println!("Bias: {}", neuron.bias);
                println!("{:?}", neuron.get_weights());
            }
            println!("********************");
        }
    }

    pub fn accuracy(&self, is_classification: bool) -> Option<f32> {
        if self.test_instances == 0 {
            None
        } else {
            if is_classification {
                Some((self.tests_correct as f32 / self.test_instances as f32) * 100.0)
            } else {
                Some(self.mse)
            }
        }
    }

    pub fn get_output(&self) -> Vec<f32> {
        self.layers[self.layers.len() - 1].iter().map(|node| node.activation).collect::<Vec<f32>>()
    }
}
