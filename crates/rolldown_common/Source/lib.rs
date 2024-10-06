mod chunk;
mod css;
mod ecmascript;
mod file_emitter;
mod inner_bundler_options;
mod module;
mod type_aliases;
mod types;

/// This module is to help `rolldown` crate could export types related bundler
/// options easily. `rolldown` crate could use `pub use
/// rolldown_common::bundler_options::*;` to export all types, so we don't need
/// write the same code in `rolldown` crate again.
pub mod bundler_options {
	pub use crate::inner_bundler_options::{
		types::{
			advanced_chunks_options::{AdvancedChunksOptions, MatchGroup},
			es_module_flag::EsModuleFlag,
			experimental_options::ExperimentalOptions,
			filename_template::{FileNameRenderOptions, FilenameTemplate},
			inject_import::InjectImport,
			input_item::InputItem,
			is_external::IsExternal,
			module_type::ModuleType,
			normalized_bundler_options::{
				NormalizedBundlerOptions,
				SharedNormalizedBundlerOptions,
			},
			output_exports::OutputExports,
			output_format::OutputFormat,
			output_option::{AddonFunction, AddonOutputOption, ChunkFilenamesOutputOption},
			platform::Platform,
			resolve_options::ResolveOptions,
			source_map_type::SourceMapType,
			sourcemap_ignore_list::SourceMapIgnoreList,
			sourcemap_path_transform::SourceMapPathTransform,
			treeshake::{InnerOptions, ModuleSideEffects, TreeshakeOptions},
		},
		BundlerOptions,
	};
}

// We don't want internal position adjustment of files affect users, so all
// items are exported in the root.
pub use bundler_options::*;

pub use crate::{
	chunk::{
		chunk_table::ChunkTable,
		types::{
			cross_chunk_import_item::CrossChunkImportItem,
			preliminary_filename::PreliminaryFilename,
		},
		Chunk,
	},
	css::{css_module::CssModule, css_module_idx::CssModuleIdx, css_view::CssView},
	ecmascript::{
		ecma_asset_meta::EcmaAssetMeta,
		ecma_view::{EcmaModuleAstUsage, EcmaView},
		module_idx::ModuleIdx,
	},
	file_emitter::{EmittedAsset, FileEmitter, SharedFileEmitter},
	module::{external_module::ExternalModule, normal_module::NormalModule, Module},
	types::{
		asset::Asset,
		asset_idx::AssetIdx,
		asset_meta::InstantiationKind,
		asset_source::AssetSource,
		ast_scopes::AstScopes,
		bundler_file_system::BundlerFileSystem,
		chunk_idx::ChunkIdx,
		chunk_kind::ChunkKind,
		ecma_ast_idx::EcmaAstIdx,
		entry_point::{EntryPoint, EntryPointKind},
		exports_kind::ExportsKind,
		external_module_idx::ExternalModuleIdx,
		import_record::{
			ImportKind,
			ImportRecord,
			ImportRecordIdx,
			ImportRecordMeta,
			RawImportRecord,
		},
		importer_record::ImporterRecord,
		instantiated_chunk::InstantiatedChunk,
		interop::Interop,
		member_expr_ref::MemberExprRef,
		module_def_format::ModuleDefFormat,
		module_id::ModuleId,
		module_idx::LegacyModuleIdx,
		module_info::ModuleInfo,
		module_table::{IndexExternalModules, IndexModules, ModuleTable},
		module_view::ModuleView,
		named_export::LocalExport,
		named_import::{NamedImport, Specifier},
		output::{Output, OutputAsset},
		output_chunk::OutputChunk,
		package_json::PackageJson,
		rendered_module::RenderedModule,
		resolved_export::ResolvedExport,
		resolved_request_info::ResolvedId,
		rollup_pre_rendered_chunk::RollupPreRenderedChunk,
		rollup_rendered_chunk::RollupRenderedChunk,
		side_effects,
		stmt_info::{DebugStmtInfoForTreeShaking, StmtInfo, StmtInfoIdx, StmtInfos},
		str_or_bytes::StrOrBytes,
		symbol_or_member_expr_ref::SymbolOrMemberExprRef,
		symbol_ref::SymbolRef,
		wrap_kind::WrapKind,
	},
};
