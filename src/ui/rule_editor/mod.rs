mod action_selector;
mod device_selector;
mod trigger_selector;

use std::rc::Rc;

use gtk4::prelude::*;

use crate::{
    app_state::{self, SharedRules},
    model::Rule,
    openrgb,
};

use action_selector::ActionSelector;
use device_selector::DeviceSelector;
use trigger_selector::TriggerSelector;

fn show_error(parent: Option<&gtk4::Window>, msg: &str) {
    let dialog = gtk4::AlertDialog::builder()
        .message("Error")
        .detail(msg)
        .build();
    dialog.show(parent);
}

/// Open the rule editor as a modal window.
/// Pass `existing_rule = Some(rule)` for editing, `None` for adding.
/// `on_saved` is called with the completed Rule when the user clicks Save.
pub fn open_editor(
    parent: Option<&gtk4::Window>,
    rules: SharedRules,
    existing_rule: Option<Rule>,
    on_saved: impl Fn(Rule) + 'static,
) {
    let is_edit = existing_rule.is_some();
    let rule_id = existing_rule
        .as_ref()
        .map(|r| r.id.clone())
        .unwrap_or_else(app_state::generate_id);
    let initial_enabled = existing_rule.as_ref().map(|r| r.enabled).unwrap_or(true);

    let window = gtk4::Window::new();
    window.set_modal(true);
    window.set_default_width(480);
    window.set_resizable(false);
    window.set_title(Some(if is_edit { "Edit Rule" } else { "New Rule" }));
    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let content = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    content.set_margin_top(20);
    content.set_margin_bottom(20);
    content.set_margin_start(20);
    content.set_margin_end(20);

    // --- Name ---
    let name_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let name_lbl = gtk4::Label::new(Some("Name:"));
    name_lbl.set_width_chars(8);
    name_lbl.set_xalign(0.0);
    name_row.append(&name_lbl);
    let name_entry = gtk4::Entry::new();
    name_entry.set_hexpand(true);
    name_entry.set_placeholder_text(Some("My Rule"));
    name_row.append(&name_entry);
    content.append(&name_row);

    // --- Trigger ---
    let trigger_frame = gtk4::Frame::new(Some("Trigger"));
    let trigger_sel = Rc::new(TriggerSelector::new());
    trigger_sel.widget.set_margin_top(8);
    trigger_sel.widget.set_margin_bottom(8);
    trigger_sel.widget.set_margin_start(8);
    trigger_sel.widget.set_margin_end(8);
    trigger_frame.set_child(Some(&trigger_sel.widget));
    content.append(&trigger_frame);

    // --- Action ---
    let action_frame = gtk4::Frame::new(Some("Action"));
    let action_sel = Rc::new(ActionSelector::new());
    action_sel.widget.set_margin_top(8);
    action_sel.widget.set_margin_bottom(8);
    action_sel.widget.set_margin_start(8);
    action_sel.widget.set_margin_end(8);
    action_frame.set_child(Some(&action_sel.widget));
    content.append(&action_frame);

    // --- Devices ---
    let device_frame = gtk4::Frame::new(Some("Devices"));
    let device_sel = Rc::new(DeviceSelector::new());
    device_sel.widget.set_margin_top(8);
    device_sel.widget.set_margin_bottom(8);
    device_sel.widget.set_margin_start(8);
    device_sel.widget.set_margin_end(8);
    device_frame.set_child(Some(&device_sel.widget));
    content.append(&device_frame);

    // --- Load existing rule ---
    if let Some(ref rule) = existing_rule {
        name_entry.set_text(&rule.name);
        trigger_sel.load(&rule.trigger);
        action_sel.load(&rule.action);
        device_sel.load(&rule.device_target);
    }

    // --- Buttons ---
    let btn_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    btn_row.set_halign(gtk4::Align::End);
    let test_btn = gtk4::Button::with_label("Test Action");
    let cancel_btn = gtk4::Button::with_label("Cancel");
    let save_btn = gtk4::Button::with_label(if is_edit { "Update" } else { "Save" });
    save_btn.add_css_class("suggested-action");
    btn_row.append(&test_btn);
    btn_row.append(&cancel_btn);
    btn_row.append(&save_btn);
    content.append(&btn_row);

    window.set_child(Some(&content));

    // Cancel
    {
        let win = window.downgrade();
        cancel_btn.connect_clicked(move |_| {
            if let Some(w) = win.upgrade() { w.close(); }
        });
    }

    // Test Action — execute current action+target without saving
    {
        let action_c = Rc::clone(&action_sel);
        let device_c = Rc::clone(&device_sel);
        let win_weak = window.downgrade();
        test_btn.connect_clicked(move |_| {
            let Some(action) = action_c.get_action() else {
                show_error(win_weak.upgrade().as_ref(), "Invalid action settings.\nCheck that the hex color is a valid 6-character value.");
                return;
            };
            let target = device_c.get_target();
            if let Err(e) = openrgb::execute_action(&action, &target) {
                show_error(win_weak.upgrade().as_ref(), &format!("Test action failed:\n{e}"));
            }
        });
    }

    // Save
    {
        let win = window.downgrade();
        let name_entry_c = name_entry.clone();
        let trigger_c = Rc::clone(&trigger_sel);
        let action_c = Rc::clone(&action_sel);
        let device_c = Rc::clone(&device_sel);
        let rules_c = rules.clone();

        save_btn.connect_clicked(move |_| {
            let name = name_entry_c.text().trim().to_string();
            if name.is_empty() {
                name_entry_c.add_css_class("error");
                return;
            }
            name_entry_c.remove_css_class("error");

            let Some(trigger) = trigger_c.get_trigger() else { return; };
            let Some(action) = action_c.get_action() else { return; };
            let device_target = device_c.get_target();

            let rule = Rule {
                id: rule_id.clone(),
                name,
                enabled: initial_enabled,
                trigger,
                action,
                device_target,
            };

            if is_edit {
                let _ = app_state::update_rule(&rules_c, rule.clone());
            } else {
                let _ = app_state::add_rule(&rules_c, rule.clone());
            }

            on_saved(rule);

            if let Some(w) = win.upgrade() { w.close(); }
        });
    }

    window.present();
}
