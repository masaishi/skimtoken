use super::method::EstimationMethod;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicFeatures {
    pub char_count: usize,
    pub word_count: usize,
    pub avg_word_length: f32,
    pub space_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicParameters {
    pub char_coef: f32,
    pub word_coef: f32,
    pub avg_word_length_coef: f32,
    pub space_coef: f32,
    pub intercept: f32,
}

impl Default for BasicParameters {
    fn default() -> Self {
        Self {
            char_coef: 0.321_774_5,
            word_coef: 0.070_228_82,
            avg_word_length_coef: 0.509_098_2,
            space_coef: -0.158_310_9,
            intercept: 1.591_021,
        }
    }
}

pub struct BasicMethod {
    parameters: BasicParameters,
}

impl BasicMethod {
    pub fn new() -> Self {
        Self {
            parameters: BasicParameters::default(),
        }
    }
}

impl Default for BasicMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl EstimationMethod for BasicMethod {
    type Features = BasicFeatures;
    type Parameters = BasicParameters;

    fn count(&self, text: &str) -> Self::Features {
        let char_count = text.chars().count();
        let space_count = text.chars().filter(|c| c.is_whitespace()).count();
        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len();

        let avg_word_length = if word_count > 0 {
            let total_word_chars: usize = words.iter().map(|w| w.chars().count()).sum();
            total_word_chars as f32 / word_count as f32
        } else {
            0.0
        };

        BasicFeatures {
            char_count,
            word_count,
            avg_word_length,
            space_count,
        }
    }

    fn estimate(&self, text: &str) -> usize {
        let features = self.count(text);
        let estimate = self.parameters.char_coef * features.char_count as f32
            + self.parameters.word_coef * features.word_count as f32
            + self.parameters.avg_word_length_coef * features.avg_word_length
            + self.parameters.space_coef * features.space_count as f32
            + self.parameters.intercept;

        estimate.round().max(0.0) as usize
    }

    fn parameters(&self) -> Self::Parameters {
        self.parameters.clone()
    }

    fn set_parameters(&mut self, params: Self::Parameters) {
        self.parameters = params;
    }
}
