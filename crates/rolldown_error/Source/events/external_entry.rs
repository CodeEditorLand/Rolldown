use std::path::PathBuf;

use super::BuildEvent;
use crate::{event_kind::EventKind, types::diagnostic_options::DiagnosticOptions};

#[derive(Debug)]
pub struct ExternalEntry {
	pub(crate) id:PathBuf,
}

impl BuildEvent for ExternalEntry {
	fn kind(&self) -> crate::event_kind::EventKind { EventKind::UnresolvedEntry }

	fn message(&self, opts:&DiagnosticOptions) -> String {
		format!("Entry module {:?} cannot be external.", opts.stabilize_path(&self.id))
	}
}
