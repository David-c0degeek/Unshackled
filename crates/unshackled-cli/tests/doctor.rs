use assert_cmd::Command;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn doctor_reports_foundation_status() -> Result<(), Box<dyn Error>> {
    let workspace = TempDir::new("doctor-workspace")?;
    let config_home = TempDir::new("doctor-config")?;
    let workspace_path = workspace.path().canonicalize()?;
    fs::write(
        workspace.path().join(".unshackled.toml"),
        "provider = \"local\"\n",
    )?;

    let output = doctor_output(&workspace_path, config_home.path())?;
    let normalized = normalize_output(&output, &workspace_path, config_home.path());

    let expected = include_str!("snapshots/doctor.snap").trim_end_matches('\n');
    assert_eq!(normalized, expected);

    Ok(())
}

#[test]
fn doctor_does_not_print_secret_values() -> Result<(), Box<dyn Error>> {
    let workspace = TempDir::new("doctor-secret-workspace")?;
    let config_home = TempDir::new("doctor-secret-config")?;
    fs::write(
        workspace.path().join(".unshackled.toml"),
        "api_key = \"secret-from-config\"\n",
    )?;

    let mut cmd = Command::cargo_bin("unshackled")?;
    let output = cmd
        .arg("doctor")
        .current_dir(workspace.path())
        .env("XDG_CONFIG_HOME", config_home.path())
        .env("APPDATA", config_home.path())
        .env("HOME", config_home.path())
        .env("OPENAI_API_KEY", "secret-from-env")
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    assert!(!stdout.contains("secret-from-config"));
    assert!(!stdout.contains("secret-from-env"));
    assert!(!stderr.contains("secret-from-config"));
    assert!(!stderr.contains("secret-from-env"));

    Ok(())
}

fn doctor_output(workspace: &Path, config_home: &Path) -> Result<String, Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("unshackled")?;
    let output = cmd
        .arg("doctor")
        .current_dir(workspace)
        .env("XDG_CONFIG_HOME", config_home)
        .env("APPDATA", config_home)
        .env("HOME", config_home)
        .output()?;

    assert!(output.status.success());
    Ok(String::from_utf8(output.stdout)?)
}

fn normalize_output(output: &str, workspace: &Path, config_home: &Path) -> String {
    let workspace = workspace.display().to_string();
    let config_home = config_home.display().to_string();

    output
        .lines()
        .map(|line| {
            if line.starts_with("platform: ") {
                "platform: <platform>".to_string()
            } else if line.starts_with("  git: ") {
                "  git: <tool-status>".to_string()
            } else if line.starts_with("  rg: ") {
                "  rg: <tool-status>".to_string()
            } else {
                line.replace(&workspace, "<workspace>")
                    .replace(&config_home, "<config-home>")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> Result<Self, Box<dyn Error>> {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let path = std::env::temp_dir().join(format!(
            "unshackled-{prefix}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
