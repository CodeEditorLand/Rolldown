use std::path::PathBuf;

use super::BuildEvent;
use crate::types::diagnostic_options::DiagnosticOptions;

#[derive(Debug)]
pub struct UnresolvedImport {
	pub(crate) specifier:String,
	pub(crate) importer:PathBuf,
}

impl BuildEvent for UnresolvedImport {
	fn kind(&self) -> crate::event_kind::EventKind {
		crate::event_kind::EventKind::UnresolvedImport
	}

	fn message(&self, opts:&DiagnosticOptions) -> String {
		format!(
			"Could not resolve {} from {}.",
			self.specifier,
			opts.stabilize_path(&self.importer)
		)
	}
}
