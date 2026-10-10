# Edits are no longer slow in projects with image sequences

`fix/slow-edits-with-image-sequences` · fix · part of [[index]] · managed as in [[branching]]

## Why

With the disk cache on (the default), every edit read the size and modification time of every footage
file from disk, down to each frame of every image sequence. In a project with long image sequences that
cost tens of milliseconds per edit: 45 ms for a project with 18 footage items, most of them sequences.
Anything that makes many edits in a row slowed down accordingly. A script that rebuilt a 101-layer project
made about 2,000 edits and took 67 s instead of 3, with the window frozen meanwhile.

The disk cache needs those reads so that footage changed outside the app gets new cache keys
([[footage]]). It doesn't need them after every edit.

## Behaviour

The footage salt (the footage files' identity in the disk cache's keys) is read again only when:

- footage items change (imported, removed, replaced, reinterpreted, a sequence's frames), at once; or
- a second has passed since it was last read, at the next edit, so footage changed outside the app is
  still noticed.

An edit that leaves footage alone reads nothing from disk. Nothing changes for users without the disk
cache.

## Functions

| Function | File | What it does |
|---|---|---|
| `Session::bump` | `crates/engine/src/lib.rs` | After every edit: now takes the salt from `disk_salt`. |
| `Session::disk_salt` | `crates/engine/src/lib.rs` | Reuses the last salt while footage items are unchanged and it is less than a second old; otherwise reads it again (`footage_salt`) and keeps it in `Session::footage_salt`. |
| `same_footage(a, b)` | `crates/engine/src/lib.rs` | Whether two projects have the same footage items, compared in memory (ids, settings and sequence frames). |

## Tests

`edits_reuse_the_footage_salt_until_footage_changes_or_a_second_passes` in
`crates/engine/src/tests_disk_cache.rs`: edits that leave footage alone reuse the salt, a footage change
reads it at once, and an edit a second later reads it again. The other disk cache tests in that file still
pass, including `footage_changes_change_the_salt`. How to run them: [[testing]].

Measured on the 101-layer project with the disk cache on: 67.5 s before, 3.7 s after.

## Where it is recorded

[[gaps]] (Performance).
