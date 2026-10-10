//! Double-clicking a layer opens it, as in After Effects (egui_kittest): in the Timeline, on its
//! name or its bar, a precomp opens its composition (Alt: the Layer panel), a camera or light
//! opens its settings and a text layer is edited with all its text selected; in the Composition
//! panel, a precomp opens its composition and footage or a solid opens in the Layer panel.

use effectcraft_engine::Session;
use effectcraft_engine::project::ItemId;
use effectcraft_ui_egui::dock::PanelKind;
use effectcraft_ui_egui::{Dialog, EffectcraftApp};
use egui::{Event, Modifiers, Pos2, pos2};
use egui_kittest::Harness;
use serde_json::json;

struct Ids {
    main: u64,
    other: u64,
    pre: u64,
    cam: u64,
    key: u64,
    title: u64,
    ctrl: u64,
    red: u64,
}

/// Comp "Main" (open) holding a red solid, a precomp of "Other", a camera, a light, a text layer
/// and a null.
fn harness() -> (Harness<'static, EffectcraftApp>, Ids) {
    let mut s = Session::default();
    let other = s.execute("comp.new", json!({"name": "Other", "width": 320, "height": 180, "duration": 4})).unwrap()["comp"].as_u64().unwrap();
    let main = s.execute("comp.new", json!({"name": "Main", "width": 320, "height": 180, "duration": 4})).unwrap()["comp"].as_u64().unwrap();
    let layer = |s: &mut Session, cmd: &str, p: serde_json::Value| s.execute(cmd, p).unwrap()["layer"].as_u64().unwrap();
    let red = layer(&mut s, "layer.newSolid", json!({"name": "Red", "color": [1.0, 0.0, 0.0, 1.0]}));
    let pre = layer(&mut s, "layer.addItem", json!({"item": other}));
    let cam = layer(&mut s, "layer.newCamera", json!({"name": "Cam"}));
    let key = layer(&mut s, "layer.newLight", json!({"name": "Key", "kind": "Point"}));
    let title = layer(&mut s, "layer.newText", json!({"text": "Hello", "name": "Title"}));
    let ctrl = layer(&mut s, "layer.newNull", json!({"name": "Ctrl"}));
    s.execute("edit.deselectAll", json!({})).unwrap();
    s.state.open_comps = vec![ItemId(main)];
    s.state.active_comp = Some(ItemId(main));
    let mut h = Harness::builder().with_size(egui::vec2(1600.0, 1000.0)).build_eframe(|_| EffectcraftApp::new(s));
    h.run_steps(3);
    (h, Ids { main, other, pre, cam, key, title, ctrl, red })
}

fn rect(h: &Harness<'_, EffectcraftApp>, id: &str) -> egui::Rect {
    let e = h.state().auto.previous.iter().chain(h.state().auto.elements.iter()).find(|e| e.id == id).cloned().unwrap_or_else(|| {
        let have: Vec<&String> = h.state().auto.elements.iter().map(|e| &e.id).collect();
        panic!("no element {id}; have {have:?}")
    });
    egui::Rect::from_min_size(pos2(e.rect[0], e.rect[1]), egui::vec2(e.rect[2], e.rect[3]))
}

fn double_click(h: &mut Harness<'_, EffectcraftApp>, p: Pos2, modifiers: Modifiers) {
    h.input_mut().events.push(Event::PointerMoved(p));
    h.input_mut().events.push(Event::ModifiersChanged(modifiers));
    h.step();
    for _ in 0..2 {
        h.input_mut().events.push(Event::PointerButton { pos: p, button: egui::PointerButton::Primary, pressed: true, modifiers });
        h.input_mut().events.push(Event::PointerButton { pos: p, button: egui::PointerButton::Primary, pressed: false, modifiers });
    }
    h.step();
    h.input_mut().events.push(Event::ModifiersChanged(Modifiers::NONE));
    h.run_steps(3);
}

fn invoke(h: &mut Harness<'_, EffectcraftApp>, id: &str, params: serde_json::Value) {
    let ctx = h.ctx.clone();
    effectcraft_ui_egui::menus::invoke(h.state_mut(), &ctx, id, params).unwrap();
    h.run_steps(3);
}

fn active(h: &Harness<'_, EffectcraftApp>) -> Option<u64> {
    h.state().session.active_comp_id().map(|c| c.0)
}

fn bar(h: &Harness<'_, EffectcraftApp>, layer: u64) -> Pos2 {
    rect(h, &format!("timeline.layer.{layer}.bar")).center()
}

/// A point on the layer's name in the Timeline (just right of its twirl and icon).
fn name(h: &Harness<'_, EffectcraftApp>, layer: u64) -> Pos2 {
    let tw = rect(h, &format!("timeline.layer.{layer}.twirl"));
    pos2(tw.max.x + 40.0, tw.center().y)
}

