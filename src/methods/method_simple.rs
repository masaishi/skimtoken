use super::method::EstimationMethod;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleParameters {
    pub coefficient: f32,
}

impl Default for SimpleParameters {
    fn default() -> Self {
        Self {
            coefficient: 0.329_268_3,
        }
    }
}

pub struct SimpleMethod {
    parameters: SimpleParameters,
}

impl SimpleMethod {
    pub fn new() -> Self {
        Self {
            parameters: SimpleParameters::default(),
        }
    }
}

impl Default for SimpleMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl EstimationMethod for SimpleMethod {
    type Features = usize; // Just character count
    type Parameters = SimpleParameters;

    fn count(&self, text: &str) -> Self::Features {
        text.chars().count()
    }

    fn estimate(&self, text: &str) -> usize {
        let char_count = self.count(text);
        (char_count as f32 * self.parameters.coefficient).round() as usize
    }

    fn parameters(&self) -> Self::Parameters {
        self.parameters.clone()
    }

    fn set_parameters(&mut self, params: Self::Parameters) {
        self.parameters = params;
    }
}
