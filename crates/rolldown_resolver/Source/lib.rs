// An wrapper around the `oxc_resolver` crate to provide a more
// rolldown-specific API.

pub mod error;
mod resolver;

pub use oxc_resolver::ResolveError;
pub use rolldown_common::bundler_options::ResolveOptions;

pub use crate::resolver::{ResolveReturn, Resolver};
