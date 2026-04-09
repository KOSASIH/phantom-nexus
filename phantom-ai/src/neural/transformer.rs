//! Phantom Transformer - Custom AI Model
//!
//! Mixture of Experts (MoE) transformer architecture optimized for
//! financial prediction and DeFi strategy optimization.

use ndarray::{Array1, Array2};
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("invalid input dimension: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    #[error("model not trained")]
    NotTrained,
    #[error("training failed: {reason}")]
    TrainingFailed { reason: String },
}

/// Attention head for transformer
#[derive(Clone, Debug)]
pub struct AttentionHead {
    pub query_weights: Array2<f64>,
    pub key_weights: Array2<f64>,
    pub value_weights: Array2<f64>,
    pub d_k: f64,
}

impl AttentionHead {
    pub fn new(d_model: usize, d_k: usize) -> Self {
        let mut rng = rand::thread_rng();
        let scale = (2.0 / (d_model + d_k) as f64).sqrt();

        Self {
            query_weights: Array2::from_shape_fn((d_model, d_k), |_| rng.gen::<f64>() * scale),
            key_weights: Array2::from_shape_fn((d_model, d_k), |_| rng.gen::<f64>() * scale),
            value_weights: Array2::from_shape_fn((d_model, d_k), |_| rng.gen::<f64>() * scale),
            d_k: d_k as f64,
        }
    }

    pub fn forward(&self, input: &Array2<f64>) -> Array2<f64> {
        let q = input.dot(&self.query_weights);
        let k = input.dot(&self.key_weights);
        let v = input.dot(&self.value_weights);

        // Scaled dot-product attention
        let scores = q.dot(&k.t()) / self.d_k.sqrt();
        let attention = softmax_2d(&scores);
        attention.dot(&v)
    }
}

/// Expert module for MoE
#[derive(Clone, Debug)]
pub struct Expert {
    pub weights: Array2<f64>,
    pub bias: Array1<f64>,
    pub specialization: ExpertType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExpertType {
    MarketAnalysis,
    RiskAssessment,
    ArbitrageDetection,
    SentimentAnalysis,
    OnChainAnalytics,
    MacroEconomic,
}

impl Expert {
    pub fn new(input_dim: usize, output_dim: usize, specialization: ExpertType) -> Self {
        let mut rng = rand::thread_rng();
        let scale = (2.0 / input_dim as f64).sqrt();
        Self {
            weights: Array2::from_shape_fn((input_dim, output_dim), |_| rng.gen::<f64>() * scale),
            bias: Array1::from_shape_fn(output_dim, |_| rng.gen::<f64>() * 0.01),
            specialization,
        }
    }

    pub fn forward(&self, input: &Array1<f64>) -> Array1<f64> {
        let output = input.dot(&self.weights) + &self.bias;
        relu(&output)
    }
}

/// Phantom Transformer - Main model
pub struct PhantomTransformer {
    pub attention_heads: Vec<AttentionHead>,
    pub experts: Vec<Expert>,
    pub gating_weights: Array2<f64>,
    pub d_model: usize,
    pub num_experts: usize,
    pub top_k: usize,  // Number of experts per token
    pub trained: bool,
}

impl PhantomTransformer {
    /// Create a new Phantom Transformer
    pub fn new(d_model: usize, num_heads: usize, num_experts: usize, top_k: usize) -> Self {
        let d_k = d_model / num_heads;
        let mut rng = rand::thread_rng();

        let attention_heads = (0..num_heads)
            .map(|_| AttentionHead::new(d_model, d_k))
            .collect();

        let expert_types = [
            ExpertType::MarketAnalysis,
            ExpertType::RiskAssessment,
            ExpertType::ArbitrageDetection,
            ExpertType::SentimentAnalysis,
            ExpertType::OnChainAnalytics,
            ExpertType::MacroEconomic,
        ];

        let experts = (0..num_experts)
            .map(|i| Expert::new(d_model, d_model, expert_types[i % expert_types.len()].clone()))
            .collect();

        let gating_weights = Array2::from_shape_fn((d_model, num_experts), |_| rng.gen::<f64>() * 0.1);

        Self {
            attention_heads,
            experts,
            gating_weights,
            d_model,
            num_experts,
            top_k,
            trained: false,
        }
    }

