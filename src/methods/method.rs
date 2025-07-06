use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

/// Trait for token estimation methods
pub trait EstimationMethod {
    /// Type for the features extracted from text
    type Features;

    /// Type for the parameters used by this method
    type Parameters: Serialize + for<'de> Deserialize<'de> + Default;

    /// Extract features from text for optimization
    fn count(&self, text: &str) -> Self::Features;

    /// Estimate token count using extracted features and parameters
    fn estimate(&self, text: &str) -> usize;

    /// Get current parameters
    fn parameters(&self) -> Self::Parameters;

    /// Set parameters
    fn set_parameters(&mut self, params: Self::Parameters);

    /// Load parameters from TOML file
    fn load_parameters(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let params: Self::Parameters = toml::from_str(&content)?;
        self.set_parameters(params);
        Ok(())
    }

    /// Save parameters to TOML file
    fn save_parameters(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let params = self.parameters();
        let content = toml::to_string_pretty(&params)?;
        fs::write(path, content)?;
        Ok(())
    }
}
