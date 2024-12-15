use oxc_index::IndexVec;

use crate::{
	__inner::SharedPluginable,
	PluginContext,
	types::{hook_filter::HookFilterOptions, plugin_idx::PluginIdx},
};

pub type IndexPluginable = IndexVec<PluginIdx, SharedPluginable>;
pub type IndexPluginContext = IndexVec<PluginIdx, PluginContext>;
pub type IndexPluginFilter = IndexVec<PluginIdx, HookFilterOptions>;
