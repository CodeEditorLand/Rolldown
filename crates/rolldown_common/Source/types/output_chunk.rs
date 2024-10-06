use arcstr::ArcStr;
use rolldown_sourcemap::SourceMap;
use rustc_hash::FxHashMap;

use super::rendered_module::RenderedModule;
use crate::ModuleId;

#[derive(Debug)]
pub struct OutputChunk {
	// PreRenderedChunk
	pub name:ArcStr,
	pub is_entry:bool,
	pub is_dynamic_entry:bool,
	pub facade_module_id:Option<ModuleId>,
	pub module_ids:Vec<ModuleId>,
	pub exports:Vec<String>,
	// RenderedChunk
	pub filename:ModuleId,
	pub modules:FxHashMap<ModuleId, RenderedModule>,
	pub imports:Vec<ModuleId>,
	pub dynamic_imports:Vec<ModuleId>,
	// OutputChunk
	pub code:String,
	pub map:Option<SourceMap>,
	pub sourcemap_filename:Option<String>,
	pub preliminary_filename:String,
}
