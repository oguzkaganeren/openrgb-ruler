use gtk4::prelude::*;

use crate::app_state::{self, SharedRules};
use crate::model::Rule;

pub fn build(rule: &Rule, rules: SharedRules, list_box: &gtk4::ListBox) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.set_activatable(false);

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    hbox.set_margin_top(8);
    hbox.set_margin_bottom(8);
    hbox.set_margin_start(12);
    hbox.set_margin_end(12);

    let switch = gtk4::Switch::new();
    switch.set_active(rule.enabled);
    switch.set_valign(gtk4::Align::Center);

    let label = gtk4::Label::new(Some(&rule.name));
    label.set_hexpand(true);
    label.set_xalign(0.0);
    label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

    let edit_btn = gtk4::Button::with_label("Edit");
    edit_btn.set_valign(gtk4::Align::Center);

    let delete_btn = gtk4::Button::with_label("Delete");
    delete_btn.set_valign(gtk4::Align::Center);
    delete_btn.add_css_class("destructive-action");

    hbox.append(&switch);
    hbox.append(&label);
    hbox.append(&edit_btn);
    hbox.append(&delete_btn);
    row.set_child(Some(&hbox));

    // Toggle enable/disable
    {
        let rules = rules.clone();
        let id = rule.id.clone();
        switch.connect_state_set(move |_, _state| {
            let _ = app_state::toggle_rule(&rules, &id);
            gtk4::glib::Propagation::Proceed
        });
    }

    // Delete rule and remove row from list
    {
        let rules = rules.clone();
        let id = rule.id.clone();
        let row_weak = row.downgrade();
        let lb_weak = list_box.downgrade();
        delete_btn.connect_clicked(move |_| {
            let _ = app_state::delete_rule(&rules, &id);
            if let (Some(r), Some(lb)) = (row_weak.upgrade(), lb_weak.upgrade()) {
                lb.remove(&r);
            }
        });
    }

    // Edit button
    {
        let id = rule.id.clone();
        let rules_edit = rules.clone();
        let label_weak = label.downgrade();
        let rule_snapshot = rule.clone();
        edit_btn.connect_clicked(move |btn| {
            let parent = btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            let current_rule = {
                let rs = rules_edit.read().unwrap();
                rs.iter().find(|r| r.id == id).cloned()
            }
            .unwrap_or_else(|| rule_snapshot.clone());
            let lbl_weak = label_weak.clone();
            crate::ui::rule_editor::open_editor(
                parent.as_ref(),
                rules_edit.clone(),
                Some(current_rule),
                move |updated| {
                    if let Some(lbl) = lbl_weak.upgrade() {
                        lbl.set_text(&updated.name);
                    }
                },
            );
        });
    }

    row
}
