use oxc_resolver::ResolveError;

/// rewrite error reason and ignore path param
/// Note this is just the error message fallback, for better dx,
/// you could polish the error message case by case in the caller side
pub fn oxc_resolve_error_to_reason(e:&ResolveError) -> String {
	match e {
		ResolveError::Ignored(_) => "Path is ignored".to_string(),
		ResolveError::NotFound(_) => "Cannot find module".to_string(),
		ResolveError::TsconfigNotFound(_) => "Tsconfig not found".to_string(),
		ResolveError::TsconfigSelfReference(_) => {
			"Tsconfig's project reference path points to this tsconfig".to_string()
		},
		ResolveError::IOError(_) => "IO error".to_string(),
		ResolveError::Builtin(_) => "Builtin module".to_string(),
		ResolveError::ExtensionAlias(_) => {
			"All of the aliased extensions are not found".to_string()
		},
		ResolveError::Specifier(_) => "The provided path specifier cannot be parsed".to_string(),
		ResolveError::JSON(_) => "JSON parse error".to_string(),
		ResolveError::Restriction(..) => "Path restriction".to_string(),
		ResolveError::InvalidModuleSpecifier(..) => "Invalid module specifier".to_string(),
		ResolveError::InvalidPackageTarget(..) => "Invalid package target".to_string(),
		ResolveError::PackagePathNotExported(..) => {
			"Package subpath is not defined by exports".to_string()
		},
		ResolveError::InvalidPackageConfig(_) => "Invalid package config".to_string(),
		ResolveError::InvalidPackageConfigDefault(_) => {
			"Default condition should be last one in package config".to_string()
		},
		ResolveError::InvalidPackageConfigDirectory(_) => {
			"Expecting folder to folder mapping".to_string()
		},
		ResolveError::PackageImportNotDefined(..) => {
			"Package import specifier is not defined".to_string()
		},
		ResolveError::Unimplemented(_) => "Unimplemented".to_string(),
		ResolveError::Recursion => "Recursion in resolving".to_string(),
	}
}