fn back_to_main(h: &mut Harness<'_, EffectcraftApp>, ids: &Ids) {
    h.state_mut().dialog = None;
    invoke(h, "comp.open", json!({"comp": ids.main}));
    invoke(h, "edit.deselectAll", json!({}));
}

/// Double-clicking a precomp's bar (not only its name) in the Timeline opens its composition, as
/// in After Effects; Alt+double-click opens it in the Layer panel instead.
#[test]
fn double_clicking_a_precomp_bar_or_name_opens_it() {
    let (mut h, ids) = harness();
    let p = bar(&h, ids.pre);
    double_click(&mut h, p, Modifiers::NONE);
    assert_eq!(active(&h), Some(ids.other), "the bar opened the precomp");

    back_to_main(&mut h, &ids);
    let p = name(&h, ids.pre);
    double_click(&mut h, p, Modifiers::NONE);
    assert_eq!(active(&h), Some(ids.other), "the name opened the precomp");

    back_to_main(&mut h, &ids);
    let p = bar(&h, ids.pre);
    double_click(&mut h, p, Modifiers::ALT);
    assert_eq!(active(&h), Some(ids.main), "Alt keeps the comp");
    assert_eq!(h.state().ui.layer_panel, Some(ids.pre), "Alt opened the Layer panel");
    assert!(h.state().ui.dock.is_visible(PanelKind::Layer));
}

/// Cameras and lights open their settings, text layers are edited with all their text selected,
/// and other layers (a null) are still renamed from their name; their bar does nothing.
#[test]
fn double_clicking_cameras_lights_and_text_in_the_timeline() {
    let (mut h, ids) = harness();
    let p = bar(&h, ids.cam);
    double_click(&mut h, p, Modifiers::NONE);
    assert_eq!(h.state().dialog, Some(Dialog::CameraSettings));

    back_to_main(&mut h, &ids);
    let p = name(&h, ids.key);
    double_click(&mut h, p, Modifiers::NONE);
    assert_eq!(h.state().dialog, Some(Dialog::LightSettings));

    back_to_main(&mut h, &ids);
    let p = bar(&h, ids.title);
    double_click(&mut h, p, Modifiers::NONE);
    let e = h.state().session.state.text_edit.clone().expect("text editing started");
    assert_eq!((e.layer.0, e.range().start, e.range().end), (ids.title, 0, 5), "all of \"Hello\" is selected");
    invoke(&mut h, "text.endEdit", json!({}));

    back_to_main(&mut h, &ids);
    let p = bar(&h, ids.ctrl);
    double_click(&mut h, p, Modifiers::NONE);
    assert!(h.state().auto.find(&format!("timeline.layer.{}.nameEdit", ids.ctrl)).is_none(), "the bar does not rename");
    assert_eq!((active(&h), h.state().dialog), (Some(ids.main), None));
    let p = name(&h, ids.ctrl);
    double_click(&mut h, p, Modifiers::NONE);
    assert!(h.state().auto.find(&format!("timeline.layer.{}.nameEdit", ids.ctrl)).is_some(), "the name is renamed");
}

/// In the Composition panel a precomp opens its composition (Alt: the Layer panel) and a solid
/// opens in the Layer panel.
#[test]
fn double_clicking_layers_in_the_composition_panel() {
    let (mut h, ids) = harness();
    let centre = |h: &Harness<'_, EffectcraftApp>| effectcraft_ui_egui::panels::viewer::comp_to_screen(&h.ctx, [160.0, 90.0]).unwrap();
    // The topmost layer under the pointer is the one opened: bring the precomp above the text
    // and the null. The title underneath used to be edited instead of the precomp opening.
    invoke(&mut h, "layer.arrange", json!({"layers": [ids.pre], "to": "front"}));
    invoke(&mut h, "edit.deselectAll", json!({}));
    let p = centre(&h);
    double_click(&mut h, p, Modifiers::NONE);
    assert_eq!(active(&h), Some(ids.other));
    assert!(h.state().session.state.text_edit.is_none(), "the title underneath is not edited");

    back_to_main(&mut h, &ids);
    let p = centre(&h);
    double_click(&mut h, p, Modifiers::ALT);
    assert_eq!(active(&h), Some(ids.main));
    assert_eq!(h.state().ui.layer_panel, Some(ids.pre));

    back_to_main(&mut h, &ids);
    invoke(&mut h, "layer.arrange", json!({"layers": [ids.red], "to": "front"}));
    invoke(&mut h, "edit.deselectAll", json!({}));
    h.state_mut().ui.layer_panel = None;
    let p = centre(&h);
    double_click(&mut h, p, Modifiers::NONE);
    assert_eq!(active(&h), Some(ids.main));
    assert_eq!(h.state().ui.layer_panel, Some(ids.red), "the solid opened in the Layer panel");
}
