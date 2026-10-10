# Converting `.aep` projects to `.ecproj`

`feat/aep-to-ecproj-import` · feature · **in progress** · part of [[index]] · managed as in [[branching]]

> Placeholder. This page is completed when the branch lands. Sections marked TODO are filled from the
> finished work, not guessed.

## Why

`.aep` files cannot be opened, which is the largest single barrier to moving a project over ([[gaps]],
[[parity]]).

## How it works

The conversion runs in two steps through an intermediate JSON description of the project:

1. An export script run in the original application writes the project as JSON.
2. An import script, run in this application's scripting model, rebuilds it as an `.ecproj` and writes a
   report of anything it could not recreate.

Scripting model: [[plugins]]. Driving the import from outside the window: [[agents]], [[control-protocol]],
[[mcp]].

## TODO when the branch is ready

- The commands and menu entries it adds, with their ids (see [[architecture]] for the command rules).
- Functions and files touched, per crate.
- What transfers and what does not, and how the report is read.
- Tests ([[testing]]) and how to run the round trip.
- The rows to change in [[parity]] and [[gaps]].
- Footage relinking ([[footage]]).
