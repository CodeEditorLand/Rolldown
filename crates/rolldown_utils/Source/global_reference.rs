// cSpell:disable
use oxc::span::Atom;

static GLOBAL_IDENT: phf::Set<&str> = phf::phf_set![
	// Rolldown specific, because oxc treated them as identifiers, esbuild did not
	"Infinity",
	"undefined",
	"NaN",
	// These global identifiers should exist in all JavaScript environments.
	"Array",
	"Boolean",
	"Function",
	"Math",
	"Number",
	"Object",
	"RegExp",
	"String",
	// Other globals present in both the browser and node (except "eval" because
	// it has special behavior)
	"AbortController",
	"AbortSignal",
	"AggregateError",
	"ArrayBuffer",
	"BigInt",
	"DataView",
	"Date",
	"Error",
	"EvalError",
	"Event",
	"EventTarget",
	"Float32Array",
	"Float64Array",
	"Int16Array",
	"Int32Array",
	"Int8Array",
	"Intl",
	"JSON",
	"Map",
	"MessageChannel",
	"MessageEvent",
	"MessagePort",
	"Promise",
	"Proxy",
	"RangeError",
	"ReferenceError",
	"Reflect",
	"Set",
	"Symbol",
	"SyntaxError",
	"TextDecoder",
	"TextEncoder",
	"TypeError",
	"URIError",
	"URL",
	"URLSearchParams",
	"Uint16Array",
	"Uint32Array",
	"Uint8Array",
	"Uint8ClampedArray",
	"WeakMap",
	"WeakSet",
	"WebAssembly",
	"clearInterval",
	"clearTimeout",
	"console",
	"decodeURI",
	"decodeURIComponent",
	"encodeURI",
	"encodeURIComponent",
	"escape",
	"globalThis",
	"isFinite",
	"isNaN",
	"parseFloat",
	"parseInt",
	"queueMicrotask",
	"setInterval",
	"setTimeout",
	"unescape",
	// CSSOM APIs
	"CSSAnimation",
	"CSSFontFaceRule",
	"CSSImportRule",
	"CSSKeyframeRule",
	"CSSKeyframesRule",
	"CSSMediaRule",
	"CSSNamespaceRule",
	"CSSPageRule",
	"CSSRule",
	"CSSRuleList",
	"CSSStyleDeclaration",
	"CSSStyleRule",
	"CSSStyleSheet",
	"CSSSupportsRule",
	"CSSTransition",
	// SVG DOM
	"SVGAElement",
	"SVGAngle",
	"SVGAnimateElement",
	"SVGAnimateMotionElement",
	"SVGAnimateTransformElement",
	"SVGAnimatedAngle",
	"SVGAnimatedBoolean",
	"SVGAnimatedEnumeration",
	"SVGAnimatedInteger",
	"SVGAnimatedLength",
	"SVGAnimatedLengthList",
	"SVGAnimatedNumber",
	"SVGAnimatedNumberList",
	"SVGAnimatedPreserveAspectRatio",
	"SVGAnimatedRect",
	"SVGAnimatedString",
	"SVGAnimatedTransformList",
	"SVGAnimationElement",
	"SVGCircleElement",
	"SVGClipPathElement",
	"SVGComponentTransferFunctionElement",
	"SVGDefsElement",
	"SVGDescElement",
	"SVGElement",
	"SVGEllipseElement",
	"SVGFEBlendElement",
	"SVGFEColorMatrixElement",
	"SVGFEComponentTransferElement",
	"SVGFECompositeElement",
	"SVGFEConvolveMatrixElement",
	"SVGFEDiffuseLightingElement",
	"SVGFEDisplacementMapElement",
	"SVGFEDistantLightElement",
	"SVGFEDropShadowElement",
	"SVGFEFloodElement",
	"SVGFEFuncAElement",
	"SVGFEFuncBElement",
	"SVGFEFuncGElement",
	"SVGFEFuncRElement",
	"SVGFEGaussianBlurElement",
	"SVGFEImageElement",
	"SVGFEMergeElement",
	"SVGFEMergeNodeElement",
	"SVGFEMorphologyElement",
	"SVGFEOffsetElement",
	"SVGFEPointLightElement",
	"SVGFESpecularLightingElement",
	"SVGFESpotLightElement",
	"SVGFETileElement",
	"SVGFETurbulenceElement",
	"SVGFilterElement",
	"SVGForeignObjectElement",
	"SVGGElement",
	"SVGGeometryElement",
	"SVGGradientElement",
	"SVGGraphicsElement",
	"SVGImageElement",
	"SVGLength",
	"SVGLengthList",
	"SVGLineElement",
	"SVGLinearGradientElement",
	"SVGMPathElement",
	"SVGMarkerElement",
	"SVGMaskElement",
	"SVGMatrix",
	"SVGMetadataElement",
	"SVGNumber",
	"SVGNumberList",
	"SVGPathElement",
	"SVGPatternElement",
	"SVGPoint",
	"SVGPointList",
	"SVGPolygonElement",
	"SVGPolylineElement",
	"SVGPreserveAspectRatio",
	"SVGRadialGradientElement",
	"SVGRect",
	"SVGRectElement",
	"SVGSVGElement",
	"SVGScriptElement",
	"SVGSetElement",
	"SVGStopElement",
	"SVGStringList",
	"SVGStyleElement",
	"SVGSwitchElement",
	"SVGSymbolElement",
	"SVGTSpanElement",
	"SVGTextContentElement",
	"SVGTextElement",
	"SVGTextPathElement",
	"SVGTextPositioningElement",
	"SVGTitleElement",
	"SVGTransform",
	"SVGTransformList",
	"SVGUnitTypes",
	"SVGUseElement",
	"SVGViewElement",
	// Other browser APIs
	//
	// This list contains all globals present in modern versions of Chrome, Safari,
	// and Firefox except for the following properties, since they have a side effect
	// of triggering layout (https://gist.github.com/paulirish/5d52fb081b3570c81e3a):
	//
	//   - scrollX
	//   - scrollY
	//   - innerWidth
	//   - innerHeight
	//   - pageXOffset
	//   - pageYOffset
	//
	// The following globals have also been removed since they sometimes throw an
	// exception when accessed, which is a side effect (for more information see
	// https://stackoverflow.com/a/33047477):
	//
	//   - localStorage
	//   - sessionStorage
	//
	"AnalyserNode",
	"Animation",
	"AnimationEffect",
	"AnimationEvent",
	"AnimationPlaybackEvent",
	"AnimationTimeline",
	"Attr",
	"Audio",
	"AudioBuffer",
	"AudioBufferSourceNode",
	"AudioDestinationNode",
	"AudioListener",
	"AudioNode",
	"AudioParam",
	"AudioProcessingEvent",
	"AudioScheduledSourceNode",
	"BarProp",
	"BeforeUnloadEvent",
	"BiquadFilterNode",
	"Blob",
	"BlobEvent",
	"ByteLengthQueuingStrategy",
	"CDATASection",
	"CSS",
	"CanvasGradient",
	"CanvasPattern",
	"CanvasRenderingContext2D",
	"ChannelMergerNode",
	"ChannelSplitterNode",
	"CharacterData",
	"ClipboardEvent",
	"CloseEvent",
	"Comment",
	"CompositionEvent",
	"ConvolverNode",
	"CountQueuingStrategy",
	"Crypto",
	"CustomElementRegistry",
	"CustomEvent",
	"DOMException",
	"DOMImplementation",
	"DOMMatrix",
	"DOMMatrixReadOnly",
	"DOMParser",
	"DOMPoint",
	"DOMPointReadOnly",
	"DOMQuad",
	"DOMRect",
	"DOMRectList",
	"DOMRectReadOnly",
	"DOMStringList",
	"DOMStringMap",
	"DOMTokenList",
	"DataTransfer",
	"DataTransferItem",
	"DataTransferItemList",
	"DelayNode",
	"Document",
	"DocumentFragment",
	"DocumentTimeline",
	"DocumentType",
	"DragEvent",
	"DynamicsCompressorNode",
	"Element",
	"ErrorEvent",
	"EventSource",
	"File",
	"FileList",
	"FileReader",
	"FocusEvent",
	"FontFace",
	"FormData",
	"GainNode",
	"Gamepad",
	"GamepadButton",
	"GamepadEvent",
	"Geolocation",
	"GeolocationPositionError",
	"HTMLAllCollection",
	"HTMLAnchorElement",
	"HTMLAreaElement",
	"HTMLAudioElement",
	"HTMLBRElement",
	"HTMLBaseElement",
	"HTMLBodyElement",
	"HTMLButtonElement",
	"HTMLCanvasElement",
	"HTMLCollection",
	"HTMLDListElement",
	"HTMLDataElement",
	"HTMLDataListElement",
	"HTMLDetailsElement",
	"HTMLDirectoryElement",
	"HTMLDivElement",
	"HTMLDocument",
	"HTMLElement",
	"HTMLEmbedElement",
	"HTMLFieldSetElement",
	"HTMLFontElement",
	"HTMLFormControlsCollection",
	"HTMLFormElement",
	"HTMLFrameElement",
	"HTMLFrameSetElement",
	"HTMLHRElement",
	"HTMLHeadElement",
	"HTMLHeadingElement",
	"HTMLHtmlElement",
	"HTMLIFrameElement",
	"HTMLImageElement",
	"HTMLInputElement",
	"HTMLLIElement",
	"HTMLLabelElement",
	"HTMLLegendElement",
	"HTMLLinkElement",
	"HTMLMapElement",
	"HTMLMarqueeElement",
	"HTMLMediaElement",
	"HTMLMenuElement",
	"HTMLMetaElement",
	"HTMLMeterElement",
	"HTMLModElement",
	"HTMLOListElement",
	"HTMLObjectElement",
	"HTMLOptGroupElement",
	"HTMLOptionElement",
	"HTMLOptionsCollection",
	"HTMLOutputElement",
	"HTMLParagraphElement",
	"HTMLParamElement",
	"HTMLPictureElement",
	"HTMLPreElement",
	"HTMLProgressElement",
	"HTMLQuoteElement",
	"HTMLScriptElement",
	"HTMLSelectElement",
	"HTMLSlotElement",
	"HTMLSourceElement",
	"HTMLSpanElement",
	"HTMLStyleElement",
	"HTMLTableCaptionElement",
	"HTMLTableCellElement",
	"HTMLTableColElement",
	"HTMLTableElement",
	"HTMLTableRowElement",
	"HTMLTableSectionElement",
	"HTMLTemplateElement",
	"HTMLTextAreaElement",
	"HTMLTimeElement",
	"HTMLTitleElement",
	"HTMLTrackElement",
	"HTMLUListElement",
	"HTMLUnknownElement",
	"HTMLVideoElement",
	"HashChangeEvent",
	"Headers",
	"History",
	"IDBCursor",
	"IDBCursorWithValue",
	"IDBDatabase",
	"IDBFactory",
	"IDBIndex",
	"IDBKeyRange",
	"IDBObjectStore",
	"IDBOpenDBRequest",
	"IDBRequest",
	"IDBTransaction",
	"IDBVersionChangeEvent",
	"Image",
	"ImageData",
	"InputEvent",
	"IntersectionObserver",
	"IntersectionObserverEntry",
	"KeyboardEvent",
	"KeyframeEffect",
	"Location",
	"MediaCapabilities",
	"MediaElementAudioSourceNode",
	"MediaEncryptedEvent",
	"MediaError",
	"MediaList",
	"MediaQueryList",
	"MediaQueryListEvent",
	"MediaRecorder",
	"MediaSource",
	"MediaStream",
	"MediaStreamAudioDestinationNode",
	"MediaStreamAudioSourceNode",
	"MediaStreamTrack",
	"MediaStreamTrackEvent",
	"MimeType",
	"MimeTypeArray",
	"MouseEvent",
	"MutationEvent",
	"MutationObserver",
	"MutationRecord",
	"NamedNodeMap",
	"Navigator",
	"Node",
	"NodeFilter",
	"NodeIterator",
	"NodeList",
	"Notification",
	"OfflineAudioCompletionEvent",
	"Option",
	"OscillatorNode",
	"PageTransitionEvent",
	"Path2D",
	"Performance",
	"PerformanceEntry",
	"PerformanceMark",
	"PerformanceMeasure",
	"PerformanceNavigation",
	"PerformanceObserver",
	"PerformanceObserverEntryList",
	"PerformanceResourceTiming",
	"PerformanceTiming",
	"PeriodicWave",
	"Plugin",
	"PluginArray",
	"PointerEvent",
	"PopStateEvent",
	"ProcessingInstruction",
	"ProgressEvent",
	"PromiseRejectionEvent",
	"RTCCertificate",
	"RTCDTMFSender",
	"RTCDTMFToneChangeEvent",
	"RTCDataChannel",
	"RTCDataChannelEvent",
	"RTCIceCandidate",
	"RTCPeerConnection",
	"RTCPeerConnectionIceEvent",
	"RTCRtpReceiver",
	"RTCRtpSender",
	"RTCRtpTransceiver",
	"RTCSessionDescription",
	"RTCStatsReport",
	"RTCTrackEvent",
	"RadioNodeList",
	"Range",
	"ReadableStream",
	"Request",
	"ResizeObserver",
	"ResizeObserverEntry",
	"Response",
	"Screen",
	"ScriptProcessorNode",
	"SecurityPolicyViolationEvent",
	"Selection",
	"ShadowRoot",
	"SourceBuffer",
	"SourceBufferList",
	"SpeechSynthesisEvent",
	"SpeechSynthesisUtterance",
	"StaticRange",
	"Storage",
	"StorageEvent",
	"StyleSheet",
	"StyleSheetList",
	"Text",
	"TextMetrics",
	"TextTrack",
	"TextTrackCue",
	"TextTrackCueList",
	"TextTrackList",
	"TimeRanges",
	"TrackEvent",
	"TransitionEvent",
	"TreeWalker",
	"UIEvent",
	"VTTCue",
	"ValidityState",
	"VisualViewport",
	"WaveShaperNode",
	"WebGLActiveInfo",
	"WebGLBuffer",
	"WebGLContextEvent",
	"WebGLFramebuffer",
	"WebGLProgram",
	"WebGLQuery",
	"WebGLRenderbuffer",
	"WebGLRenderingContext",
	"WebGLSampler",
	"WebGLShader",
	"WebGLShaderPrecisionFormat",
	"WebGLSync",
	"WebGLTexture",
	"WebGLUniformLocation",
	"WebKitCSSMatrix",
	"WebSocket",
	"WheelEvent",
	"Window",
	"Worker",
	"XMLDocument",
	"XMLHttpRequest",
	"XMLHttpRequestEventTarget",
	"XMLHttpRequestUpload",
	"XMLSerializer",
	"XPathEvaluator",
	"XPathExpression",
	"XPathResult",
	"XSLTProcessor",
	"alert",
	"atob",
	"blur",
	"btoa",
	"cancelAnimationFrame",
	"captureEvents",
	"close",
	"closed",
	"confirm",
	"customElements",
	"devicePixelRatio",
	"document",
	"event",
	"fetch",
	"find",
	"focus",
	"frameElement",
	"frames",
	"getComputedStyle",
	"getSelection",
	"history",
	"indexedDB",
	"isSecureContext",
	"length",
	"location",
	"locationbar",
	"matchMedia",
	"menubar",
	"moveBy",
	"moveTo",
	"name",
	"navigator",
	"onabort",
	"onafterprint",
	"onanimationend",
	"onanimationiteration",
	"onanimationstart",
	"onbeforeprint",
	"onbeforeunload",
	"onblur",
	"oncanplay",
	"oncanplaythrough",
	"onchange",
	"onclick",
	"oncontextmenu",
	"oncuechange",
	"ondblclick",
	"ondrag",
	"ondragend",
	"ondragenter",
	"ondragleave",
	"ondragover",
	"ondragstart",
	"ondrop",
	"ondurationchange",
	"onemptied",
	"onended",
	"onerror",
	"onfocus",
	"ongotpointercapture",
	"onhashchange",
	"oninput",
	"oninvalid",
	"onkeydown",
	"onkeypress",
	"onkeyup",
	"onlanguagechange",
	"onload",
	"onloadeddata",
	"onloadedmetadata",
	"onloadstart",
	"onlostpointercapture",
	"onmessage",
	"onmousedown",
	"onmouseenter",
	"onmouseleave",
	"onmousemove",
	"onmouseout",
	"onmouseover",
	"onmouseup",
	"onoffline",
	"ononline",
	"onpagehide",
	"onpageshow",
	"onpause",
	"onplay",
	"onplaying",
	"onpointercancel",
	"onpointerdown",
	"onpointerenter",
	"onpointerleave",
	"onpointermove",
	"onpointerout",
	"onpointerover",
	"onpointerup",
	"onpopstate",
	"onprogress",
	"onratechange",
	"onrejectionhandled",
	"onreset",
	"onresize",
	"onscroll",
	"onseeked",
	"onseeking",
	"onselect",
	"onstalled",
	"onstorage",
	"onsubmit",
	"onsuspend",
	"ontimeupdate",
	"ontoggle",
	"ontransitioncancel",
	"ontransitionend",
	"ontransitionrun",
	"ontransitionstart",
	"onunhandledrejection",
	"onunload",
	"onvolumechange",
	"onwaiting",
	"onwebkitanimationend",
	"onwebkitanimationiteration",
	"onwebkitanimationstart",
	"onwebkittransitionend",
	"onwheel",
	"open",
	"opener",
	"origin",
	"outerHeight",
	"outerWidth",
	"parent",
	"performance",
	"personalbar",
	"postMessage",
	"print",
	"prompt",
	"releaseEvents",
	"requestAnimationFrame",
	"resizeBy",
	"resizeTo",
	"screen",
	"screenLeft",
	"screenTop",
	"screenX",
	"screenY",
	"scroll",
	"scrollBy",
	"scrollTo",
	"scrollbars",
	"self",
	"speechSynthesis",
	"status",
	"statusbar",
	"stop",
	"toolbar",
	"top",
	"webkitURL",
	"window",
];

