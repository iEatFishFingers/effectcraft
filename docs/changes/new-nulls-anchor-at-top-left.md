# New nulls anchor at their top-left corner, as in After Effects

`fix/new-nulls-anchor-at-top-left` · fix · part of [[index]] · managed as in [[branching]]

## Why

A new null's anchor point was at the centre of its 100×100 square ([50, 50, 0]). After Effects puts it
at the square's top-left corner ([0, 0, 0]). A layer parented to a null is positioned in the null's
layer space, whose origin is that corner, so with a centred anchor every untouched null moved its
children 50 px up and to the left of where After Effects puts them, and a chain of nulls added the
offsets up. A project converted from After Effects with its camera under a chain of 12 untouched nulls
had the camera about 600 px off at the start, filming empty space (x 357, y −57 instead of the comp
centre, x 957, y 543). Scripts that build rigs with `addNull()` the way they do in After Effects hit the
same offset.

## Behaviour

- New nulls have Anchor Point [0, 0, 0], however they're made: Layer ▸ New ▸ Null Object
  (`layer.newNull`), scripting's `layers.addNull()`, and the features that create nulls. Their Position
  is unchanged (the comp centre), so the anchor still sits there, and the null's square is drawn from it
  to the right and down, as in After Effects.
- Features that put a null on a point set its Position, which is where the anchor is, so they still land
  on the point: Create Nulls From Paths (Points Follow Nulls, Nulls Follow Points, Trace Path), the 3D
  Camera Tracker's Create Null, Audio Amplitude. Create Orbit Null reads the null's anchor, and the
  Stereo 3D Controls null isn't drawn.
- Center Anchor Point in Layer Content still moves a null's anchor to the middle of its square
  ([50, 50, 0]) and moves Position so the null stays put.
- Existing projects don't change. Project files store every property's value, so a null saved with the
  old centred anchor opens with it, and everything parented to it stays where it was. The project
  schema is unchanged.
- The demo project's Controller null sets its anchor to [50, 50, 0] itself, so it looks the same.
- A Lottie null without an anchor in the file now gets [0, 0], Lottie's own default, instead of
  [50, 50].

## Functions

| Function | File | What it does |
|---|---|---|
| `build::layer` | `crates/project/src/build.rs` | A null's default anchor is [0, 0], like text, shape, camera, light and 3D model layers, instead of [50, 50]. Layers with a source rectangle (footage, solids, precomps) keep theirs at its centre. |

## Tests

- `new_nulls_anchor_at_their_top_left_corner_as_in_after_effects` in `crates/project/src/tests.rs`: a
  new null's anchor is [0, 0, 0] and its Position the comp centre; a solid's anchor is still its centre.
- `new_nulls_anchor_at_their_top_left_so_children_sit_where_after_effects_puts_them` in
  `crates/engine/src/tests_3d.rs`: a camera at [0, 0, −500] under two untouched nulls (made with
  `layer.newNull`, parented without compensation, as an import sets them) looks from the comp centre,
  not 100 px up and left.
- `nulls_saved_with_the_old_centred_anchor_open_unchanged` in `crates/engine/src/tests_3d.rs`: the same
  rig with the old [50, 50, 0] anchors is saved and opened again; the anchors and the camera stay where
  they were.

The project, render, script, Lottie, interchange and engine suites pass, as do the ui-egui tests that
make nulls. How to run them: [[testing]].

Checked by hand with this build's `effectcraft-cli`: a 355-layer project converted from After Effects and
saved by EffectCraft 0.7.0 opened with its 12 chained nulls still at [50, 50, 0] and its camera where it
was saved, and a null added to it got [0, 0, 0].

## Where it is recorded

Only here for now: the parity and gaps tables are being reworked upstream, so this branch doesn't edit
them.
