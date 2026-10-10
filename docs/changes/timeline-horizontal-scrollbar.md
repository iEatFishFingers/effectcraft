# The time graph scrolls sideways with a scroll bar

`feat/timeline-horizontal-scrollbar` · feature · part of [[index]] · managed as in [[branching]]

## Why

Zoomed in on a long comp, the Timeline showed a slice of it with nothing that looked like a way
across. The layers had a scroll bar beside them and the outline columns one under them, but the time
graph didn't. Moving along it needed the time navigator (a thin bar above the ruler that is also a
zoom control), Shift+wheel or a trackpad, or Spacebar-dragging. Shift+wheel and the trackpad also
kept going past the comp's end, into empty time.

## Behaviour

- While the time graph shows less than the whole comp, a scroll bar runs along the top of the
  Timeline's footer, under the graph. Its thumb is the visible span. Dragging it, or pressing the track
  beside it, moves the graph through the comp at the same zoom. The layers, the current time and the
  work area stay where they are.
- It stops with the comp's end at the graph's right edge.
- Shift+wheel and sideways trackpad scrolling over the time graph stop there too, as Spacebar-dragging
  already did.
- At the whole comp (no zoom, `;`, or the time navigator stretched to both ends) the bar is hidden, as
  the other scroll bars hide when there is nothing to scroll.

Agents find it as `timeline.timeScroll`, labelled `scroll/max` in pixels like the Timeline's
other scroll bars (`timeline.scroll`, `timeline.outlineScroll`).

## Functions

| Item | File | What it does |
|---|---|---|
| `show` (the Timeline panel) | `crates/ui-egui/src/panels/timeline.rs` | Works out `last_start`, the latest the graph scrolls to, once a frame. It draws the bar with the shared `widgets::scroll_bar` after the zoom slider, so the strip is the bar's. It clamps Shift+wheel / trackpad and Spacebar-drag scrolling to `last_start`. |

Nothing new below the UI: the visible span is the Timeline's existing UI state
(`TimelineState::start` / `pps`), as the navigator and the Hand use it.

## Tests

`the_time_graph_scrolls_sideways_with_its_scroll_bar` in `crates/ui-egui/tests/ui_timeline_zoom.rs`
checks that:

- the bar sits under the time graph;
- dragging its thumb scrolls the graph without changing the zoom, the layer or the current time;
- the bar and Shift+wheel stop at the comp's end, and Shift+wheel scrolls back;
- the zoom slider just below it still zooms;
- showing the whole comp hides it.

`spacebar_drag_scrolls_the_time_graph` in the same file still passes. How to run them: [[testing]].

## Where it is recorded

[[parity]] (Interface).
