mod app_state;
mod autostart;
mod config;
mod model;
mod openrgb;
mod tray;
mod ui;
mod watcher;

use std::env;
use std::sync::mpsc;

use app_state::new_shared_rules;
use relm4::RelmApp;
use tray::AppTray;
use ui::window::{AppInit, AppWindow};

fn main() {
    let start_in_tray = env::args().any(|a| a == "--tray");

    openrgb::prefetch();

    let rules = new_shared_rules();
    let (tray_tx, tray_rx) = mpsc::channel();
    let (watcher_tx, watcher_rx) = mpsc::channel::<watcher::WatcherEvent>();

    // ksni runs the tray in its own thread via D-Bus (no tokio needed)
    ksni::TrayService::new(AppTray::new(tray_tx)).spawn();

    // Tokio runtime lives on its own thread; all async watchers run there
    let rules_for_watchers = rules.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");
        rt.block_on(async move {
            watcher::start_watchers(rules_for_watchers, watcher_tx);
            std::future::pending::<()>().await
        });
    });

    let app = RelmApp::new("org.openrgb.ruler");
    app.run::<AppWindow>(AppInit {
        start_in_tray,
        rules,
        tray_rx,
        watcher_rx,
    });
}
