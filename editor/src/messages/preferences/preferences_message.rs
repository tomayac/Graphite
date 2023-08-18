use crate::messages::prelude::*;

use graph_craft::imaginate_input::ImaginateServerBackend;

use serde::{Deserialize, Serialize};

#[impl_message(Message, Preferences)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum PreferencesMessage {
	Load { preferences: String },
	ResetToDefaults,

	ImaginateRefreshFrequency { seconds: f64 },
	ImaginateServerHostname { hostname: String },
	ImaginateServerBackend { backend: ImaginateServerBackend },
	ModifyLayout { zoom_with_scroll: bool },
}
