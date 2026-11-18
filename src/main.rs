mod mouse;
use mouse::VirtualMouse;
use ashpd::{
    desktop::global_shortcuts::{GlobalShortcuts, NewShortcut},
    register_host_app,
};
use evdev::KeyCode;
use futures_util::StreamExt;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

const APP_ID: &str = "cat.kafka.MyAutoclicker";
const INTERVAL: Duration = Duration::from_millis(500);
const HOLD_MILISECONDS: Duration = Duration::from_millis(10);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Estat compartit entre els dos fils: actiu o no
    let active = Arc::new(AtomicBool::new(false));

    // 2. Fil del clicker
    let mut mouse = VirtualMouse::new("my_autoclicker", &[KeyCode::BTN_LEFT, KeyCode::BTN_RIGHT], HOLD_MILISECONDS)?;

    let active_clicker = Arc::clone(&active);
    thread::spawn(move || {
        loop {
            if active_clicker.load(Ordering::Relaxed) {
                if let Err(e) = mouse.click(KeyCode::BTN_LEFT) {
                    eprintln!("Error fent clic: {e}");
                    break;
                }
                thread::sleep(INTERVAL);
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
    });

    // 3. Ens identifiquem davant del portal
    register_host_app(APP_ID.try_into()?).await?;

    // 4. Demanem la drecera global
    let portal = GlobalShortcuts::new().await?;
    let session = portal.create_session(Default::default()).await?;
    let shortcut = NewShortcut::new("toggle", "Activa / atura l'autoclicker")
        .preferred_trigger("F8");
    let bound = portal
        .bind_shortcuts(&session, &[shortcut], None, Default::default())
        .await?
        .response()?;

    for s in bound.shortcuts() {
        println!("Drecera '{}' assignada a: {}", s.id(), s.trigger_description());
    }

    // 5. Escoltem quan es prem la drecera
    let mut activated = portal.receive_activated().await?;
    println!("Llest! Prem la drecera per activar/aturar. Ctrl+C per sortir.");

    while let Some(event) = activated.next().await {
        if event.shortcut_id() == "toggle" {
            // fetch_xor(true) inverteix el valor i retorna l'anterior
            let was_active = active.fetch_xor(true, Ordering::Relaxed);
            println!("{}", if was_active { "Aturat" } else { "Actiu" });
        }
    }

    Ok(())
}