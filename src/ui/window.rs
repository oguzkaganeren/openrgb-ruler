use gtk4::glib;
use gtk4::prelude::*;
use relm4::prelude::*;
use std::sync::mpsc;
use std::time::Duration;

use crate::app_state::SharedRules;
use crate::autostart;
use crate::openrgb;
use crate::tray::TrayCmd;
use crate::ui::rule_list::RuleList;
use crate::watcher::WatcherEvent;

pub struct AppInit {
    pub start_in_tray: bool,
    pub rules: SharedRules,
    pub tray_rx: mpsc::Receiver<TrayCmd>,
    pub watcher_rx: mpsc::Receiver<WatcherEvent>,
}

pub struct AppWindow {
    pub rules: SharedRules,
}

#[derive(Debug)]
pub enum AppInput {}

fn load_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(
        ".status-ok  { color: #2ec27e; font-size: 18px; }
         .status-err { color: #e01b24; font-size: 18px; }",
    );
    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn make_status_label(available: bool) -> gtk4::Label {
    let label = gtk4::Label::new(Some("●"));
    if available {
        label.set_css_classes(&["status-ok"]);
        label.set_tooltip_text(Some("OpenRGB: available"));
    } else {
        label.set_css_classes(&["status-err"]);
        label.set_tooltip_text(Some("OpenRGB: not found"));
    }
    label
}

#[relm4::component(pub)]
impl SimpleComponent for AppWindow {
    type Init = AppInit;
    type Input = AppInput;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("OpenRGB Ruler"),
            set_default_width: 800,
            set_default_height: 500,
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        load_css();

        let model = AppWindow { rules: init.rules };
        let widgets = view_output!();

        // --- HeaderBar ---
        let header = gtk4::HeaderBar::new();

        // Right: OpenRGB status dot
        let status_label = make_status_label(openrgb::is_available());
        header.pack_end(&status_label);

        // Right: "Start on login" label + Switch (pack_end goes right-to-left)
        let autostart_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        let login_label = gtk4::Label::new(Some("Start on login"));
        let autostart_switch = gtk4::Switch::new();
        autostart_switch.set_active(autostart::get_autostart());
        autostart_switch.set_valign(gtk4::Align::Center);
        autostart_box.append(&login_label);
        autostart_box.append(&autostart_switch);
        header.pack_end(&autostart_box);

        autostart_switch.connect_state_set(|_, state| {
            let _ = autostart::set_autostart(state);
            glib::Propagation::Proceed
        });

        root.set_titlebar(Some(&header));

        // --- Rule list ---
        let rule_list = RuleList::new(model.rules.clone());
        {
            let rules = model.rules.clone();
            let lb = rule_list.list_box.clone();
            rule_list.add_btn.connect_clicked(move |btn| {
                let parent = btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
                let rules_for_save = rules.clone();
                let rules_for_row = rules.clone();
                let lb2 = lb.clone();
                crate::ui::rule_editor::open_editor(
                    parent.as_ref(),
                    rules_for_save,
                    None,
                    move |rule| {
                        let row = crate::ui::rule_row::build(&rule, rules_for_row.clone(), &lb2);
                        lb2.append(&row);
                    },
                );
            });
        }
        root.set_child(Some(&rule_list.widget));

        // Hide window on close instead of destroying it
        root.connect_close_request(|window| {
            window.set_visible(false);
            glib::Propagation::Stop
        });

        // Poll tray commands every 50ms on GTK main thread
        let window_weak = root.downgrade();
        let tray_rx = init.tray_rx;
        glib::timeout_add_local(Duration::from_millis(50), move || {
            loop {
                match tray_rx.try_recv() {
                    Ok(cmd) => {
                        let Some(window) = window_weak.upgrade() else {
                            return glib::ControlFlow::Break;
                        };
                        match cmd {
                            TrayCmd::Show => window.present(),
                            TrayCmd::Hide => window.set_visible(false),
                            TrayCmd::Quit => std::process::exit(0),
                            TrayCmd::EnableAll | TrayCmd::DisableAll => {}
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => {
                        return glib::ControlFlow::Break;
                    }
                }
            }
            glib::ControlFlow::Continue
        });

        // Forward watcher events to GTK thread (same poll pattern as tray)
        let watcher_rx = init.watcher_rx;
        glib::timeout_add_local(Duration::from_millis(200), move || {
            loop {
                match watcher_rx.try_recv() {
                    Ok(WatcherEvent::RuleFired { rule_name }) => {
                        eprintln!("[ui] rule fired: {rule_name}");
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => {
                        return glib::ControlFlow::Break;
                    }
                }
            }
            glib::ControlFlow::Continue
        });

        // Refresh OpenRGB status every 5 seconds
        let status_weak = status_label.downgrade();
        glib::timeout_add_local(Duration::from_secs(5), move || {
            let Some(label) = status_weak.upgrade() else {
                return glib::ControlFlow::Break;
            };
            if openrgb::is_available() {
                label.set_css_classes(&["status-ok"]);
                label.set_tooltip_text(Some("OpenRGB: available"));
            } else {
                label.set_css_classes(&["status-err"]);
                label.set_tooltip_text(Some("OpenRGB: not found"));
            }
            glib::ControlFlow::Continue
        });

        if init.start_in_tray {
            root.set_visible(false);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: AppInput, _sender: ComponentSender<Self>) {}
}