/// Console method references are assumed to have no side effects
/// https://developer.mozilla.org/en-US/docs/Web/API/console
/// `console`
static CONSOLE_SECOND_PROP: phf::Set<&str> = phf::phf_set![
	"assert",
	"clear",
	"count",
	"countReset",
	"debug",
	"dir",
	"dirxml",
	"error",
	"group",
	"groupCollapsed",
	"groupEnd",
	"info",
	"log",
	"table",
	"time",
	"timeEnd",
	"timeLog",
	"trace",
	"warn",
];

/// Reflect: Static methods
/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect#static_methods
/// `Reflect`
static REFLECT_SECOND_PROP: phf::Set<&str> = phf::phf_set![
	"apply",
	"construct",
	"defineProperty",
	"deleteProperty",
	"get",
	"getOwnPropertyDescriptor",
	"getPrototypeOf",
	"has",
	"isExtensible",
	"ownKeys",
	"preventExtensions",
	"set",
	"setPrototypeOf",
];

/// `Math`
static MATH_SECOND_PROP: phf::Set<&str> = phf::phf_set![
	// Math: Static properties
	// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math#Static_properties
	"E", "LN10", "LN2", "LOG10E", "LOG2E", "PI", "SQRT1_2", "SQRT2",
	// Math: Static methods
	// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math#Static_methods
	"abs", "acos", "acosh", "asin", "asinh", "atan", "atan2", "atanh", "cbrt", "ceil", "clz32",
	"cos", "cosh", "exp", "expm1", "floor", "fround", "hypot", "imul", "log", "log10", "log1p",
	"log2", "max", "min", "pow", "random", "round", "sign", "sin", "sinh", "sqrt", "tan", "tanh",
	"trunc",
];

