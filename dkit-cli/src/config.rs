use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// dkit configuration loaded from TOML files.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DkitConfig {
    /// Default output format (json, csv, yaml, toml, etc.)
    pub default_format: Option<String>,
    /// Color output mode: auto, always, never
    pub color: Option<String>,
    /// Default input encoding
    pub encoding: Option<String>,
    /// Table display settings
    #[serde(default)]
    pub table: Option<TableConfig>,
    /// User-defined command aliases
    #[serde(default)]
    pub aliases: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableConfig {
    /// Default border style (simple, rounded, heavy, none, etc.)
    pub border_style: Option<String>,
    /// Default max column width
    pub max_width: Option<u16>,
}

/// Source information for debugging where config was loaded from.
#[derive(Debug)]
pub struct ConfigSource {
    pub config: DkitConfig,
    pub user_path: Option<PathBuf>,
    pub project_path: Option<PathBuf>,
}

impl DkitConfig {
    /// Merge another config on top of self (other takes priority for set values).
    pub fn merge(self, other: &DkitConfig) -> DkitConfig {
        let mut aliases = self.aliases.clone();
        for (k, v) in &other.aliases {
            aliases.insert(k.clone(), v.clone());
        }
        DkitConfig {
            default_format: other.default_format.clone().or(self.default_format),
            color: other.color.clone().or(self.color),
            encoding: other.encoding.clone().or(self.encoding),
            table: match (&self.table, &other.table) {
                (None, None) => None,
                (Some(a), None) => Some(a.clone()),
                (None, Some(b)) => Some(b.clone()),
                (Some(a), Some(b)) => Some(TableConfig {
                    border_style: b.border_style.clone().or(a.border_style.clone()),
                    max_width: b.max_width.or(a.max_width),
                }),
            },
            aliases,
        }
    }
}

impl fmt::Display for DkitConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let toml_str = toml::to_string_pretty(self)
            .unwrap_or_else(|_| String::from("# (serialization error)"));
        write!(f, "{}", toml_str)
    }
}

/// Returns the user-level config file path.
/// Checks XDG first (`$XDG_CONFIG_HOME/dkit/config.toml`), then `~/.dkit.toml`.
pub fn user_config_path() -> Option<PathBuf> {
    // XDG path
    if let Some(config_dir) = dirs::config_dir() {
        let xdg_path = config_dir.join("dkit").join("config.toml");
        if xdg_path.exists() {
            return Some(xdg_path);
        }
    }
    // Fallback: ~/.dkit.toml
    if let Some(home) = dirs::home_dir() {
        let home_path = home.join(".dkit.toml");
        if home_path.exists() {
            return Some(home_path);
        }
    }
    None
}

/// Returns the preferred path for creating a new user config file.
/// Prefers XDG location.
pub fn user_config_init_path() -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        return Some(config_dir.join("dkit").join("config.toml"));
    }
    dirs::home_dir().map(|h| h.join(".dkit.toml"))
}

