use serde::Deserialize;

use crate::config_types::ReasoningEffort;
use crate::config_types::ReasoningSummary;
use crate::protocol::AskForApproval;

/// Collection of common configuration options that a user can define as a unit
/// in `config.toml`.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct ConfigProfile {
    pub model: Option<String>,
    /// The key in the `model_providers` map identifying the
    /// [`ModelProviderInfo`] to use.
    pub model_provider: Option<String>,
    pub approval_policy: Option<AskForApproval>,
    pub disable_response_storage: Option<bool>,
    pub model_reasoning_effort: Option<ReasoningEffort>,
    pub model_reasoning_summary: Option<ReasoningSummary>,
}
