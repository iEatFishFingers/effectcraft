# Expressions read camera, light and time remap values

`fix/expression-camera-light-attributes` · fix · part of [[index]] · see also
[[layer-double-click-opens-layer]]

## Why

Expressions such as `thisComp.activeCamera.zoom` or `thisComp.layer("Key").intensity` returned
`undefined`, which rendered as 0. Anything driven by a camera, a light or a time remap silently broke. A
check of 28 common expressions found 19 that read differently from the documented results; these are
the first group fixed.

## What reads now

| On a layer | Attributes |
|---|---|
| Camera | `zoom`, `pointOfInterest`, `depthOfField` (a switch: its value, so `depthOfField ? a : b` works), `focusDistance`, `aperture`, `blurLevel`, `irisShape`, `irisRotation`, `irisRoundness`, `irisAspectRatio`, `irisDiffractionFringe`, `highlightGain`, `highlightThreshold`, `highlightSaturation` |
| Light | `intensity`, `color`, `coneAngle`, `coneFeather`, `shadowDarkness`, `shadowDiffusion` |
| Any layer | `timeRemap` |
| Composition | `shutterAngle`, `shutterPhase` |

A layer without the property names it in the error instead of reading as `undefined`.

## Functions

| Item | File | What it does |
|---|---|---|
| Layer getters (`zoom`, `intensity`, `timeRemap`, ...) | `crates/expr/src/prelude.js` | Read the value straight off the layer's camera option, light option, transform or time remap group. |
| Comp getters `shutterAngle`, `shutterPhase` | `crates/expr/src/prelude.js` | Read slots 9 and 10 of the comp info block. |
| Comp info responses | `crates/expr/src/host.rs` | Sends the comp's shutter angle and phase to the script side. |

## Still differing

Not fixed here: a layer or property group called with a name or match name, `propertyGroup(n)`,
`sourceTime()`, `source.numLayers`, `fromCompToSurface()`. `layer.orientation` and `layer.rotationX`
still need checking. They are tracked in [[gaps]].

## Tests

`camera_light_time_remap_and_shutter_attributes` in `crates/expr/src/tests.rs` ([[testing]]).

## Where it is recorded

[[parity]] (Animation row), [[gaps]] (expression check of 10 October). Layering rules:
[[architecture]].
