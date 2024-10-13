import {
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  getDefaultContext as __emnapiGetDefaultContext,
  WASI as __WASI,
  createOnMessage as __wasmCreateOnMessageForFsProxy,
} from '@napi-rs/wasm-runtime'
import { memfs } from '@napi-rs/wasm-runtime/fs'
import __wasmUrl from './rolldown-binding.wasm32-wasi.wasm?url'

export const { fs: __fs, vol: __volume } = memfs()

const __wasi = new __WASI({
  version: 'preview1',
  fs: __fs,
  preopens: {
    '/': '/',
  }
})

const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  initial: 16384,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })
    worker.addEventListener('message', __wasmCreateOnMessageForFsProxy(__fs))

    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    __napi_rs_initialize_modules(instance)
  },
})

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports['__napi_register__SourceMap_struct_0']?.()
  __napiInstance.exports['__napi_register__IsolatedDeclarationsResult_struct_1']?.()
  __napiInstance.exports['__napi_register__IsolatedDeclarationsOptions_struct_2']?.()
  __napiInstance.exports['__napi_register__TransformResult_struct_3']?.()
  __napiInstance.exports['__napi_register__TransformOptions_struct_4']?.()
  __napiInstance.exports['__napi_register__TypeScriptOptions_struct_5']?.()
  __napiInstance.exports['__napi_register__JsxOptions_struct_6']?.()
  __napiInstance.exports['__napi_register__ReactRefreshOptions_struct_7']?.()
  __napiInstance.exports['__napi_register__ArrowFunctionsOptions_struct_8']?.()
  __napiInstance.exports['__napi_register__Es2015Options_struct_9']?.()
  __napiInstance.exports['__napi_register__isolated_declaration_0']?.()
  __napiInstance.exports['__napi_register__transform_1']?.()
  __napiInstance.exports['__napi_register__Bundler_struct_0']?.()
  __napiInstance.exports['__napi_register__Bundler_impl_7']?.()
  __napiInstance.exports['__napi_register__BindingExperimentalOptions_struct_8']?.()
  __napiInstance.exports['__napi_register__BindingInjectImportNamed_struct_9']?.()
  __napiInstance.exports['__napi_register__BindingInjectImportNamespace_struct_10']?.()
  __napiInstance.exports['__napi_register__BindingInputItem_struct_11']?.()
  __napiInstance.exports['__napi_register__BindingResolveOptions_struct_12']?.()
  __napiInstance.exports['__napi_register__BindingTreeshake_struct_13']?.()
  __napiInstance.exports['__napi_register__BindingInputOptions_struct_14']?.()
  __napiInstance.exports['__napi_register__BindingAdvancedChunksOptions_struct_15']?.()
  __napiInstance.exports['__napi_register__BindingMatchGroup_struct_16']?.()
  __napiInstance.exports['__napi_register__BindingOutputOptions_struct_17']?.()
  __napiInstance.exports['__napi_register__BindingPluginContext_struct_18']?.()
  __napiInstance.exports['__napi_register__BindingPluginContext_impl_24']?.()
  __napiInstance.exports['__napi_register__BindingPluginContextResolvedId_struct_25']?.()
  __napiInstance.exports['__napi_register__BindingPluginOptions_struct_26']?.()
  __napiInstance.exports['__napi_register__BindingPluginWithIndex_struct_27']?.()
  __napiInstance.exports['__napi_register__BindingTransformPluginContext_struct_28']?.()
  __napiInstance.exports['__napi_register__BindingTransformPluginContext_impl_30']?.()
  __napiInstance.exports['__napi_register__BindingAssetSource_struct_31']?.()
  __napiInstance.exports['__napi_register__BindingEmittedAsset_struct_32']?.()
  __napiInstance.exports['__napi_register__BindingGeneralHookFilter_struct_33']?.()
  __napiInstance.exports['__napi_register__BindingTransformHookFilter_struct_34']?.()
  __napiInstance.exports['__napi_register__BindingHookLoadOutput_struct_35']?.()
  __napiInstance.exports['__napi_register__BindingHookRenderChunkOutput_struct_36']?.()
  __napiInstance.exports['__napi_register__BindingHookResolveIdExtraArgs_struct_37']?.()
  __napiInstance.exports['__napi_register__BindingHookResolveIdOutput_struct_38']?.()
  __napiInstance.exports['__napi_register__BindingHookSideEffects_39']?.()
  __napiInstance.exports['__napi_register__BindingHookTransformOutput_struct_40']?.()
  __napiInstance.exports['__napi_register__BindingStringOrRegex_struct_41']?.()
  __napiInstance.exports['__napi_register__BindingPluginContextResolveOptions_struct_42']?.()
  __napiInstance.exports['__napi_register__BindingTransformHookExtraArgs_struct_43']?.()
  __napiInstance.exports['__napi_register__BindingBuiltinPlugin_struct_44']?.()
  __napiInstance.exports['__napi_register__BindingBuiltinPluginName_45']?.()
  __napiInstance.exports['__napi_register__BindingGlobImportPluginConfig_struct_46']?.()
  __napiInstance.exports['__napi_register__BindingManifestPluginConfig_struct_47']?.()
  __napiInstance.exports['__napi_register__BindingModulePreloadPolyfillPluginConfig_struct_48']?.()
  __napiInstance.exports['__napi_register__BindingJsonPluginConfig_struct_49']?.()
  __napiInstance.exports['__napi_register__BindingTransformPluginConfig_struct_50']?.()
  __napiInstance.exports['__napi_register__BindingAliasPluginConfig_struct_51']?.()
  __napiInstance.exports['__napi_register__BindingAliasPluginAlias_struct_52']?.()
  __napiInstance.exports['__napi_register__BindingBuildImportAnalysisPluginConfig_struct_53']?.()
  __napiInstance.exports['__napi_register__BindingReplacePluginConfig_struct_54']?.()
  __napiInstance.exports['__napi_register__BindingPluginOrder_55']?.()
  __napiInstance.exports['__napi_register__BindingPluginHookMeta_struct_56']?.()
  __napiInstance.exports['__napi_register__ParallelJsPluginRegistry_struct_57']?.()
  __napiInstance.exports['__napi_register__ParallelJsPluginRegistry_impl_59']?.()
  __napiInstance.exports['__napi_register__register_plugins_60']?.()
  __napiInstance.exports['__napi_register__BindingLog_struct_61']?.()
  __napiInstance.exports['__napi_register__BindingLogLevel_62']?.()
  __napiInstance.exports['__napi_register__BindingModuleInfo_struct_63']?.()
  __napiInstance.exports['__napi_register__BindingModuleInfo_impl_65']?.()
  __napiInstance.exports['__napi_register__BindingOutputAsset_struct_66']?.()
  __napiInstance.exports['__napi_register__BindingOutputAsset_impl_71']?.()
  __napiInstance.exports['__napi_register__JsOutputAsset_struct_72']?.()
  __napiInstance.exports['__napi_register__BindingOutputChunk_struct_73']?.()
  __napiInstance.exports['__napi_register__BindingOutputChunk_impl_88']?.()
  __napiInstance.exports['__napi_register__JsOutputChunk_struct_89']?.()
  __napiInstance.exports['__napi_register__BindingOutputs_struct_90']?.()
  __napiInstance.exports['__napi_register__BindingOutputs_impl_93']?.()
  __napiInstance.exports['__napi_register__JsChangedOutputs_struct_94']?.()
  __napiInstance.exports['__napi_register__PreRenderedChunk_struct_95']?.()
  __napiInstance.exports['__napi_register__RenderedChunk_struct_96']?.()
  __napiInstance.exports['__napi_register__BindingRenderedModule_struct_97']?.()
  __napiInstance.exports['__napi_register__AliasItem_struct_98']?.()
  __napiInstance.exports['__napi_register__ExtensionAliasItem_struct_99']?.()
  __napiInstance.exports['__napi_register__BindingSourcemap_struct_100']?.()
  __napiInstance.exports['__napi_register__BindingJsonSourcemap_struct_101']?.()
}
export const BindingLog = __napiModule.exports.BindingLog
export const BindingModuleInfo = __napiModule.exports.BindingModuleInfo
export const BindingOutputAsset = __napiModule.exports.BindingOutputAsset
export const BindingOutputChunk = __napiModule.exports.BindingOutputChunk
export const BindingOutputs = __napiModule.exports.BindingOutputs
export const BindingPluginContext = __napiModule.exports.BindingPluginContext
export const BindingTransformPluginContext = __napiModule.exports.BindingTransformPluginContext
export const Bundler = __napiModule.exports.Bundler
export const ParallelJsPluginRegistry = __napiModule.exports.ParallelJsPluginRegistry
export const BindingBuiltinPluginName = __napiModule.exports.BindingBuiltinPluginName
export const BindingHookSideEffects = __napiModule.exports.BindingHookSideEffects
export const BindingLogLevel = __napiModule.exports.BindingLogLevel
export const BindingPluginOrder = __napiModule.exports.BindingPluginOrder
export const isolatedDeclaration = __napiModule.exports.isolatedDeclaration
export const registerPlugins = __napiModule.exports.registerPlugins
export const transform = __napiModule.exports.transform