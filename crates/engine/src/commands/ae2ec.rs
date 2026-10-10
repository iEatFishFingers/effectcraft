//! File ▸ Import ▸ After Effects Project (.ae2ec.json)…: the ae2ec importer.
//!
//! ae2ec moves After Effects projects into EffectCraft: its After Effects script exports a
//! project as an `.ae2ec.json`, and its EffectCraft script rebuilds it. That script is installed
//! like any other (its `tools\install.ps1` puts it in File ▸ Scripts), so the importer and the
//! format change without EffectCraft changing; this menu item only runs it.

use serde_json::Value;

use super::{CommandSpec, always, bad};
use crate::{Result, Session, cmd};

/// The installed ae2ec importer script.
pub const SCRIPT: &str = "Import from After Effects.jsx";

fn import(s: &mut Session, _: &Value) -> Result<Value> {
    if super::scripts::script_code(s, SCRIPT, Some(false)).is_none() {
        return Err(bad(
            "file.importAfterEffects",
            format!("the ae2ec importer is not installed: run tools\\install.ps1 from the ae2ec repository, which adds {SCRIPT} to File ▸ Scripts"),
        ));
    }
    super::scripts::run_named(s, SCRIPT)
}

pub fn specs() -> Vec<CommandSpec> {
    vec![cmd!(
        "file.importAfterEffects",
        "After Effects Project (.ae2ec.json)...",
        ["File", "Import"],
        None,
        "{} → runs the installed `Import from After Effects.jsx` (the ae2ec importer), which asks for an .ae2ec.json exported from After Effects",
        always,
        import
    )]
}