/// `Object`
static OBJECT_SECOND_PROP: phf::Set<&str> = phf::phf_set![
	// Object: Static methods
	// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object#Static_methods
	"assign",
	"create",
	"defineProperties",
	"defineProperty",
	"entries",
	"freeze",
	"fromEntries",
	"getOwnPropertyDescriptor",
	"getOwnPropertyDescriptors",
	"getOwnPropertyNames",
	"getOwnPropertySymbols",
	"getPrototypeOf",
	"is",
	"isExtensible",
	"isFrozen",
	"isSealed",
	"keys",
	"preventExtensions",
	"seal",
	"setPrototypeOf",
	"values",
];

/// `Symbol`
static SYMBOL_SECOND_PROP: phf::Set<&str> = phf::phf_set![
	// Symbol: Static properties
	// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol#static_properties
	"asyncDispose",
	"asyncIterator",
	"dispose",
	"hasInstance",
	"isConcatSpreadable",
	"iterator",
	"match",
	"matchAll",
	"replace",
	"search",
	"species",
	"split",
	"toPrimitive",
	"toStringTag",
	"unscopables",
];

/// `Object.prototype`
static OBJECT_PROTOTYPE_THIRD_PROP: phf::Set<&str> = phf::phf_set![
	// Object: Instance methods
	// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object#Instance_methods
	"__defineGetter__",
	"__defineSetter__",
	"__lookupGetter__",
	"__lookupSetter__",
	"hasOwnProperty",
	"isPrototypeOf",
	"propertyIsEnumerable",
	"toLocaleString",
	"toString",
	"unwatch",
	"valueOf",
	"watch",
];

