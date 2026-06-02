//! Golden-task evals: deterministic tasks that prove the agent actually completes
//! work, scored with a scorecard. Tasks are driven by the fake provider so they
//! run offline; an optional live mode is gated behind `UNSHACKLED_LIVE_TESTS`.
//!
//! Fixtures are authored for this repository — never copied from another
//! implementation.
#![allow(clippy::unwrap_used)]

use std::path::Path;
use std::process::Command;
use std::sync::Arc;

use serde_json::{json, Value};
use unshackled_harness::{resume_one_step, RuleEngine, SessionConfig, SessionRuntime};
use unshackled_llm::FakeProvider;
use unshackled_recovery::{RecoveryBudget, RecoveryEngine};
use unshackled_sandbox::{Interactivity, PermissionEngine, Profile, ScriptedApprover, Workspace};
use unshackled_store::Store;
use unshackled_tools::ToolRegistry;

/// One golden task: a setup, the provider's scripted behaviour, the plan step,
/// and a check on the resulting repository.
struct GoldenTask {
    name: &'static str,
    files: Vec<(&'static str, &'static str)>,
    step: &'static str,
    script: Vec<(&'static str, &'static str, Value)>, // (tool, id, input)
    final_text: &'static str,
    expect: fn(&Path) -> bool,
}

/// The per-task scorecard fields.
#[derive(Debug)]
struct TaskScore {
    name: &'static str,
    success: bool,
    committed: bool,
}

fn git(root: &Path, args: &[&str]) {
    assert!(Command::new("git")
        .args(args)
        .current_dir(root)
        .status()
        .unwrap()
        .success());
}

fn run_task(task: &GoldenTask) -> TaskScore {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    for (rel, contents) in &task.files {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(path, contents).unwrap();
    }
    std::fs::write(
        root.join("PROGRESS.md"),
        format!(
            "# Progress: eval\nBranch: feature/eval\n\n## Steps\n\n- [ ] 1. {}\n",
            task.step
        ),
    )
    .unwrap();

    git(root, &["init"]);
    git(root, &["config", "user.email", "eval@example.com"]);
    git(root, &["config", "user.name", "Eval"]);
    git(root, &["add", "-A"]);
    git(root, &["commit", "-m", "initial"]);

    let mut provider = FakeProvider::new();
    for (tool, id, input) in &task.script {
        provider = provider.tool_call(id, tool, input.clone());
    }
    provider = provider.text(task.final_text);

    let mut runtime = SessionRuntime::new(
        Arc::new(provider),
        ToolRegistry::with_builtins(),
        PermissionEngine::new(Profile::Bypass, Vec::new()),
        Box::new(ScriptedApprover::always()),
        Store::open(root),
        Workspace::new(root).unwrap(),
        RecoveryEngine::new(RecoveryBudget::default()),
        SessionConfig {
            interactivity: Interactivity::NonInteractive,
            trusted: true,
            ..SessionConfig::default()
        },
        Vec::new(),
    );

    let rules = RuleEngine::with_baseline(&Default::default());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let outcome = rt
        .block_on(resume_one_step(&mut runtime, root, &rules, None, 3))
        .unwrap();

    TaskScore {
        name: task.name,
        success: outcome.committed && (task.expect)(root),
        committed: outcome.committed,
    }
}

fn tasks() -> Vec<GoldenTask> {
    vec![
        GoldenTask {
            name: "create a tiny CLI entrypoint",
            files: vec![],
            step: "Create src/main.rs with a hello world",
            script: vec![(
                "write_file",
                "c1",
                json!({ "path": "src/main.rs", "content": "fn main() { println!(\"hi\"); }\n" }),
            )],
            final_text: "created the entrypoint",
            expect: |root| root.join("src/main.rs").exists(),
        },
        GoldenTask {
            name: "fix a failing assertion",
            files: vec![("src/lib.rs", "pub fn two() -> i32 { 1 }\n")],
            step: "Fix two() to return 2",
            script: vec![(
                "edit_file",
                "c1",
                json!({ "path": "src/lib.rs", "old_text": "1", "new_text": "2" }),
            )],
            final_text: "fixed the return value",
            expect: |root| {
                std::fs::read_to_string(root.join("src/lib.rs"))
                    .unwrap()
                    .contains("-> i32 { 2 }")
            },
        },
        GoldenTask {
            name: "edit docs and code together",
            files: vec![
                ("README.md", "# Demo\n\ntodo\n"),
                ("notes.txt", "old note\n"),
            ],
            step: "Update README and notes",
            script: vec![
                (
                    "edit_file",
                    "c1",
                    json!({ "path": "README.md", "old_text": "todo", "new_text": "done" }),
                ),
                (
                    "write_file",
                    "c2",
                    json!({ "path": "notes.txt", "content": "new note\n" }),
                ),
            ],
            final_text: "updated docs and code",
            expect: |root| {
                std::fs::read_to_string(root.join("README.md"))
                    .unwrap()
                    .contains("done")
                    && std::fs::read_to_string(root.join("notes.txt"))
                        .unwrap()
                        .contains("new note")
            },
        },
        // A negative control: the model produces no change, so the step does not
        // complete the work — the scorecard must show this as a failure.
        GoldenTask {
            name: "negative control (no change)",
            files: vec![("keep.txt", "unchanged\n")],
            step: "Should have changed keep.txt but does not",
            script: vec![],
            final_text: "did nothing useful",
            expect: |root| std::fs::read_to_string(root.join("keep.txt")).unwrap() != "unchanged\n",
        },
    ]
}

#[test]
fn golden_task_scorecard() {
    let scores: Vec<TaskScore> = tasks().iter().map(run_task).collect();
    let total = scores.len();
    let passed = scores.iter().filter(|s| s.success).count();
    let rate = passed as f64 / total as f64;

    // Print the scorecard (visible with --nocapture); track the rate over time.
    eprintln!(
        "golden-task scorecard: {passed}/{total} ({:.0}%)",
        rate * 100.0
    );
    for score in &scores {
        eprintln!(
            "  {} success={} committed={}",
            score.name, score.success, score.committed
        );
    }

    // The three real tasks succeed; the negative control fails, so a regression in
    // any real task drops the rate below this expectation.
    assert_eq!(passed, 3, "expected exactly the three real tasks to pass");
    assert!(
        !scores
            .iter()
            .find(|s| s.name.starts_with("negative"))
            .unwrap()
            .success,
        "the negative control must not score as a success"
    );
}

#[test]
fn live_eval_is_gated_behind_an_env_var() {
    if std::env::var("UNSHACKLED_LIVE_TESTS").is_err() {
        eprintln!("skipping live eval: set UNSHACKLED_LIVE_TESTS to enable");
        return;
    }
    // A live eval would build a real provider from config and run a read-only
    // golden task; it is intentionally a no-op without a configured provider.
    eprintln!("live eval mode is enabled but requires a configured provider");
}
