use std::cell::RefCell;
use std::rc::Rc;

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

fn trigger_label(t: &Trigger) -> String {
    match t {
        Trigger::SystemLock => "System Lock".to_string(),
        Trigger::SystemUnlock => "System Unlock".to_string(),
        Trigger::ProcessStart { process_name } => format!("Process Start: {process_name}"),
        Trigger::ProcessStop { process_name } => format!("Process Stop: {process_name}"),
        Trigger::SessionIdle { seconds } => format!("Session Idle: {seconds}s"),
        Trigger::SessionActive => "Session Active".to_string(),
        Trigger::Suspend => "Suspend".to_string(),
        Trigger::Resume => "Resume".to_string(),
        Trigger::TimeOfDay { time, days } => {
            if days.is_empty() {
                format!("Time of Day: {time} (every day)")
            } else {
                let names = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
                let s: Vec<&str> = days.iter().map(|&d| names[d as usize]).collect();
                format!("Time of Day: {time} ({})", s.join(", "))
            }
        }
    }
}

fn add_row_to_list(list: &gtk4::ListBox, trigger: &Trigger, triggers: &Rc<RefCell<Vec<Trigger>>>) {
    let row = gtk4::ListBoxRow::new();
    row.set_activatable(false);

    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    hbox.set_margin_top(4);
    hbox.set_margin_bottom(4);
    hbox.set_margin_start(6);
    hbox.set_margin_end(6);

    let lbl = gtk4::Label::new(Some(&trigger_label(trigger)));
    lbl.set_hexpand(true);
    lbl.set_xalign(0.0);

    let remove_btn = gtk4::Button::with_label("✕");
    remove_btn.add_css_class("flat");

    hbox.append(&lbl);
    hbox.append(&remove_btn);
    row.set_child(Some(&hbox));
    list.append(&row);

    let row_ref = row.clone();
    let list_ref = list.clone();
    let triggers_ref = Rc::clone(triggers);
    remove_btn.connect_clicked(move |_| {
        let idx = row_ref.index() as usize;
        triggers_ref.borrow_mut().remove(idx);
        list_ref.remove(&row_ref);
    });
}

pub struct TriggerSelector {
    pub widget: gtk4::Box,
    triggers: Rc<RefCell<Vec<Trigger>>>,
    triggers_list: gtk4::ListBox,
    dropdown: gtk4::DropDown,
    process_entry: gtk4::Entry,
    idle_entry: gtk4::Entry,
    time_entry: gtk4::Entry,
    day_buttons: Vec<gtk4::CheckButton>,
    stack: gtk4::Stack,
}

impl TriggerSelector {
    pub fn new() -> Self {
        let triggers: Rc<RefCell<Vec<Trigger>>> = Rc::new(RefCell::new(Vec::new()));

        let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 8);

        // List of already-added triggers
        let triggers_list = gtk4::ListBox::new();
        triggers_list.set_show_separators(true);
        triggers_list.add_css_class("boxed-list");
        vbox.append(&triggers_list);

        // Dropdown + "Add" button
        let add_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        let dropdown = gtk4::DropDown::from_strings(TRIGGER_LABELS);
        dropdown.set_hexpand(true);
        let add_btn = gtk4::Button::with_label("+ Add");
        add_row.append(&dropdown);
        add_row.append(&add_btn);
        vbox.append(&add_row);

        // Stack for extra params (same pages as before)
        let stack = gtk4::Stack::new();

        let none_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        stack.add_named(&none_box, Some("none"));

        let process_entry = gtk4::Entry::new();
        process_entry.set_placeholder_text(Some("e.g. steam"));
        let proc_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        proc_box.append(&gtk4::Label::new(Some("Process name:")));
        proc_box.append(&process_entry);
        stack.add_named(&proc_box, Some("process"));

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

        // "Add" button: read current dropdown + extra params, push to list
        {
            let dd = dropdown.clone();
            let proc = process_entry.clone();
            let idle = idle_entry.clone();
            let time = time_entry.clone();
            let days = day_buttons.clone();
            let triggers_c = Rc::clone(&triggers);
            let list_c = triggers_list.clone();

            add_btn.connect_clicked(move |_| {
                let trigger = match dd.selected() {
                    0 => Some(Trigger::SystemLock),
                    1 => Some(Trigger::SystemUnlock),
                    2 => {
                        let name = proc.text().trim().to_string();
                        if name.is_empty() { return; }
                        Some(Trigger::ProcessStart { process_name: name })
                    }
                    3 => {
                        let name = proc.text().trim().to_string();
                        if name.is_empty() { return; }
                        Some(Trigger::ProcessStop { process_name: name })
                    }
                    4 => Some(Trigger::SessionIdle { seconds: idle.text().parse().unwrap_or(60) }),
                    5 => Some(Trigger::SessionActive),
                    6 => Some(Trigger::Suspend),
                    7 => Some(Trigger::Resume),
                    8 => {
                        let t = time.text().trim().to_string();
                        if t.is_empty() { return; }
                        let selected_days: Vec<u8> = days.iter().enumerate()
                            .filter(|(_, btn)| btn.is_active())
                            .map(|(i, _)| i as u8)
                            .collect();
                        Some(Trigger::TimeOfDay { time: t, days: selected_days })
                    }
                    _ => None,
                };

                if let Some(t) = trigger {
                    triggers_c.borrow_mut().push(t.clone());
                    add_row_to_list(&list_c, &t, &triggers_c);
                }
            });
        }

        TriggerSelector { widget: vbox, triggers, triggers_list, dropdown, process_entry, idle_entry, time_entry, day_buttons, stack }
    }

    pub fn load(&self, triggers: &[Trigger]) {
        while let Some(row) = self.triggers_list.row_at_index(0) {
            self.triggers_list.remove(&row);
        }
        self.triggers.borrow_mut().clear();

        for t in triggers {
            self.triggers.borrow_mut().push(t.clone());
            add_row_to_list(&self.triggers_list, t, &self.triggers);
        }
    }

    pub fn get_triggers(&self) -> Option<Vec<Trigger>> {
        let v = self.triggers.borrow().clone();
        if v.is_empty() { None } else { Some(v) }
    }
}
