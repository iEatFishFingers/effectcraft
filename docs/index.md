# Documentation index

The hub for this build. Pages link to each other with `[[wikilinks]]`: the name between the brackets is a
file name in `docs/` (or `docs/changes/`) without the `.md`.

## Changes in this build

Each change lives on its own branch and can be taken on its own. Its page says why it was needed, what it
does, which functions it touches and which tests cover it.

| Page | Branch | Type | Status |
|---|---|---|---|
| [[layer-double-click-opens-layer]] | `fix/layer-double-click-opens-layer` | fix | CI pending |
| [[expression-camera-light-attributes]] | `fix/expression-camera-light-attributes` | fix | CI pending |
| [[aep-to-ecproj-import]] | `feat/aep-to-ecproj-import` | feature | CI pending |

How the branches are managed: [[branching]].

## Project documentation

| Page | Covers |
|---|---|
| [[architecture]] | Crate layering and the rules between layers |
| [[agents]] | Driving the app from a script or an AI agent |
| [[control-protocol]] | The command and automation interface |
| [[mcp]] | The MCP server |
| [[plugins]] | Scripts and extensions |
| [[effects]] | The effect catalogue |
| [[footage]] | Importing and handling footage |
| [[preferences]] | Settings |
| [[web]] | The web build |
| [[releasing]] | Cutting a release |
| [[testing]] | Tests, headless snapshots and CI |
| [[parity]] | Feature-by-feature checklist |
| [[gaps]] | What is missing, with evidence |
| [[contributors]] | Who built what |
