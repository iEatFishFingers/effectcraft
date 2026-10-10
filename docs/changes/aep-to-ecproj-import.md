# Converting `.aep` projects to `.ecproj`

`feat/aep-to-ecproj-import` · feature · part of [[index]] · managed as in [[branching]] · see also
[[layer-double-click-opens-layer]], [[expression-camera-light-attributes]]

## Why

`.aep` files cannot be opened, which is the largest single barrier to moving a project over ([[gaps]],
section G4; [[parity]]). This branch gives users a way across without this app ever reading the `.aep`
format: the original application describes its own project through its scripting API, and a script here
rebuilds it.

## How it works

```
 original application                 JSON                      this application
 ┌──────────────┐  export script  ┌───────────────────┐  import script  ┌───────────────┐
 │  .aep project│ ──────────────► │ project.ae2ec.json│ ──────────────► │ .ecproj / open│
 └──────────────┘                 └───────────────────┘   File ▸ Import │ window        │
                                                                         └───────────────┘
```

1. The export script, run in the original application, writes the project as `<project>.ae2ec.json`.
2. The import script, installed in File ▸ Scripts, rebuilds it here and reports what it could not recreate.
3. This branch adds the pieces the import script needs from the app: the menu entry that runs it, the
   system file dialogs it uses to ask for the export, a way to show its progress while it works, and a
   layout fix so its option window lines up.

The exporter, the importer and the JSON format live in the separate **ae2ec** repository, so they change
without this app changing. This app only runs the installed script.

## Changes on this branch

| Commit | What it adds | Why the converter needs it |
|---|---|---|
| Scripts can show the system's file dialogs | `File.openDialog`, `File.saveDialog`, `Folder.selectDialog` | Scripts returned `null` from every dialog, so the importer could not ask for the export file. |
| Checkboxes and radio buttons line up at the left | Layout fix in script windows | The importer's option window had a column of checkboxes that drifted right by label length. |
| File ▸ Import entry that runs the installed converter | Command `file.importAfterEffects` | One menu entry, shortcut-able and reachable from the CLI and MCP, instead of hunting through File ▸ Scripts. |
| Scripts can show their progress while they work | `Window.update()` | A script held the app until it ended, so a long import left the window frozen and "Not responding". The importer now shows a progress bar, its steps and a Cancel button, and the app keeps drawing. |

## Functions

| Item | File | What it does |
|---|---|---|
| `import` / `specs` (`file.importAfterEffects`) | `crates/engine/src/commands/ae2ec.rs` | Finds the installed script `Import from After Effects.jsx` and runs it; if it is missing, the error says how to install it. Registered in the File ▸ Import menu (`crates/engine/src/menus.rs`). |
| `FileDialogRequest`, `Picked` | `crates/engine/src/scriptui.rs` | The request a waiting script sends to the app and what the user picked. |
| `file_dialog` (`scriptui.fileDialog`) | `crates/engine/src/scriptui.rs` | Hands the script the picked paths, or `null` when cancelled. Agents answer the same way ([[agents]], [[control-protocol]]). |
| `native_file_dialog` | `crates/script/src/ui.rs` | The script-side `File.openDialog` / `saveDialog` / `Folder.selectDialog`; waits like a modal dialog's `show()`. |
| `answer_file_dialog` | `crates/ui-egui/src/panels/scriptui_view.rs` | Shows the system dialog and replies to the waiting script. |
| `native_update` | `crates/script/src/ui.rs` | `Window.update()`: at most every 50 ms, hands the session back to the app with `ScriptUi::busy` set, then carries on at the answer; clicks in the script's windows meanwhile run their handlers. |
| `resume` (`scriptui.resume`), `ScriptUi::busy`, `ScriptUi::frontend` | `crates/engine/src/scriptui.rs` | Lets the busy script carry on; which script is at work; whether a frontend shows file dialogs and progress (the desktop app). |
| `show_windows` | `crates/ui-egui/src/panels/scriptui_view.rs` | While a script is at work: blocks the app behind its windows, draws them, then resumes it and asks for the next frame. |

Reading what the user picked (and writing a file picked in a save dialog) needs no "Allow Scripts to Write
Files" setting, because the user chose the file. Headless (`effectcraft-cli`, MCP), in the Script Console and
on the web the dialogs return `null` at once, and `Window.update()` does nothing. Details for script
authors: [[plugins]].

## What transfers

From the ae2ec project's own testing: folders, footage with its interpretation settings, comps, every layer
type, switches, parenting, track mattes, keyframes with their easing, expressions, effects and their settings,
masks, shapes, text, markers and layer styles. Its report lists anything missing.

Known gaps in that tool: third-party plug-ins, effect data scripts cannot read (such as Curves), per-character
text styling, and animated GIFs arriving as stills. Footage can be relinked afterwards ([[footage]]).

## Tests

- `file_import_after_effects_runs_the_installed_ae2ec_importer` in `crates/script/src/tests_ui.rs`.
- `open_dialog_waits_for_the_app_and_the_picked_file_can_be_read`,
  `cancelled_and_headless_dialogs_return_null`, `save_and_folder_dialogs`, `a_dialog_button_opens_a_file_dialog`
  in `crates/script/src/tests_file_dialogs.rs`.
- `update_shows_progress_and_carries_on_at_resume`, `a_palette_button_works_while_the_script_works`,
  `without_an_app_to_show_it_update_does_nothing` in `crates/script/src/tests_update.rs`.
- File dialog answering and `a_working_script_shows_its_progress_and_the_app_lets_it_carry_on` in
  `crates/ui-egui/tests/ui_scriptui.rs` (the checkbox layout fix has no test of its own).

How to run them: [[testing]].

## Where it is recorded

[[gaps]] (G4), [[plugins]] (file dialogs, progress and the import entry), [[agents]] (the `fileDialog`
and `busy` replies).