    /// Forward pass through the transformer
    pub fn forward(&self, input: &Array2<f64>) -> Result<Array2<f64>, ModelError> {
        if input.ncols() != self.d_model {
            return Err(ModelError::DimensionMismatch {
                expected: self.d_model,
                actual: input.ncols(),
            });
        }

        // Multi-head attention
        let mut attention_output = Array2::zeros(input.raw_dim());
        for head in &self.attention_heads {
            attention_output = attention_output + head.forward(input);
        }
        attention_output /= self.attention_heads.len() as f64;

        // Mixture of Experts
        let batch_size = input.nrows();
        let mut output = Array2::zeros((batch_size, self.d_model));

        for i in 0..batch_size {
            let token = attention_output.row(i).to_owned();

            // Gating: select top-k experts
            let gate_scores = token.dot(&self.gating_weights);
            let mut indexed_scores: Vec<(usize, f64)> = gate_scores.iter()
                .enumerate()
                .map(|(idx, &score)| (idx, score))
                .collect();
            indexed_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            // Route to top-k experts
            let mut expert_output = Array1::zeros(self.d_model);
            let mut total_weight = 0.0;

            for &(expert_idx, score) in indexed_scores.iter().take(self.top_k) {
                let weight = score.exp();
                expert_output = expert_output + self.experts[expert_idx].forward(&token) * weight;
                total_weight += weight;
            }

            if total_weight > 0.0 {
                expert_output /= total_weight;
            }

            output.row_mut(i).assign(&expert_output);
        }

        Ok(output)
    }

    /// Self-evolution: mutate model architecture
    pub fn evolve(&mut self) {
        let mut rng = rand::thread_rng();
        // Randomly adjust expert weights (neural architecture search)
        for expert in &mut self.experts {
            let noise = Array2::from_shape_fn(expert.weights.raw_dim(), |_| {
                rng.gen::<f64>() * 0.001
            });
            expert.weights = &expert.weights + &noise;
        }
        log::info!("Model evolved: {} experts mutated", self.experts.len());
    }
}

// Utility functions
fn softmax_2d(input: &Array2<f64>) -> Array2<f64> {
    let max_vals = input.map_axis(ndarray::Axis(1), |row| {
        row.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    });
    let exp_input = input.clone() - &max_vals.insert_axis(ndarray::Axis(1));
    let exp_input = exp_input.mapv(f64::exp);
    let sum_exp = exp_input.sum_axis(ndarray::Axis(1)).insert_axis(ndarray::Axis(1));
    exp_input / sum_exp
}

fn relu(input: &Array1<f64>) -> Array1<f64> {
    input.mapv(|x| x.max(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformer_creation() {
        let model = PhantomTransformer::new(128, 8, 16, 4);
        assert_eq!(model.d_model, 128);
        assert_eq!(model.attention_heads.len(), 8);
        assert_eq!(model.experts.len(), 16);
    }

    #[test]
    fn test_forward_pass() {
        let model = PhantomTransformer::new(64, 4, 8, 2);
        let input = Array2::from_shape_fn((4, 64), |_| rand::random::<f64>());
        let output = model.forward(&input).unwrap();
        assert_eq!(output.shape(), &[4, 64]);
    }

    #[test]
    fn test_dimension_mismatch() {
        let model = PhantomTransformer::new(64, 4, 8, 2);
        let input = Array2::from_shape_fn((4, 32), |_| rand::random::<f64>());
        assert!(model.forward(&input).is_err());
    }
}