pub fn is_global_ident_ref(ident: &str) -> bool {
	GLOBAL_IDENT.contains(ident)
}

pub fn is_side_effect_free_member_expr_of_len_two(member_expr: &[Atom]) -> bool {
	if member_expr.len() != 2 {
		return false;
	}
	match member_expr[0].as_ref() {
		"console" => CONSOLE_SECOND_PROP.contains(member_expr[1].as_ref()),
		"Reflect" => REFLECT_SECOND_PROP.contains(member_expr[1].as_ref()),
		"Math" => MATH_SECOND_PROP.contains(member_expr[1].as_ref()),
		"Object" => OBJECT_SECOND_PROP.contains(member_expr[1].as_ref()),
		"Symbol" => SYMBOL_SECOND_PROP.contains(member_expr[1].as_ref()),
		"JSON" => member_expr[1] == "stringify" || member_expr[1] == "parse",
		_ => false,
	}
}

pub fn is_side_effect_free_member_expr_of_len_three(member_expr: &[Atom]) -> bool {
	if member_expr.len() != 3 {
		return false;
	}
	if member_expr[0].as_ref() != "Object" {
		return false;
	}

	if member_expr[1].as_ref() != "prototype" {
		return false;
	}
	OBJECT_PROTOTYPE_THIRD_PROP.contains(member_expr[2].as_str())
}
