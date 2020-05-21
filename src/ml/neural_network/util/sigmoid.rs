use std::f64::consts::E;

pub fn sigmoid(activity_value: f32) -> f32 {
    (1.0 / (1.0 + E.powf(-activity_value as f64))) as f32
}

/// error_value
/// desired_output - actual_output ==> output_node
/// Sum(outbound_weight * previous_sigmoid_delta) ==> hidden node
pub fn sigmoid_delta(neuron_activation_value: f32, error_value: f32) -> f32 {
    neuron_activation_value * (1.0 - neuron_activation_value) * error_value
}
