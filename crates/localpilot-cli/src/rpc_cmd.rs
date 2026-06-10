//! `localpilot rpc` — drive the session runtime over stdin/stdout.
//!
//! Newline-delimited JSON: typed commands in, streamed session events out.
//! Permission asks are surfaced as events and answered by `permission_reply`
//! commands; an unanswered ask is denied, exactly like non-interactive mode.

use localpilot_config::{CliOverrides, ConfigPaths};
use localpilot_harness::{SessionConfig, SessionRuntime};
use localpilot_llm::ProviderRegistry;
use localpilot_recovery::{RecoveryBudget, RecoveryEngine};
use localpilot_rpc::{serve, serve_acp, RpcApprover, ServeContext};
use localpilot_sandbox::{Interactivity, PermissionEngine, Profile, Workspace};
use localpilot_store::Store;

/// Which stdio protocol to serve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireProtocol {
    /// The native newline-delimited JSON protocol.
    Native,
    /// The Agent Client Protocol (JSON-RPC 2.0) for editors.
    Acp,
}

/// Serve one client on stdin/stdout until shutdown or end of input.
///
/// # Errors
/// Returns an error if configuration, the provider, or the workspace cannot
/// be set up, or the transport fails.
pub async fn run(
    model: Option<&str>,
    provider_id: Option<&str>,
    profile: Profile,
    protocol: WireProtocol,
) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let config = localpilot_config::load(&ConfigPaths::standard(&cwd), &CliOverrides::default())?;
    let model = model
        .map(str::to_string)
        .or_else(|| config.resolve_model(provider_id))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "no model: pass --model, or set a default in .localpilot.toml \
                 ([providers.<id>] model = \"...\")"
            )
        })?;
    let registry = ProviderRegistry::from_config(&config)?;
    let provider = match provider_id {
        Some(id) => registry.get(id),
        None => registry.default_provider(),
    }
    .cloned()
    .ok_or_else(|| anyhow::anyhow!("no provider is configured"))?;

    let (approver, ask_rx, asks) = RpcApprover::new();
    let context_token_limit = localpilot_harness::effective_context_limit(
        provider.declaration().max_context_tokens,
        config.harness.context_token_limit,
    );
    let mut runtime = SessionRuntime::new(
        provider,
        crate::mcp::McpTools::load(&config).await.registry(),
        PermissionEngine::new(profile, Vec::new()),
        Box::new(approver),
        Store::open(&cwd),
        Workspace::new(&cwd)?,
        RecoveryEngine::new(RecoveryBudget::default()),
        SessionConfig {
            model: model.clone(),
            // The wire client answers asks; the engine itself treats the
            // session as interactive so ask-class effects reach the client
            // instead of being denied outright.
            interactivity: Interactivity::Interactive,
            trusted: profile == Profile::Bypass,
            context_token_limit,
            ..SessionConfig::default()
        },
        Vec::new(),
    );

    match protocol {
        WireProtocol::Native => {
            let context = ServeContext {
                model,
                profile: profile_label(profile).to_string(),
                root: Some(cwd),
            };
            serve(
                &mut runtime,
                ask_rx,
                asks,
                tokio::io::stdin(),
                tokio::io::stdout(),
                &context,
            )
            .await?;
        }
        WireProtocol::Acp => {
            serve_acp(
                &mut runtime,
                ask_rx,
                asks,
                tokio::io::stdin(),
                tokio::io::stdout(),
            )
            .await?;
        }
    }
    Ok(())
}

fn profile_label(profile: Profile) -> &'static str {
    match profile {
        Profile::Default => "default",
        Profile::Relaxed => "relaxed",
        Profile::Bypass => "bypass",
    }
}
