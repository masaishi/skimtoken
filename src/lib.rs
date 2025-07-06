#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use std::path::Path;

// Import modules
mod methods {
    pub mod method;
    pub mod method_basic;
    pub mod method_multilingual;
    pub mod method_simple;
}

// Re-export for convenience
pub use methods::method::EstimationMethod;
pub use methods::method_basic::{BasicMethod, BasicParameters};
pub use methods::method_multilingual::{MultilingualMethod, MultilingualMethodParameters};
pub use methods::method_simple::{SimpleMethod, SimpleParameters};

// Enum for selecting estimation method
#[derive(Debug, Clone, Default)]
pub enum Method {
    #[default]
    Simple,
    Basic,
    Multilingual,
}

// Main estimation function - uses simple method by default
pub fn estimate_tokens(text: &str) -> usize {
    let mut estimator = SimpleMethod::new();
    // Try to load parameters from file, fallback to default if failed
    let _ = estimator.load_parameters(Path::new("params/simple.toml"));
    estimator.estimate(text)
}

// Python bindings
#[cfg(feature = "pyo3")]
#[pymodule]
fn _skimtoken_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Main estimation function
    #[pyfn(m)]
    #[pyo3(name = "estimate_tokens")]
    fn estimate_tokens_py(text: &str) -> PyResult<usize> {
        Ok(estimate_tokens(text))
    }

    // Simple method estimation
    #[pyfn(m)]
    #[pyo3(name = "estimate_tokens_simple")]
    fn estimate_tokens_simple_py(text: &str) -> PyResult<usize> {
        let mut estimator = SimpleMethod::new();
        let _ = estimator.load_parameters(Path::new("params/simple.toml"));
        Ok(estimator.estimate(text))
    }

    // Basic method estimation
    #[pyfn(m)]
    #[pyo3(name = "estimate_tokens_basic")]
    fn estimate_tokens_basic_py(text: &str) -> PyResult<usize> {
        let mut estimator = BasicMethod::new();
        let _ = estimator.load_parameters(Path::new("params/basic.toml"));
        Ok(estimator.estimate(text))
    }

    // Multilingual method estimation
    #[pyfn(m)]
    #[pyo3(name = "estimate_tokens_multilingual")]
    fn estimate_tokens_multilingual_py(text: &str) -> PyResult<usize> {
        let mut estimator = MultilingualMethod::new();
        let _ = estimator.load_parameters(Path::new("params/multilingual.toml"));
        Ok(estimator.estimate(text))
    }

    // Feature extraction functions for optimization
    #[pyfn(m)]
    #[pyo3(name = "count_simple")]
    fn count_simple_py(text: &str) -> PyResult<usize> {
        let estimator = SimpleMethod::new();
        Ok(estimator.count(text))
    }

    #[pyfn(m)]
    #[pyo3(name = "count_basic")]
    fn count_basic_py(text: &str) -> PyResult<(usize, usize, f64, usize)> {
        let estimator = BasicMethod::new();
        let features = estimator.count(text);
        Ok((
            features.char_count,
            features.word_count,
            features.avg_word_length,
            features.space_count,
        ))
    }

    #[pyfn(m)]
    #[pyo3(name = "count_multilingual")]
    fn count_multilingual_py(text: &str) -> PyResult<(usize, usize, f64, usize, String)> {
        let estimator = MultilingualMethod::new();
        let features = estimator.count(text);
        Ok((
            features.basic_features.char_count,
            features.basic_features.word_count,
            features.basic_features.avg_word_length,
            features.basic_features.space_count,
            features.language,
        ))
    }

    // Language detection function
    #[pyfn(m)]
    #[pyo3(name = "detect_language")]
    fn detect_language_py(text: &str) -> PyResult<String> {
        use whatlang::detect;
        let language = detect(text)
            .map(|info| info.lang().code())
            .unwrap_or("unknown")
            .to_string();
        Ok(language)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens() {
        let text = "Hello, world!";
        let result = estimate_tokens(text);
        assert!(result > 0);
    }

    #[test]
    fn test_estimate_tokens_longer() {
        let text = "The quick brown fox jumps over the lazy dog.";
        let result = estimate_tokens(text);
        assert!(result > 0);
    }

    #[test]
    fn test_estimate_tokens_multilingual() {
        let text = "This is an English text.";
        let result = estimate_tokens(text);
        assert!(result > 0);

        let text_jp = "これは日本語のテキストです。";
        let result_jp = estimate_tokens(text_jp);
        assert!(result_jp > 0);
    }
}
