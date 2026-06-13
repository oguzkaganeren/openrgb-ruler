use std::collections::HashSet;
use std::sync::{mpsc, Arc, RwLock};
use futures_util::StreamExt;
use tokio::time::{interval, Duration};
use chrono::{Timelike, Datelike};

use crate::model::{Rule, Trigger};
use crate::openrgb;

#[derive(Debug, Clone)]
pub enum WatcherEvent {
    RuleFired { rule_name: String },
}

// ── DBus proxies ──────────────────────────────────────────────────────────────

#[zbus::proxy(
    interface = "org.freedesktop.ScreenSaver",
    default_service = "org.freedesktop.ScreenSaver",
    default_path = "/org/freedesktop/ScreenSaver"
)]
trait ScreenSaver {
    #[zbus(signal)]
    fn active_changed(&self, active: bool) -> zbus::Result<()>;
}

#[zbus::proxy(
    interface = "org.freedesktop.login1.Session",
    default_service = "org.freedesktop.login1"
)]
trait Login1Session {
    #[zbus(signal)]
    fn lock(&self) -> zbus::Result<()>;

    #[zbus(signal)]
    fn unlock(&self) -> zbus::Result<()>;
}

#[zbus::proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait Login1Manager {
    #[zbus(signal)]
    fn prepare_for_sleep(&self, start: bool) -> zbus::Result<()>;
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Spawn all background watcher tasks. Called once at app startup.
pub fn start_watchers(rules: Arc<RwLock<Vec<Rule>>>, event_tx: mpsc::Sender<WatcherEvent>) {
    let r1 = rules.clone(); let tx1 = event_tx.clone();
    let r2 = rules.clone(); let tx2 = event_tx.clone();
    let r3 = rules.clone(); let tx3 = event_tx.clone();
    let r4 = rules.clone(); let tx4 = event_tx.clone();
    let r5 = rules;         let tx5 = event_tx;

    tokio::spawn(async move {
        if let Err(e) = screensaver_watcher(r1, tx1).await {
            eprintln!("[watcher] ScreenSaver DBus error: {e}");
        }
    });

    tokio::spawn(async move {
        if let Err(e) = login1_watcher(r2, tx2).await {
            eprintln!("[watcher] login1 DBus error: {e}");
        }
    });

    tokio::spawn(async move {
        if let Err(e) = sleep_watcher(r3, tx3).await {
            eprintln!("[watcher] login1 sleep DBus error: {e}");
        }
    });

    tokio::spawn(process_watcher(r4, tx4));
    tokio::spawn(time_watcher(r5, tx5));
}

// ── Rule evaluator ────────────────────────────────────────────────────────────

fn evaluate_trigger(
    event: &Trigger,
    rules: &RwLock<Vec<Rule>>,
    event_tx: &mpsc::Sender<WatcherEvent>,
) {
    let matched = {
        let Ok(guard) = rules.read() else { return };
        guard
            .iter()
            .find(|r| r.enabled && triggers_match(&r.trigger, event))
            .map(|r| (r.action.clone(), r.device_target.clone(), r.name.clone()))
    }; // lock released before the (potentially slow) openrgb call

    if let Some((action, target, name)) = matched {
        if let Err(e) = openrgb::execute_action(&action, &target) {
            eprintln!("[watcher] openrgb action failed: {e}");
        }
        let _ = event_tx.send(WatcherEvent::RuleFired { rule_name: name });
    }
}

fn triggers_match(rule_trigger: &Trigger, event: &Trigger) -> bool {
    match (rule_trigger, event) {
        (Trigger::SystemLock, Trigger::SystemLock) => true,
        (Trigger::SystemUnlock, Trigger::SystemUnlock) => true,
        (Trigger::SessionActive, Trigger::SessionActive) => true,
        (Trigger::Suspend, Trigger::Suspend) => true,
        (Trigger::Resume, Trigger::Resume) => true,
        (
            Trigger::ProcessStart { process_name: a },
            Trigger::ProcessStart { process_name: b },
        ) => a == b,
        (
            Trigger::ProcessStop { process_name: a },
            Trigger::ProcessStop { process_name: b },
        ) => a == b,
        (
            Trigger::TimeOfDay { time: rule_time, days: rule_days },
            Trigger::TimeOfDay { time: event_time, days: event_days },
        ) => {
            rule_time == event_time
                && (rule_days.is_empty() || event_days.iter().any(|d| rule_days.contains(d)))
        }
        _ => false,
    }
}

// ── DBus: ScreenSaver (primary lock/unlock source) ────────────────────────────

async fn screensaver_watcher(
    rules: Arc<RwLock<Vec<Rule>>>,
    event_tx: mpsc::Sender<WatcherEvent>,
) -> zbus::Result<()> {
    let connection = zbus::Connection::session().await?;
    let proxy = ScreenSaverProxy::new(&connection).await?;
    let mut stream = proxy.receive_active_changed().await?;

    while let Some(signal) = stream.next().await {
        let args = signal.args()?;
        let trigger = if *args.active() {
            Trigger::SystemLock
        } else {
            Trigger::SessionActive
        };
        evaluate_trigger(&trigger, &rules, &event_tx);
    }

    Ok(())
}

