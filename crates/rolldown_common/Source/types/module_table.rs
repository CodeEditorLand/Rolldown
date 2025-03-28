use oxc_index::IndexVec;

use crate::{ExternalModule, ExternalModuleIdx, Module, ModuleIdx};

pub type IndexModules = IndexVec<ModuleIdx, Module>;
pub type IndexExternalModules = IndexVec<ExternalModuleIdx, ExternalModule>;

#[derive(Debug, Default, Clone)]
pub struct ModuleTable {
	pub modules:IndexModules,
}
