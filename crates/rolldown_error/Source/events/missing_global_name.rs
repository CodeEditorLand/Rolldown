use arcstr::ArcStr;

use crate::{DiagnosticOptions, EventKind, events::BuildEvent};

#[derive(Debug)]
pub struct MissingGlobalName {
	pub module_name:ArcStr,
	pub guessed_name:ArcStr,
}

impl BuildEvent for MissingGlobalName {
	fn kind(&self) -> EventKind { EventKind::MissingGlobalName }

	fn message(&self, _opts:&DiagnosticOptions) -> String {
		format!(
			r#"No name was provided for external module "{}" in "output.globals" – guessing "{}"."#,
			&self.module_name, &self.guessed_name
		)
	}
}
