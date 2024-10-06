use arcstr::ArcStr;

use crate::{events::BuildEvent, DiagnosticOptions, EventKind};

#[derive(Debug)]
pub struct IllegalIdentifierAsName {
	pub(crate) identifier_name:ArcStr,
}

impl BuildEvent for IllegalIdentifierAsName {
	fn kind(&self) -> EventKind { EventKind::IllegalIdentifierAsName }

	fn message(&self, _opts:&DiagnosticOptions) -> String {
		format!(
			r#"Given name "{}" is not a legal JS identifier. If you need this, you can try "output.extend: true"."#,
			self.identifier_name
		)
	}
}
