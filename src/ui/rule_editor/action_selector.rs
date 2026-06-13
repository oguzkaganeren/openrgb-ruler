use gtk4::prelude::*;

use crate::{model::RgbAction, openrgb};

const ACTION_LABELS: &[&str] = &["Turn Off", "Set Color", "Load Profile"];

fn page_for_index(i: u32) -> &'static str {
    match i {
        1 => "color",
        2 => "profile",
        _ => "turnoff",
    }
}

pub struct ActionSelector {
    pub widget: gtk4::Box,
    dropdown: gtk4::DropDown,
    color_entry: gtk4::Entry,
    brightness_spin: gtk4::SpinButton,
    profile_dropdown: gtk4::DropDown,
    profiles: Vec<String>,
    stack: gtk4::Stack,
}

impl ActionSelector {
    pub fn new() -> Self {
        let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 8);

        let dropdown = gtk4::DropDown::from_strings(ACTION_LABELS);
        vbox.append(&dropdown);

        let stack = gtk4::Stack::new();

        // "turnoff" page
        let turnoff_lbl = gtk4::Label::new(Some("Will turn off all LEDs."));
        turnoff_lbl.add_css_class("dim-label");
        turnoff_lbl.set_halign(gtk4::Align::Start);
        stack.add_named(&turnoff_lbl, Some("turnoff"));

        // "color" page
        let color_vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 6);

        let color_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        color_row.append(&gtk4::Label::new(Some("Hex color:")));
        let color_entry = gtk4::Entry::new();
        color_entry.set_placeholder_text(Some("FF0000"));
        color_entry.set_max_width_chars(8);
        color_row.append(&color_entry);
        color_vbox.append(&color_row);

        let brightness_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        brightness_row.append(&gtk4::Label::new(Some("Brightness %:")));
        let adj = gtk4::Adjustment::new(100.0, 1.0, 100.0, 1.0, 10.0, 0.0);
        let brightness_spin = gtk4::SpinButton::new(Some(&adj), 1.0, 0);
        brightness_spin.set_numeric(true);
        brightness_row.append(&brightness_spin);
        color_vbox.append(&brightness_row);

        stack.add_named(&color_vbox, Some("color"));

        // "profile" page
        let profiles = openrgb::list_profiles().unwrap_or_default();
        let profile_strs: Vec<&str> = profiles.iter().map(|s| s.as_str()).collect();
        let profile_dropdown = if profile_strs.is_empty() {
            gtk4::DropDown::from_strings(&["(no profiles found)"])
        } else {
            gtk4::DropDown::from_strings(&profile_strs)
        };

        let profile_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        profile_box.append(&gtk4::Label::new(Some("Profile:")));
        profile_box.append(&profile_dropdown);
        stack.add_named(&profile_box, Some("profile"));

        vbox.append(&stack);

        {
            let stack_c = stack.clone();
            dropdown.connect_selected_notify(move |dd| {
                stack_c.set_visible_child_name(page_for_index(dd.selected()));
            });
        }

        ActionSelector { widget: vbox, dropdown, color_entry, brightness_spin, profile_dropdown, profiles, stack }
    }

    pub fn load(&self, action: &RgbAction) {
        match action {
            RgbAction::TurnOff => {
                self.dropdown.set_selected(0);
                self.stack.set_visible_child_name("turnoff");
            }
            RgbAction::SetColor { hex, percent } => {
                self.dropdown.set_selected(1);
                self.color_entry.set_text(hex);
                self.brightness_spin.set_value(*percent as f64);
                self.stack.set_visible_child_name("color");
            }
            RgbAction::LoadProfile { name } => {
                self.dropdown.set_selected(2);
                if let Some(idx) = self.profiles.iter().position(|p| p == name) {
                    self.profile_dropdown.set_selected(idx as u32);
                }
                self.stack.set_visible_child_name("profile");
            }
        }
    }

    pub fn get_action(&self) -> Option<RgbAction> {
        match self.dropdown.selected() {
            0 => Some(RgbAction::TurnOff),
            1 => {
                let raw = self.color_entry.text();
                let hex = raw.trim().trim_start_matches('#').to_uppercase();
                if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                    return None;
                }
                let percent = self.brightness_spin.value() as u8;
                Some(RgbAction::SetColor { hex, percent })
            }
            2 => {
                let idx = self.profile_dropdown.selected() as usize;
                let name = self.profiles.get(idx)?.clone();
                Some(RgbAction::LoadProfile { name })
            }
            _ => None,
        }
    }
}