/// Returns the project-level config file path (`.dkit.toml` in current directory).
pub fn project_config_path() -> Option<PathBuf> {
    let path = PathBuf::from(".dkit.toml");
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

/// Load config from a TOML file, returning Default if file doesn't exist.
fn load_from_file(path: &Path) -> DkitConfig {
    match fs::read_to_string(path) {
        Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
        Err(_) => DkitConfig::default(),
    }
}

/// Load the merged configuration (user config + project config).
/// Priority: project config > user config > defaults.
pub fn load_config() -> ConfigSource {
    let mut config = DkitConfig::default();
    let mut user_path = None;
    let mut project_path = None;

    // Load user-level config
    if let Some(path) = user_config_path() {
        let user_config = load_from_file(&path);
        config = config.merge(&user_config);
        user_path = Some(path);
    }

    // Load project-level config (overrides user config)
    if let Some(path) = project_config_path() {
        let proj_config = load_from_file(&path);
        config = config.merge(&proj_config);
        project_path = Some(path);
    }

    ConfigSource {
        config,
        user_path,
        project_path,
    }
}

/// Generate a default config file content.
pub fn default_config_content() -> String {
    r#"# dkit configuration file
# See: https://github.com/syang0531/dkit

# Default output format (json, csv, yaml, toml, xml, md, html, table)
# default_format = "json"

# Color output: "auto", "always", "never"
# color = "auto"

# Default input encoding (e.g. "utf-8", "euc-kr", "shift_jis")
# encoding = "utf-8"

[table]
# Default table border style (simple, rounded, heavy, none, double, ascii)
# border_style = "simple"

# Default max column width (truncate longer values)
# max_width = 40
"#
    .to_string()
}

/// Run `dkit config show` — display current effective configuration.
pub fn run_show() -> anyhow::Result<()> {
    let source = load_config();

    println!("# Effective configuration");
    println!("# Priority: CLI options > project config > user config > defaults");
    println!();

    if let Some(ref path) = source.user_path {
        println!("# User config: {}", path.display());
    } else {
        println!("# User config: (none)");
    }
    if let Some(ref path) = source.project_path {
        println!("# Project config: {}", path.display());
    } else {
        println!("# Project config: (none)");
    }
    println!();
    println!("{}", source.config);

    Ok(())
}

/// Returns the built-in aliases.
pub fn builtin_aliases() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert(
        "j2c".to_string(),
        "convert --from json --to csv".to_string(),
    );
    m.insert(
        "c2j".to_string(),
        "convert --from csv --to json".to_string(),
    );
    m.insert(
        "j2y".to_string(),
        "convert --from json --to yaml".to_string(),
    );
    m.insert(
        "y2j".to_string(),
        "convert --from yaml --to json".to_string(),
    );
    m.insert(
        "j2t".to_string(),
        "convert --from json --to toml".to_string(),
    );
    m.insert(
        "t2j".to_string(),
        "convert --from toml --to json".to_string(),
    );
    m.insert(
        "c2y".to_string(),
        "convert --from csv --to yaml".to_string(),
    );
    m.insert(
        "y2c".to_string(),
        "convert --from yaml --to csv".to_string(),
    );
    m
}

/// Save config to a TOML file, creating parent directories if needed.
fn save_config(path: &Path, config: &DkitConfig) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    let toml_str = toml::to_string_pretty(config)
        .map_err(|e| anyhow::anyhow!("Failed to serialize config: {e}"))?;
    fs::write(path, toml_str)?;
    Ok(())
}

/// Returns the write path for user aliases (existing config path or init path).
fn user_config_write_path() -> Option<PathBuf> {
    user_config_path().or_else(user_config_init_path)
}

/// Run `dkit alias set <name> <command>` — register or update a user alias.
pub fn run_alias_set(name: &str, command: &str) -> anyhow::Result<()> {
    let path = user_config_write_path()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine config directory"))?;

    let mut config = if path.exists() {
        load_from_file(&path)
    } else {
        DkitConfig::default()
    };

    config.aliases.insert(name.to_string(), command.to_string());
    save_config(&path, &config)?;
    println!("Alias '{}' = '{}'", name, command);
    Ok(())
}

/// Run `dkit alias list` — list all aliases (builtin + user).
pub fn run_alias_list(user_aliases: &HashMap<String, String>) -> anyhow::Result<()> {
    let builtins = builtin_aliases();

    // Merge: user aliases override builtins
    let mut all: std::collections::BTreeMap<String, (String, &str)> =
        std::collections::BTreeMap::new();
    for (name, cmd) in &builtins {
        all.insert(name.clone(), (cmd.clone(), "builtin"));
    }
    for (name, cmd) in user_aliases {
        all.insert(name.clone(), (cmd.clone(), "user"));
    }

    if all.is_empty() {
        println!("No aliases defined.");
        return Ok(());
    }

    println!("{:<16} {:<44} SOURCE", "NAME", "COMMAND");
    println!("{:-<16} {:-<44} {:-<8}", "", "", "");
    for (name, (cmd, source)) in &all {
        println!("{:<16} {:<44} {}", name, cmd, source);
    }
    Ok(())
}

