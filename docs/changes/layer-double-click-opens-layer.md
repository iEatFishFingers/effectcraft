# Double-clicking a layer opens it

`fix/layer-double-click-opens-layer` · fix · part of [[index]] · see also
[[expression-camera-light-attributes]]

## Why

Double-clicking a layer's bar in the Timeline did nothing for a precomp, so the precomp could only be
opened from the Composition panel's navigator. In the Composition panel, a double-click on a precomp
sitting in front of a title edited the title instead of opening the precomp. A double-click should open
whatever was clicked, the same way in both panels.

## Behaviour

The same rule applies to a layer's name and its bar in the Timeline.

| Layer | Double-click | Alt + double-click | Command |
|---|---|---|---|
| Precomp | opens its composition | opens it in the Layer panel | `comp.open` / `layer.openLayer` |
| Footage, solid | opens in the Layer panel | same | `layer.openLayer` |
| Camera, light | opens its settings | same | `layer.settings` |
| Text | edits the text, all selected | same | `text.edit` |
| Anything else | renames (name only) | same | `layer.rename` |

In the Composition panel the layer opened is the one a click at that point would select: a selected layer
under the pointer, otherwise the frontmost. A precomp in front of a title opens; the title is no longer
edited by mistake.

## Functions

| Function | File | What it does |
|---|---|---|
| `open_on_double_click(layer, alt)` | `crates/ui-egui/src/panels/timeline.rs` | Maps a layer and the Alt modifier to the command and parameters to run, or `None` to rename. Used by both the name and the bar. |
| `covers(ectx, layer, point)` | `crates/ui-egui/src/panels/viewer.rs` | Whether a layer's content box covers a composition point, i.e. what a click there hits. `pick` is built on it. |
| `text_layer_to_open(...)` | `crates/ui-egui/src/panels/viewer_text.rs` | The text layer a double-click edits, only when the layer a click there acts on is a text layer. |

Every action goes through the engine's command layer, so the CLI and the MCP server see the same commands
([[control-protocol]], [[agents]]). The bar is reachable as the automation id
`timeline.layer.<id>.bar`.

## Tests

`crates/ui-egui/tests/ui_layer_double_click.rs`, run headlessly ([[testing]]):

- `double_clicking_a_precomp_bar_or_name_opens_it`
- `double_clicking_cameras_lights_and_text_in_the_timeline`
- `double_clicking_layers_in_the_composition_panel` (a precomp in front of a title)

## Where it is recorded

[[parity]] (Interface row), [[gaps]] (user reports of 10 October), the README's Timeline paragraph.
Layering rules: [[architecture]].
