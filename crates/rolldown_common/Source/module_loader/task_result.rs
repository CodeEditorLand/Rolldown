use oxc_index::IndexVec;
use rolldown_ecmascript::EcmaAst;
use rolldown_error::BuildDiagnostic;
use rustc_hash::FxHashMap;

use crate::{
	ImportRecordIdx,
	Module,
	ModuleIdx,
	RawImportRecord,
	ResolvedId,
	SymbolRefDbForModule,
	dynamic_import_usage::DynamicImportExportsUsage,
};

pub struct NormalModuleTaskResult {
	pub module_idx:ModuleIdx,
	pub resolved_deps:IndexVec<ImportRecordIdx, ResolvedId>,
	pub raw_import_records:IndexVec<ImportRecordIdx, RawImportRecord>,
	pub warnings:Vec<BuildDiagnostic>,
	pub module:Module,
	pub ecma_related:Option<EcmaRelated>,
}

pub struct EcmaRelated {
	pub ast:EcmaAst,
	pub symbols:SymbolRefDbForModule,
	pub dynamic_import_rec_exports_usage:FxHashMap<ImportRecordIdx, DynamicImportExportsUsage>,
}
