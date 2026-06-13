use ksni::menu::StandardItem;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum TrayCmd {
    Show,
    Hide,
    EnableAll,
    DisableAll,
    Quit,
}

pub struct AppTray {
    sender: Sender<TrayCmd>,
}

impl AppTray {
    pub fn new(sender: Sender<TrayCmd>) -> Self {
        Self { sender }
    }
}

impl ksni::Tray for AppTray {
    fn icon_name(&self) -> String {
        "preferences-color".to_string()
    }

    fn title(&self) -> String {
        "OpenRGB Ruler".to_string()
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        let _ = self.sender.send(TrayCmd::Show);
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        vec![
            StandardItem {
                label: "Show".into(),
                activate: Box::new(|t: &mut Self| {
                    let _ = t.sender.send(TrayCmd::Show);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Hide".into(),
                activate: Box::new(|t: &mut Self| {
                    let _ = t.sender.send(TrayCmd::Hide);
                }),
                ..Default::default()
            }
            .into(),
            ksni::MenuItem::Separator,
            StandardItem {
                label: "Enable All Rules".into(),
                activate: Box::new(|t: &mut Self| {
                    let _ = t.sender.send(TrayCmd::EnableAll);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Disable All Rules".into(),
                activate: Box::new(|t: &mut Self| {
                    let _ = t.sender.send(TrayCmd::DisableAll);
                }),
                ..Default::default()
            }
            .into(),
            ksni::MenuItem::Separator,
            StandardItem {
                label: "Quit".into(),
                activate: Box::new(|t: &mut Self| {
                    let _ = t.sender.send(TrayCmd::Quit);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}
