use std::{fmt::Debug, future::Future, pin::Pin, sync::Arc};

type SourceMapPathTransformFn = dyn Fn(&str, &str) -> Pin<Box<(dyn Future<Output = anyhow::Result<String>> + Send + 'static)>>
	+ Send
	+ Sync;

#[derive(Clone)]
pub struct SourceMapPathTransform(Arc<SourceMapPathTransformFn>);

impl Debug for SourceMapPathTransform {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "SourceMapPathTransform::Fn(...)")
	}
}

impl SourceMapPathTransform {
	pub fn new(f:Arc<SourceMapPathTransformFn>) -> Self { Self(f) }

	pub async fn call(&self, source:&str, sourcemap_path:&str) -> anyhow::Result<String> {
		self.0(source, sourcemap_path).await
	}
}
