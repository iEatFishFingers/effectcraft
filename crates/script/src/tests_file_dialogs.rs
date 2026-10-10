//! `File.openDialog`, `File.saveDialog` and `Folder.selectDialog`: in an app that shows file
//! dialogs the script waits (the session comes back meanwhile) until `scriptui.fileDialog`
//! answers; what the user picked may be read without the Allow Scripts to Write Files preference
//! (writing still needs it, a file picked in a save dialog too, as in After Effects). Only the app
//! or an agent answers a dialog, never a script. Elsewhere they return null at once.

use std::path::PathBuf;
use std::sync::Arc;

use effectcraft_engine::Session;
use serde_json::json;

use crate::run_code;

fn session() -> Session {
    let mut s = Session { expr: Some(Arc::new(effectcraft_expr::Expressions)), ..Default::default() };
    crate::install(&mut s);
    s.script_ui.frontend = true;
    s
}

/// A fresh folder with `export.json` and `other.json`.
fn folder(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("ec-file-dialogs-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    std::fs::write(d.join("export.json"), r#"{"a": 1}"#).unwrap();
    std::fs::write(d.join("other.json"), r#"{"a": 2}"#).unwrap();
    d
}

fn js(p: &std::path::Path) -> String {
    serde_json::to_string(&p.to_string_lossy()).unwrap()
}

const OPEN: &str = r#"
writeLn("asking");
var f = File.openDialog("Pick an export", "Exports:*.json");
if (f) { f.open("r"); var t = f.read(); f.close(); writeLn(f.name + " " + JSON.parse(t).a); } else { writeLn("cancelled"); }
"#;

#[test]
fn open_dialog_waits_for_the_app_and_the_picked_file_can_be_read() {
    let d = folder("open");
    let mut s = session();
    assert!(!s.prefs.scripting.allow_scripts_write_files);
    let o = run_code(&mut s, OPEN, "open.jsx");
    assert!(o.waiting, "{o:?}");
    assert_eq!(o.output, ["asking"]);
    let req = s.script_ui.file_dialog.clone().unwrap();
    assert_eq!((req.kind.as_str(), req.prompt.as_str(), req.extensions.clone(), req.multi), ("open", "Pick an export", vec!["json".to_string()], false));
    // The reply says what it waits for (agents answer with scriptui.fileDialog).
    assert_eq!(o.file_dialog.as_ref(), Some(&req));
    assert_eq!(o.to_json()["fileDialog"]["kind"], "open");
    let r = s.execute("scriptui.fileDialog", json!({"paths": [d.join("export.json")]})).unwrap();
    assert_eq!(r["output"], "export.json 1", "{r}");
    assert!(r.get("waiting").is_none() && s.script_ui.file_dialog.is_none());
    // Only what was picked: its neighbour still needs the preference.
    let o = run_code(&mut s, &format!("var f = new File({}); f.open('r'); f.read()", js(&d.join("other.json"))), "other.jsx");
    assert!(o.error.as_ref().is_some_and(|e| e.message.contains("Allow Scripts to Write Files")), "{o:?}");
    let o = run_code(&mut s, &format!("var f = new File({}); f.open('r'); f.read()", js(&d.join("export.json"))), "again.jsx");
    assert!(o.error.is_none(), "{o:?}");
    // Nothing is waiting any more.
    assert!(s.execute("scriptui.fileDialog", json!({"paths": null})).is_err());
}

#[test]
fn cancelled_and_headless_dialogs_return_null() {
    let mut s = session();
    let o = run_code(&mut s, OPEN, "open.jsx");
    assert!(o.waiting);
    let r = s.execute("scriptui.fileDialog", json!({"paths": null})).unwrap();
    assert_eq!(r["output"], "cancelled", "{r}");
    // Without an app to show them (the CLI, MCP), the dialogs return null at once.
    s.script_ui.frontend = false;
    let o = run_code(&mut s, "[File.openDialog('x'), File.saveDialog('x'), Folder.selectDialog('x')].join(',') === ',,'", "headless.jsx");
    assert!(!o.waiting && o.error.is_none(), "{o:?}");
    assert_eq!(o.result, json!(true));
    assert!(s.script_ui.file_dialog.is_none());
}

#[test]
fn save_and_folder_dialogs() {
    const SAVE: &str = r#"var f = File.saveDialog("Save the report", "JSON:*.json"); f.open("w"); f.write("done"); f.close(); f.name"#;
    let d = folder("save");
    let mut s = session();
    let o = run_code(&mut s, SAVE, "save.jsx");
    assert!(o.waiting, "{o:?}");
    let req = s.script_ui.file_dialog.clone().unwrap();
    assert_eq!((req.kind.as_str(), req.extensions.clone()), ("save", vec!["json".to_string()]));
    // Picking where to save doesn't let the script write there: that still needs Allow Scripts to
    // Write Files, as in After Effects.
    let r = s.execute("scriptui.fileDialog", json!({"paths": [d.join("report.json")]})).unwrap();
    assert_eq!(r["ok"], false, "{r}");
    assert!(r["error"]["message"].as_str().is_some_and(|m| m.contains("Allow Scripts to Write Files")), "{r}");
    assert!(!d.join("report.json").exists());
    // With it on, the script saves where the user picked.
    s.prefs.scripting.allow_scripts_write_files = true;
    let o = run_code(&mut s, SAVE, "save.jsx");
    assert!(o.waiting, "{o:?}");
    let r = s.execute("scriptui.fileDialog", json!({"paths": [d.join("report.json")]})).unwrap();
    assert_eq!(r["result"], "report.json", "{r}");
    assert_eq!(std::fs::read_to_string(d.join("report.json")).unwrap(), "done");
    s.prefs.scripting.allow_scripts_write_files = false;
    // A file picked to open can't be written either.
    let o = run_code(&mut s, OPEN, "open.jsx");
    assert!(o.waiting);
    s.execute("scriptui.fileDialog", json!({"paths": [d.join("export.json")]})).unwrap();
    let o = run_code(&mut s, &format!("var f = new File({}); f.open('w'); f.write('x')", js(&d.join("export.json"))), "write.jsx");
    assert!(o.error.is_some(), "{o:?}");
    // A folder: what is in it can be listed and read.
    let o = run_code(&mut s, r#"var d = Folder.selectDialog("Footage folder"); d.getFiles("*.json").length"#, "folder.jsx");
    assert!(o.waiting && s.script_ui.file_dialog.as_ref().is_some_and(|r| r.kind == "folder"));
    let r = s.execute("scriptui.fileDialog", json!({"paths": [d]})).unwrap();
    assert_eq!(r["result"], 3, "{r}");
}

const BROWSE: &str = r#"
var d = new Window("dialog", "Import");
var path = d.add("edittext", undefined, "");
var browse = d.add("button", undefined, "Browse...");
browse.onClick = function () { var f = File.openDialog("Pick", "*.json"); if (f) path.text = f.fsName; };
d.add("button", undefined, "OK", { name: "ok" });
var r = d.show();
r + " " + path.text.split(/[\/\\]/).pop()
"#;

#[test]
fn a_dialog_button_opens_a_file_dialog() {
    let d = folder("browse");
    let mut s = session();
    let o = run_code(&mut s, BROWSE, "browse.jsx");
    assert!(o.waiting && s.script_ui.windows.len() == 1);
    // Browse… asks for a file: the dialog waits for it.
    let r = s.execute("scriptui.click", json!({"widget": "Browse..."})).unwrap();
    assert_eq!(r["waiting"], true, "{r}");
    assert_eq!(r["fileDialog"]["prompt"], "Pick", "{r}");
    assert!(s.script_ui.file_dialog.is_some());
    assert!(s.execute("scriptui.click", json!({"widget": "ok"})).is_err(), "the dialog waits for its file dialog");
    s.execute("scriptui.fileDialog", json!({"paths": [d.join("other.json")]})).unwrap();
    let w = s.script_ui.windows[0].clone();
    assert!(w.root.children[0].text.ends_with("other.json"), "{:?}", w.root.children[0].text);
    let r = s.execute("scriptui.click", json!({"widget": "ok"})).unwrap();
    assert_eq!(r["result"], "1 other.json", "{r}");
    assert!(s.script_ui.windows.is_empty());
}

/// Only the app (or an agent) answers a file dialog. A script running meanwhile (a palette's
/// handler, a scheduled task) can't: it would pick a file for itself, or for another script, to
/// read without the preference.
#[test]
fn scripts_cannot_answer_a_file_dialog() {
    let d = folder("answer");
    let mut s = session();
    let o = run_code(&mut s, OPEN, "open.jsx");
    assert!(o.waiting, "{o:?}");
    let other = js(&d.join("other.json"));
    for code in [
        format!("__call('scriptui.fileDialog', {{paths: [{other}]}})"),
        format!("__exec('scriptui.fileDialog', JSON.stringify({{paths: [{other}]}}))"),
        format!("__call('engine.batch', {{steps: [{{command: 'scriptui.fileDialog', params: {{paths: [{other}]}}}}]}})"),
    ] {
        let o = run_code(&mut s, &code, "answer.jsx");
        assert!(o.error.is_some(), "{code}: {o:?}");
        assert!(s.script_ui.file_dialog.is_some(), "{code}: the dialog still waits");
    }
    // Nothing was granted: other.json still needs the preference.
    let o = run_code(&mut s, &format!("var f = new File({other}); f.open('r'); f.read()"), "read.jsx");
    assert!(o.error.as_ref().is_some_and(|e| e.message.contains("Allow Scripts to Write Files")), "{o:?}");
    // The app answers it.
    let r = s.execute("scriptui.fileDialog", json!({"paths": null})).unwrap();
    assert_eq!(r["output"], "cancelled", "{r}");
}
