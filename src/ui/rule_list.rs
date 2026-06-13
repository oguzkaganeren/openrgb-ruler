use gtk4::prelude::*;

use crate::app_state::{self, SharedRules};
use crate::model::Rule;
use crate::ui::rule_row;

pub struct RuleList {
    pub widget: gtk4::Box,
    pub list_box: gtk4::ListBox,
    pub add_btn: gtk4::Button,
    rules: SharedRules,
}

impl RuleList {
    pub fn new(rules: SharedRules) -> Self {
        let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        outer.set_hexpand(true);
        outer.set_vexpand(true);

        let scrolled = gtk4::ScrolledWindow::new();
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        scrolled.set_margin_top(12);
        scrolled.set_margin_bottom(0);
        scrolled.set_margin_start(12);
        scrolled.set_margin_end(12);

        let list_box = gtk4::ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::None);
        list_box.add_css_class("boxed-list");

        // Built-in GTK placeholder shown when list is empty
        let placeholder = gtk4::Label::builder()
            .label("No rules yet — click + to add one")
            .margin_top(32)
            .margin_bottom(32)
            .build();
        placeholder.add_css_class("dim-label");
        list_box.set_placeholder(Some(&placeholder));

        scrolled.set_child(Some(&list_box));
        outer.append(&scrolled);

        // Bottom bar with "+" button
        let bottom = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        bottom.set_halign(gtk4::Align::Center);
        bottom.set_margin_top(8);
        bottom.set_margin_bottom(12);

        let add_btn = gtk4::Button::with_label("+");
        add_btn.set_tooltip_text(Some("Add new rule"));
        add_btn.add_css_class("suggested-action");
        add_btn.add_css_class("circular");

        bottom.append(&add_btn);
        outer.append(&bottom);

        // Populate with saved rules
        let initial_rules = app_state::get_rules(&rules);
        for rule in &initial_rules {
            let row = rule_row::build(rule, rules.clone(), &list_box);
            list_box.append(&row);
        }

        RuleList {
            widget: outer,
            list_box,
            add_btn,
            rules,
        }
    }

    /// Append a newly created rule to the list.
    pub fn append_rule(&self, rule: &Rule) {
        let row = rule_row::build(rule, self.rules.clone(), &self.list_box);
        self.list_box.append(&row);
    }
}
