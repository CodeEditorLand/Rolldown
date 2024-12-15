mod asset;
mod chunk;
mod css;
mod ecmascript;
mod file_emitter;
mod inner_bundler_options;
mod module;
mod module_loader;
mod type_aliases;
mod types;

/// This module is to help `rolldown` crate could export types related bundler
/// options easily. `rolldown` crate could use `pub use
/// rolldown_common::bundler_options::*;` to export all types, so we don't need
/// write the same code in `rolldown` crate again.
pub mod bundler_options {
	pub use crate::inner_bundler_options::{
		BundlerOptions,
		types::{
			advanced_chunks_options::{AdvancedChunksOptions, MatchGroup},
			checks_options::ChecksOptions,
			comments::Comments,
			es_module_flag::EsModuleFlag,
			experimental_options::ExperimentalOptions,
			filename_template::{FileNameRenderOptions, FilenameTemplate},
			hash_characters::HashCharacters,
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
			output_option::{
				AddonFunction,
				AddonOutputOption,
				ChunkFilenamesOutputOption,
				GlobalsOutputOption,
			},
			platform::Platform,
			resolve_options::ResolveOptions,
			source_map_type::SourceMapType,
			sourcemap_ignore_list::SourceMapIgnoreList,
			sourcemap_path_transform::SourceMapPathTransform,
			target::ESTarget,
			treeshake::{InnerOptions, ModuleSideEffects, ModuleSideEffectsRule, TreeshakeOptions},
			watch_option::{NotifyOption, WatchOption},
		},
	};
}

// We don't want internal position adjustment of files affect users, so all
// items are exported in the root.
pub use bundler_options::*;

pub use crate::{
	asset::asset_view::AssetView,
	chunk::{
		Chunk,
		chunk_table::ChunkTable,
		types::{
			cross_chunk_import_item::CrossChunkImportItem,
			preliminary_filename::PreliminaryFilename,
		},
	},
	css::{
		css_module::CssModule,
		css_module_idx::CssModuleIdx,
		css_view::{CssAssetNameReplacer, CssRenderer, CssView},
	},
	ecmascript::{
		comment_annotation::{ROLLDOWN_IGNORE, get_leading_comment},
		dynamic_import_usage,
		ecma_asset_meta::EcmaAssetMeta,
		ecma_view::{
			EcmaModuleAstUsage,
			EcmaView,
			EcmaViewMeta,
			ImportMetaRolldownAssetReplacer,
			ThisExprReplaceKind,
			generate_replace_this_expr_map,
		},
		module_idx::ModuleIdx,
		node_builtin_modules::is_existing_node_builtin_modules,
	},
	file_emitter::{EmittedAsset, FileEmitter, SharedFileEmitter},
	module::{
		Module,
		external_module::ExternalModule,
		normal_module::{ModuleRenderArgs, NormalModule},
	},
	module_loader::{
		ModuleLoaderMsg,
		runtime_module_brief::{RUNTIME_MODULE_ID, RuntimeModuleBrief},
		runtime_task_result::RuntimeModuleTaskResult,
		task_result::{EcmaRelated, NormalModuleTaskResult},
	},
	types::{
		asset::Asset,
		asset_idx::AssetIdx,
		asset_meta::InstantiationKind,
		ast_scopes::AstScopes,
		bundler_file_system::BundlerFileSystem,
		chunk_idx::ChunkIdx,
		chunk_kind::ChunkKind,
		ecma_ast_idx::EcmaAstIdx,
		entry_point::{EntryPoint, EntryPointKind},
		exports_kind::ExportsKind,
		external_module_idx::ExternalModuleIdx,
		import_kind::ImportKind,
		import_record::{ImportRecordIdx, ImportRecordMeta, RawImportRecord, ResolvedImportRecord},
		importer_record::ImporterRecord,
		instantiated_chunk::InstantiatedChunk,
		interop::Interop,
		member_expr_ref::MemberExprRef,
		module_def_format::ModuleDefFormat,
		module_id::ModuleId,
		module_idx::LegacyModuleIdx,
		module_info::ModuleInfo,
		module_render_output::ModuleRenderOutput,
		module_table::{IndexExternalModules, IndexModules, ModuleTable},
		module_view::ModuleView,
		named_export::LocalExport,
		named_import::{NamedImport, Specifier},
		namespace_alias::NamespaceAlias,
		output::{Output, OutputAsset},
		output_chunk::OutputChunk,
		outputs_diagnostics::OutputsDiagnostics,
		package_json::PackageJson,
		rendered_module::RenderedModule,
		resolved_export::ResolvedExport,
		resolved_request_info::ResolvedId,
		rollup_pre_rendered_chunk::RollupPreRenderedChunk,
		rollup_rendered_chunk::RollupRenderedChunk,
		side_effects,
		source_mutation::SourceMutation,
		stmt_info::{DebugStmtInfoForTreeShaking, StmtInfo, StmtInfoIdx, StmtInfoMeta, StmtInfos},
		str_or_bytes::StrOrBytes,
		symbol_name_ref_token::SymbolNameRefToken,
		symbol_or_member_expr_ref::SymbolOrMemberExprRef,
		symbol_ref::SymbolRef,
		symbol_ref_db::{GetLocalDb, SymbolRefDb, SymbolRefDbForModule, SymbolRefFlags},
		watch::{
			BundleEndEventData,
			BundleEvent,
			WatcherChangeData,
			WatcherChangeKind,
			WatcherEvent,
		},
		wrap_kind::WrapKind,
	},
};
