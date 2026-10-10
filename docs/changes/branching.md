# How changes are managed

part of [[index]]

| Branch | Purpose |
|---|---|
| `main` | A clean mirror of the original project. Never committed to. |
| `fix/<name>` | One bug fix, cut from the original's `main`. |
| `feat/<name>` | One feature, cut from the original's `main`. |
| `dev` | This build: every fix and feature merged together. |

## Rules

1. One purpose per branch. If the description needs "and", split it.
2. Each branch builds and passes `cargo xtask ci` on its own against the original's `main`, so any branch
   can be proposed upstream without the others.
3. Each branch carries its own docs: a page in `docs/changes/` that explains why, behaviour, functions
   and tests, plus the rows it changes in [[parity]] and [[gaps]]. The page is linked from [[index]] on `dev`.
4. Branch descriptions say what was wrong and what is different now. They name the problem, not the tool
   that inspired the fix.
5. New work: `git switch -c fix/name upstream/main`, then merge it into `dev` when it is done.

## Current branches

[[layer-double-click-opens-layer]], [[expression-camera-light-attributes]], [[slow-edits-with-image-sequences]],
[[aep-to-ecproj-import]].
