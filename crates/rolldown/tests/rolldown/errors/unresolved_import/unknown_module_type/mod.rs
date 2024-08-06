use std::{borrow::Cow, sync::Arc};

use rolldown::{BundlerOptions, InputItem};
use rolldown_plugin::{
  HookResolveIdArgs, HookResolveIdOutput, HookResolveIdReturn, Plugin, SharedPluginContext,
};
use rolldown_testing::{abs_file_dir, integration_test::IntegrationTest, test_config::TestMeta};

#[derive(Debug)]
struct UnknownModuleType;

impl Plugin for UnknownModuleType {
  fn name(&self) -> Cow<'static, str> {
    "unresolved-import-unknown-module-type".into()
  }

  async fn resolve_id(
    &self,
    _ctx: &SharedPluginContext,
    args: &HookResolveIdArgs<'_>,
  ) -> HookResolveIdReturn {
    if args.specifier == "test.javascript" {
      return Ok(Some(HookResolveIdOutput {
        id: args.specifier.to_string(),
        external: Some(false),
        ..Default::default()
      }));
    }
    Ok(None)
  }
}

#[tokio::test(flavor = "multi_thread")]
async fn should_failed_to_guess_module_type_with_diagnostic() {
  let cwd = abs_file_dir!();

  IntegrationTest::new(TestMeta { expect_error: true, ..Default::default() })
    .run_with_plugins(
      BundlerOptions {
        input: Some(vec![InputItem {
          name: Some("entry".to_string()),
          import: "./entry.js".to_string(),
        }]),
        cwd: Some(cwd),
        ..Default::default()
      },
      vec![Arc::new(UnknownModuleType)],
    )
    .await;
}
