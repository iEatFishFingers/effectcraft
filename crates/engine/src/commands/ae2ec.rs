//! File ▸ Import ▸ After Effects Project (.ae2ec.json)…: the ae2ec importer.
//!
//! ae2ec (<https://github.com/iEatFishFingers/Aep-2-Ecproj>) moves After Effects projects into
//! EffectCraft: its After Effects script describes the open project through After Effects' own
//! scripting API and saves it as an `.ae2ec.json`, and its EffectCraft script rebuilds it through
//! EffectCraft's scripting API. EffectCraft never reads `.aep` files. The EffectCraft script is
//! installed like any other (ae2ec's `tools\install.ps1` puts it in File ▸ Scripts), so the
//! importer and its format change without EffectCraft changing; this menu item only runs it.

use serde_json::Value;

use super::{CommandSpec, always, bad};
use crate::{Result, Session, cmd};

/// The installed ae2ec importer script.
pub const SCRIPT: &str = "Import from After Effects.jsx";

/// Where to get ae2ec.
pub const REPO: &str = "https://github.com/iEatFishFingers/Aep-2-Ecproj";

fn import(s: &mut Session, _: &Value) -> Result<Value> {
    if super::scripts::script_code(s, SCRIPT, Some(false)).is_none() {
        return Err(bad(
            "file.importAfterEffects",
            format!("the ae2ec importer is not installed: get ae2ec from {REPO} and run its tools\\install.ps1, which adds {SCRIPT} to File ▸ Scripts"),
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
