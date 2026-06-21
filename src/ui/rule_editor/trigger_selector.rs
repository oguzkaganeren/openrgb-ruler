use gtk4::prelude::*;

use crate::model::Trigger;

const TRIGGER_LABELS: &[&str] = &[
    "System Lock",
    "System Unlock",
    "Process Start",
    "Process Stop",
    "Session Idle",
    "Session Active",
    "Suspend",
    "Resume",
    "Time of Day",
];

fn page_for_index(i: u32) -> &'static str {
    match i {
        2 | 3 => "process",
        4 => "idle",
        8 => "time",
        _ => "none",
    }
}

pub struct TriggerSelector {
    pub widget: gtk4::Box,
    dropdown: gtk4::DropDown,
    process_entry: gtk4::Entry,
    idle_entry: gtk4::Entry,
    time_entry: gtk4::Entry,
    day_buttons: Vec<gtk4::CheckButton>,
    stack: gtk4::Stack,
}

impl TriggerSelector {
    pub fn new() -> Self {
        let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 8);

        let dropdown = gtk4::DropDown::from_strings(TRIGGER_LABELS);
        vbox.append(&dropdown);

        let stack = gtk4::Stack::new();

        // "none" page — for triggers with no extra fields
        let none_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        stack.add_named(&none_box, Some("none"));

        // "process" page — process name entry
        let process_entry = gtk4::Entry::new();
        process_entry.set_placeholder_text(Some("e.g. steam"));
        let proc_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        proc_box.append(&gtk4::Label::new(Some("Process name:")));
        proc_box.append(&process_entry);
        stack.add_named(&proc_box, Some("process"));

        // "idle" page — explicit -/+ buttons around a plain entry
        let idle_entry = gtk4::Entry::new();
        idle_entry.set_text("60");
        idle_entry.set_width_chars(6);
        idle_entry.set_max_width_chars(6);
        idle_entry.set_input_purpose(gtk4::InputPurpose::Digits);

        let dec_btn = gtk4::Button::with_label("−");
        let inc_btn = gtk4::Button::with_label("+");

        {
            let e = idle_entry.clone();
            dec_btn.connect_clicked(move |_| {
                let v: u64 = e.text().parse().unwrap_or(60);
                e.set_text(&v.saturating_sub(1).max(1).to_string());
            });
        }
        {
            let e = idle_entry.clone();
            inc_btn.connect_clicked(move |_| {
                let v: u64 = e.text().parse().unwrap_or(60);
                e.set_text(&(v + 1).min(86400).to_string());
            });
        }

        let idle_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        idle_box.append(&gtk4::Label::new(Some("Idle seconds:")));
        idle_box.append(&dec_btn);
        idle_box.append(&idle_entry);
        idle_box.append(&inc_btn);
        stack.add_named(&idle_box, Some("idle"));

        // "time" page — HH:MM entry + day checkboxes
        let time_vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
        let time_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        time_row.append(&gtk4::Label::new(Some("Time (HH:MM):")));
        let time_entry = gtk4::Entry::new();
        time_entry.set_placeholder_text(Some("08:00"));
        time_entry.set_max_width_chars(6);
        time_row.append(&time_entry);
        time_vbox.append(&time_row);

        let days_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
        let day_names = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let mut day_buttons = Vec::new();
        for name in &day_names {
            let btn = gtk4::CheckButton::with_label(name);
            days_box.append(&btn);
            day_buttons.push(btn);
        }
        time_vbox.append(&days_box);
        stack.add_named(&time_vbox, Some("time"));

        vbox.append(&stack);

        {
            let stack_c = stack.clone();
            dropdown.connect_selected_notify(move |dd| {
                stack_c.set_visible_child_name(page_for_index(dd.selected()));
            });
        }

        TriggerSelector { widget: vbox, dropdown, process_entry, idle_entry, time_entry, day_buttons, stack }
    }

    pub fn load(&self, trigger: &Trigger) {
        match trigger {
            Trigger::SystemLock => {
                self.dropdown.set_selected(0);
                self.stack.set_visible_child_name("none");
            }
            Trigger::SystemUnlock => {
                self.dropdown.set_selected(1);
                self.stack.set_visible_child_name("none");
            }
            Trigger::ProcessStart { process_name } => {
                self.dropdown.set_selected(2);
                self.process_entry.set_text(process_name);
                self.stack.set_visible_child_name("process");
            }
            Trigger::ProcessStop { process_name } => {
                self.dropdown.set_selected(3);
                self.process_entry.set_text(process_name);
                self.stack.set_visible_child_name("process");
            }
            Trigger::SessionIdle { seconds } => {
                self.dropdown.set_selected(4);
                self.idle_entry.set_text(&seconds.to_string());
                self.stack.set_visible_child_name("idle");
            }
            Trigger::SessionActive => {
                self.dropdown.set_selected(5);
                self.stack.set_visible_child_name("none");
            }
            Trigger::Suspend => {
                self.dropdown.set_selected(6);
                self.stack.set_visible_child_name("none");
            }
            Trigger::Resume => {
                self.dropdown.set_selected(7);
                self.stack.set_visible_child_name("none");
            }
            Trigger::TimeOfDay { time, days } => {
                self.dropdown.set_selected(8);
                self.time_entry.set_text(time);
                for (i, btn) in self.day_buttons.iter().enumerate() {
                    btn.set_active(days.contains(&(i as u8)));
                }
                self.stack.set_visible_child_name("time");
            }
        }
    }

    pub fn get_trigger(&self) -> Option<Trigger> {
        match self.dropdown.selected() {
            0 => Some(Trigger::SystemLock),
            1 => Some(Trigger::SystemUnlock),
            2 => {
                let name = self.process_entry.text().trim().to_string();
                if name.is_empty() { return None; }
                Some(Trigger::ProcessStart { process_name: name })
            }
            3 => {
                let name = self.process_entry.text().trim().to_string();
                if name.is_empty() { return None; }
                Some(Trigger::ProcessStop { process_name: name })
            }
            4 => {
                let seconds = self.idle_entry.text().parse().unwrap_or(60);
                Some(Trigger::SessionIdle { seconds })
            }
            5 => Some(Trigger::SessionActive),
            6 => Some(Trigger::Suspend),
            7 => Some(Trigger::Resume),
            8 => {
                let time = self.time_entry.text().trim().to_string();
                if time.is_empty() { return None; }
                let days: Vec<u8> = self.day_buttons.iter().enumerate()
                    .filter(|(_, btn)| btn.is_active())
                    .map(|(i, _)| i as u8)
                    .collect();
                Some(Trigger::TimeOfDay { time, days })
            }
            _ => None,
        }
    }
}
