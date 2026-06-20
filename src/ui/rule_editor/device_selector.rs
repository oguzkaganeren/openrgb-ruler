use gtk4::prelude::*;

use crate::{
    model::{DeviceTarget, RgbDevice},
    openrgb,
};

pub struct DeviceSelector {
    pub widget: gtk4::Box,
    all_radio: gtk4::CheckButton,
    specific_radio: gtk4::CheckButton,
    devices: Vec<RgbDevice>,
    device_checks: Vec<gtk4::CheckButton>,
    specific_box: gtk4::Box,
}

impl DeviceSelector {
    pub fn new() -> Self {
        let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 8);

        let all_radio = gtk4::CheckButton::with_label("All Devices");
        all_radio.set_active(true);
        let specific_radio = gtk4::CheckButton::with_label("Specific Devices");
        specific_radio.set_group(Some(&all_radio));

        let radio_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
        radio_row.append(&all_radio);
        radio_row.append(&specific_radio);
        vbox.append(&radio_row);

        // Device checklist (hidden until "Specific" is selected)
        let specific_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        specific_box.set_visible(false);
        specific_box.set_margin_start(8);

        let devices = openrgb::list_devices_cached();
        let mut device_checks = Vec::new();

        if devices.is_empty() {
            let lbl = gtk4::Label::new(Some("No devices found (is OpenRGB running?)"));
            lbl.add_css_class("dim-label");
            lbl.set_halign(gtk4::Align::Start);
            specific_box.append(&lbl);
        } else {
            for dev in &devices {
                let check = gtk4::CheckButton::with_label(&format!("{}: {}", dev.id, dev.name));
                specific_box.append(&check);
                device_checks.push(check);
            }
        }

        vbox.append(&specific_box);

        {
            let specific_box_c = specific_box.clone();
            specific_radio.connect_toggled(move |btn| {
                specific_box_c.set_visible(btn.is_active());
            });
        }

        DeviceSelector { widget: vbox, all_radio, specific_radio, devices, device_checks, specific_box }
    }

    pub fn load(&self, target: &DeviceTarget) {
        match target {
            DeviceTarget::All => {
                self.all_radio.set_active(true);
                self.specific_box.set_visible(false);
            }
            DeviceTarget::Specific { ids } => {
                self.specific_radio.set_active(true);
                self.specific_box.set_visible(true);
                for (i, check) in self.device_checks.iter().enumerate() {
                    if let Some(dev) = self.devices.get(i) {
                        check.set_active(ids.contains(&dev.id));
                    }
                }
            }
        }
    }

    pub fn get_target(&self) -> DeviceTarget {
        if self.all_radio.is_active() {
            return DeviceTarget::All;
        }
        let ids: Vec<u32> = self.device_checks.iter().enumerate()
            .filter(|(_, btn)| btn.is_active())
            .filter_map(|(i, _)| self.devices.get(i).map(|d| d.id))
            .collect();
        DeviceTarget::Specific { ids }
    }
}