// ── DBus: login1 Session (fallback for systemd-logind) ────────────────────────

async fn login1_watcher(
    rules: Arc<RwLock<Vec<Rule>>>,
    event_tx: mpsc::Sender<WatcherEvent>,
) -> zbus::Result<()> {
    let connection = zbus::Connection::system().await?;

    let manager: zbus::Proxy<'_> = zbus::proxy::Builder::new(&connection)
        .destination("org.freedesktop.login1")?
        .path("/org/freedesktop/login1")?
        .interface("org.freedesktop.login1.Manager")?
        .build()
        .await?;

    let pid = std::process::id();
    let session_path: zbus::zvariant::OwnedObjectPath =
        manager.call("GetSessionByPID", &(pid,)).await?;

    let session: Login1SessionProxy<'_> = Login1SessionProxy::builder(&connection)
        .path(session_path)?
        .build()
        .await?;

    let mut lock_stream = session.receive_lock().await?;
    let mut unlock_stream = session.receive_unlock().await?;

    loop {
        tokio::select! {
            Some(_) = lock_stream.next() => {
                evaluate_trigger(&Trigger::SystemLock, &rules, &event_tx);
            }
            Some(_) = unlock_stream.next() => {
                evaluate_trigger(&Trigger::SystemUnlock, &rules, &event_tx);
            }
            else => break,
        }
    }

    Ok(())
}

// ── DBus: login1 Manager (suspend / resume) ───────────────────────────────────

async fn sleep_watcher(
    rules: Arc<RwLock<Vec<Rule>>>,
    event_tx: mpsc::Sender<WatcherEvent>,
) -> zbus::Result<()> {
    let connection = zbus::Connection::system().await?;
    let proxy = Login1ManagerProxy::new(&connection).await?;
    let mut stream = proxy.receive_prepare_for_sleep().await?;

    while let Some(signal) = stream.next().await {
        let args = signal.args()?;
        let trigger = if *args.start() {
            Trigger::Suspend
        } else {
            Trigger::Resume
        };
        evaluate_trigger(&trigger, &rules, &event_tx);
    }

    Ok(())
}

// ── Process watcher (poll /proc every 2 s) ────────────────────────────────────

async fn process_watcher(rules: Arc<RwLock<Vec<Rule>>>, event_tx: mpsc::Sender<WatcherEvent>) {
    let mut prev = get_running_processes();
    let mut ticker = interval(Duration::from_secs(2));
    ticker.tick().await;

    loop {
        ticker.tick().await;
        let current = get_running_processes();

        let watched: HashSet<String> = {
            let Ok(guard) = rules.read() else { continue };
            guard
                .iter()
                .filter(|r| r.enabled)
                .filter_map(|r| match &r.trigger {
                    Trigger::ProcessStart { process_name }
                    | Trigger::ProcessStop { process_name } => Some(process_name.clone()),
                    _ => None,
                })
                .collect()
        };

        for name in current.difference(&prev) {
            if watched.contains(name) {
                evaluate_trigger(
                    &Trigger::ProcessStart { process_name: name.clone() },
                    &rules,
                    &event_tx,
                );
            }
        }

        for name in prev.difference(&current) {
            if watched.contains(name) {
                evaluate_trigger(
                    &Trigger::ProcessStop { process_name: name.clone() },
                    &rules,
                    &event_tx,
                );
            }
        }

        prev = current;
    }
}

// ── Time-of-day watcher (poll every 10 s, fire once per minute) ───────────────

async fn time_watcher(rules: Arc<RwLock<Vec<Rule>>>, event_tx: mpsc::Sender<WatcherEvent>) {
    let mut last_fired = String::new();
    let mut ticker = interval(Duration::from_secs(10));
    ticker.tick().await;

    loop {
        ticker.tick().await;

        let has_time_rule = {
            let Ok(guard) = rules.read() else { continue };
            guard.iter().any(|r| r.enabled && matches!(&r.trigger, Trigger::TimeOfDay { .. }))
        };
        if !has_time_rule {
            continue;
        }

        let now = chrono::Local::now();
        let time_str = format!("{:02}:{:02}", now.hour(), now.minute());
        if time_str == last_fired {
            continue;
        }
        last_fired = time_str.clone();

        let weekday = now.weekday().num_days_from_sunday() as u8;
        evaluate_trigger(
            &Trigger::TimeOfDay { time: time_str, days: vec![weekday] },
            &rules,
            &event_tx,
        );
    }
}

fn get_running_processes() -> HashSet<String> {
    let mut names = HashSet::new();
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return names;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !dir_name.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        let Ok(data) = std::fs::read(path.join("cmdline")) else {
            continue;
        };
        let exe = data
            .split(|&b| b == 0)
            .next()
            .and_then(|b| std::str::from_utf8(b).ok())
            .unwrap_or("")
            .to_string();

        let basename = std::path::Path::new(&exe)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        if !basename.is_empty() {
            names.insert(basename);
        }
    }

    names
}
