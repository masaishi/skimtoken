#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use std::path::Path;

// Import modules
mod methods {
    pub mod method;
    pub mod method_basic;
    pub mod method_multilingual;
    pub mod method_multilingual_simple;
    pub mod method_simple;
}

// Re-export for convenience
pub use methods::method::EstimationMethod;
pub use methods::method_basic::{BasicMethod, BasicParameters};
pub use methods::method_multilingual::{MultilingualMethod, MultilingualMethodParameters};
pub use methods::method_multilingual_simple::{
    MultilingualSimpleMethod, MultilingualSimpleMethodParameters,
};
pub use methods::method_simple::{SimpleMethod, SimpleParameters};

// Enum for selecting estimation method
#[derive(Debug, Clone, Default)]
pub enum Method {
    #[default]
    Simple,
    Basic,
    Multilingual,
    MultilingualSimple,
}

// Main estimation function - uses basic method by default
pub fn estimate_tokens(text: &str) -> usize {
    let mut estimator = MultilingualSimpleMethod::new();
    let _ = estimator.load_parameters(Path::new("params/multilingual_simple.toml"));
    estimator.estimate(text)
}

// Python bindings
#[cfg(feature = "pyo3")]
#[pymodule]
fn _skimtoken_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Main estimation function
    #[pyfn(m)]
    #[pyo3(name = "estimate_tokens")]
    fn estimate_tokens_py(text: &Bound<'_, PyAny>) -> PyResult<usize> {
        // Handle Python strings that may contain invalid UTF-8 sequences
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            // If extraction fails, try to get string representation
            match text.str() {
                Ok(py_str) => {
                    // PyString in PyO3 always returns valid UTF-8 or converts lossy
                    py_str.to_string()
                }
                Err(_) => String::new(),
            }
        };
        Ok(estimate_tokens(&text_str))
    }

    // Simple method estimation
    #[pyfn(m)]
    #[pyo3(name = "estimate_tokens_simple")]
    fn estimate_tokens_simple_py(text: &Bound<'_, PyAny>) -> PyResult<usize> {
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            String::new()
        };
        let mut estimator = SimpleMethod::new();
        let _ = estimator.load_parameters(Path::new("params/simple.toml"));
        Ok(estimator.estimate(&text_str))
    }

    // Basic method estimation
    #[pyfn(m)]
    #[pyo3(name = "estimate_tokens_basic")]
    fn estimate_tokens_basic_py(text: &Bound<'_, PyAny>) -> PyResult<usize> {
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            String::new()
        };
        let mut estimator = BasicMethod::new();
        let _ = estimator.load_parameters(Path::new("params/basic.toml"));
        Ok(estimator.estimate(&text_str))
    }

    // Multilingual method estimation
    #[pyfn(m)]
    #[pyo3(name = "estimate_tokens_multilingual")]
    fn estimate_tokens_multilingual_py(text: &Bound<'_, PyAny>) -> PyResult<usize> {
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            String::new()
        };
        let mut estimator = MultilingualMethod::new();
        let _ = estimator.load_parameters(Path::new("params/multilingual.toml"));
        Ok(estimator.estimate(&text_str))
    }

    // Multilingual simple method estimation
    #[pyfn(m)]
    #[pyo3(name = "estimate_tokens_multilingual_simple")]
    fn estimate_tokens_multilingual_simple_py(text: &Bound<'_, PyAny>) -> PyResult<usize> {
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            String::new()
        };
        let mut estimator = MultilingualSimpleMethod::new();
        let _ = estimator.load_parameters(Path::new("params/multilingual_simple.toml"));
        Ok(estimator.estimate(&text_str))
    }

    // Feature extraction functions for optimization
    #[pyfn(m)]
    #[pyo3(name = "count_simple")]
    fn count_simple_py(text: &Bound<'_, PyAny>) -> PyResult<usize> {
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            String::new()
        };
        let estimator = SimpleMethod::new();
        Ok(estimator.count(&text_str))
    }

    #[pyfn(m)]
    #[pyo3(name = "count_basic")]
    fn count_basic_py(text: &Bound<'_, PyAny>) -> PyResult<(usize, usize, f32, usize)> {
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            String::new()
        };
        let estimator = BasicMethod::new();
        let features = estimator.count(&text_str);
        Ok((
            features.char_count,
            features.word_count,
            features.avg_word_length,
            features.space_count,
        ))
    }

    #[pyfn(m)]
    #[pyo3(name = "count_multilingual")]
    fn count_multilingual_py(
        text: &Bound<'_, PyAny>,
    ) -> PyResult<(usize, usize, f32, usize, String)> {
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            String::new()
        };
        let estimator = MultilingualMethod::new();
        let features = estimator.count(&text_str);
        Ok((
            features.basic_features.char_count,
            features.basic_features.word_count,
            features.basic_features.avg_word_length,
            features.basic_features.space_count,
            features.language,
        ))
    }

    #[pyfn(m)]
    #[pyo3(name = "count_multilingual_simple")]
    fn count_multilingual_simple_py(text: &Bound<'_, PyAny>) -> PyResult<(usize, String)> {
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            String::new()
        };
        let estimator = MultilingualSimpleMethod::new();
        let features = estimator.count(&text_str);
        Ok((features.char_count, features.language))
    }

    // Language detection function
    #[pyfn(m)]
    #[pyo3(name = "detect_language")]
    fn detect_language_py(text: &Bound<'_, PyAny>) -> PyResult<String> {
        let text_str = if let Ok(s) = text.extract::<String>() {
            s
        } else {
            String::new()
        };
        use whatlang::detect;
        let language = detect(&text_str)
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