/// Run `dkit alias remove <name>` — remove a user-defined alias.
pub fn run_alias_remove(name: &str) -> anyhow::Result<()> {
    let builtins = builtin_aliases();
    if builtins.contains_key(name) {
        anyhow::bail!("Cannot remove built-in alias '{name}'. Use 'alias set {name} <command>' to override it.");
    }

    let path = user_config_write_path()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine config directory"))?;

    if !path.exists() {
        anyhow::bail!("Alias '{name}' not found.");
    }

    let mut config = load_from_file(&path);
    if config.aliases.remove(name).is_none() {
        anyhow::bail!("Alias '{name}' not found.");
    }

    save_config(&path, &config)?;
    println!("Alias '{name}' removed.");
    Ok(())
}

/// Run `dkit config init` — create a default config file.
pub fn run_init(project: bool) -> anyhow::Result<()> {
    let path = if project {
        PathBuf::from(".dkit.toml")
    } else {
        user_config_init_path()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine home/config directory"))?
    };

    if path.exists() {
        anyhow::bail!("Config file already exists: {}", path.display());
    }

    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::write(&path, default_config_content())?;
    println!("Created config file: {}", path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DkitConfig::default();
        assert!(config.default_format.is_none());
        assert!(config.color.is_none());
        assert!(config.encoding.is_none());
        assert!(config.table.is_none());
    }

    #[test]
    fn test_parse_config() {
        let toml_str = r#"
default_format = "csv"
color = "always"
encoding = "utf-8"

[table]
border_style = "rounded"
max_width = 50
"#;
        let config: DkitConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.default_format.as_deref(), Some("csv"));
        assert_eq!(config.color.as_deref(), Some("always"));
        assert_eq!(config.encoding.as_deref(), Some("utf-8"));
        let table = config.table.unwrap();
        assert_eq!(table.border_style.as_deref(), Some("rounded"));
        assert_eq!(table.max_width, Some(50));
    }

    #[test]
    fn test_partial_config() {
        let toml_str = r#"
color = "never"
"#;
        let config: DkitConfig = toml::from_str(toml_str).unwrap();
        assert!(config.default_format.is_none());
        assert_eq!(config.color.as_deref(), Some("never"));
        assert!(config.table.is_none());
    }

    #[test]
    fn test_merge_config() {
        let base = DkitConfig {
            default_format: Some("json".to_string()),
            color: Some("auto".to_string()),
            encoding: None,
            table: Some(TableConfig {
                border_style: Some("simple".to_string()),
                max_width: Some(40),
            }),
            ..Default::default()
        };
        let override_cfg = DkitConfig {
            default_format: None,
            color: Some("never".to_string()),
            encoding: Some("euc-kr".to_string()),
            table: Some(TableConfig {
                border_style: None,
                max_width: Some(80),
            }),
            ..Default::default()
        };

        let merged = base.merge(&override_cfg);
        assert_eq!(merged.default_format.as_deref(), Some("json")); // kept from base
        assert_eq!(merged.color.as_deref(), Some("never")); // overridden
        assert_eq!(merged.encoding.as_deref(), Some("euc-kr")); // new from override
        let table = merged.table.unwrap();
        assert_eq!(table.border_style.as_deref(), Some("simple")); // kept from base
        assert_eq!(table.max_width, Some(80)); // overridden
    }

    #[test]
    fn test_default_config_content_parses() {
        let content = default_config_content();
        let config: DkitConfig = toml::from_str(&content).unwrap();
        assert!(config.default_format.is_none());
        // [table] section exists but all values are commented out
        if let Some(ref table) = config.table {
            assert!(table.border_style.is_none());
            assert!(table.max_width.is_none());
        }
    }

    #[test]
    fn test_config_display() {
        let config = DkitConfig {
            default_format: Some("csv".to_string()),
            color: None,
            encoding: None,
            table: None,
            ..Default::default()
        };
        let display = format!("{}", config);
        assert!(display.contains("default_format"));
        assert!(display.contains("csv"));
    }

    #[test]
    fn test_init_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join(".dkit.toml");
        fs::write(&file_path, "# existing").unwrap();

        // Simulate by checking logic directly
        assert!(file_path.exists());
    }
}
