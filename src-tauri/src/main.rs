#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use kryoto_repair::{
    crack::EmuIdentity, detection::GameIdentity, journal, release, repair, settings::Emulator,
};
use serde::Serialize;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};
use tauri::{Emitter, Manager, State};

#[derive(Default)]
struct Fixer {
    running: AtomicBool,
    logs: Mutex<Vec<String>>,
    game: Mutex<Option<GameIdentity>>,
    source: Mutex<String>,
}
struct Reset<'a>(&'a AtomicBool);
impl Drop for Reset<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

fn game_root() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .map(std::path::Path::to_owned)
        .ok_or("Place the fixer inside a game folder.".into())
}
#[derive(Clone, Serialize)]
struct Progress {
    text: String,
    percent: Option<u8>,
}
fn progress(app: &tauri::AppHandle, state: &Fixer, text: &str, percent: Option<u8>) {
    if let Ok(mut log) = state.logs.lock() {
        repair::add_log(&mut log, text, percent);
    }
    let _ = app.emit(
        "repair-progress",
        Progress {
            text: text.into(),
            percent,
        },
    );
}

#[tauri::command]
fn fixer_sources() -> Vec<release::Source> {
    release::sources()
}
#[tauri::command]
fn fixer_state() -> Result<serde_json::Value, String> {
    let record = journal::load(&game_root()?)?;
    Ok(
        serde_json::json!({ "undo_available": record.as_ref().is_some_and(|j| j.state != "undone"), "interrupted": record.as_ref().is_some_and(|j| j.state == "pending"), "report": record.as_ref().filter(|j| j.state == "applied").and_then(|j| j.report.as_ref()) }),
    )
}
#[tauri::command]
async fn fixer_apply(
    app: tauri::AppHandle,
    state: State<'_, Fixer>,
    source: Emulator,
) -> Result<repair::RepairReport, String> {
    if state.running.swap(true, Ordering::SeqCst) {
        return Err("A repair is already running.".into());
    }
    let _reset = Reset(&state.running);
    let root = game_root()?;
    state.logs.lock().map_err(|e| e.to_string())?.clear();
    *state.source.lock().map_err(|e| e.to_string())? = source.label().into();
    progress(&app, &state, "Checking if I am inside a game folder", None);
    let found = match repair::probe(&root, None).await {
        Ok(found) => found,
        Err(error) => {
            progress(&app, &state, &error, None);
            return Err(error);
        }
    };
    *state.game.lock().map_err(|e| e.to_string())? = Some(found.game);
    for signal in found.evidence {
        state
            .logs
            .lock()
            .map_err(|e| e.to_string())?
            .push(format!("Detected: {signal}"));
    }
    let cache = app
        .path()
        .app_cache_dir()
        .map_err(|e| e.to_string())?
        .join("emulators");
    let identity = EmuIdentity::default();
    let result = repair::apply(&root, &cache, None, source, &identity, |text, percent| {
        progress(&app, &state, text, percent)
    })
    .await;
    match &result {
        Ok(report) => {
            state.logs.lock().map_err(|e| e.to_string())?.push(format!(
                "{} {} · SHA256 {} · {} DLLs · {} interface lists",
                report.source,
                report.version,
                report.archive_sha256,
                report.dlls_replaced,
                report.interfaces_generated
            ));
        }
        Err(error) => progress(&app, &state, error, None),
    }
    result
}
#[tauri::command]
fn fixer_undo(state: State<'_, Fixer>) -> Result<(), String> {
    if state.running.swap(true, Ordering::SeqCst) {
        return Err("A repair is already running.".into());
    }
    let _reset = Reset(&state.running);
    journal::undo(&game_root()?)
}
#[tauri::command]
fn fixer_support(state: State<'_, Fixer>) -> Result<String, String> {
    let game = state.game.lock().map_err(|e| e.to_string())?;
    let source = state.source.lock().map_err(|e| e.to_string())?;
    let logs = state.logs.lock().map_err(|e| e.to_string())?;
    let saved = journal::load(&game_root()?)?.and_then(|j| j.report);
    let text = if logs.is_empty() {
        let details = saved
            .as_ref()
            .map(|r| {
                let mut lines = vec![format!(
                    "{} {} · SHA256 {} · {} DLLs · {} interface lists",
                    r.source, r.version, r.archive_sha256, r.dlls_replaced, r.interfaces_generated
                )];
                lines.extend(r.warnings.clone());
                lines
            })
            .unwrap_or_default();
        repair::support_text(
            game.as_ref().or_else(|| saved.as_ref().map(|r| &r.game)),
            saved.as_ref().map(|r| r.source.as_str()).unwrap_or(&source),
            &details,
        )
    } else {
        repair::support_text(game.as_ref(), &source, &logs)
    };
    kryoto_repair::clipboard::copy(&text)?;
    Ok(text)
}
#[tauri::command]
fn fixer_support_open(state: State<'_, Fixer>) -> Result<(), String> {
    let game = state.game.lock().map_err(|e| e.to_string())?;
    let saved = journal::load(&game_root()?)?.and_then(|j| j.report);
    let mut url = url_for_support(game.as_ref().or_else(|| saved.as_ref().map(|r| &r.game)));
    url.query_pairs_mut()
        .append_pair("new", "1")
        .append_pair("category", "download");
    #[cfg(windows)]
    let result = std::process::Command::new(system_directory()?.join("rundll32.exe"))
        .args(["url.dll,FileProtocolHandler", url.as_str()])
        .spawn();
    #[cfg(not(windows))]
    let result = std::process::Command::new("xdg-open")
        .arg(url.as_str())
        .spawn();
    result.map(|_| ()).map_err(|e| e.to_string())
}
#[cfg(windows)]
fn system_directory() -> Result<PathBuf, String> {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetSystemDirectoryW(buffer: *mut u16, size: u32) -> u32;
    }
    let mut buffer = [0u16; 32768];
    let size = unsafe { GetSystemDirectoryW(buffer.as_mut_ptr(), buffer.len() as u32) } as usize;
    if size == 0 || size >= buffer.len() {
        return Err("Windows could not locate the support browser launcher.".into());
    }
    use std::os::windows::ffi::OsStringExt;
    Ok(PathBuf::from(std::ffi::OsString::from_wide(
        &buffer[..size],
    )))
}
fn url_for_support(game: Option<&GameIdentity>) -> url::Url {
    let mut url = url::Url::parse("https://kryo.to/support").unwrap();
    if let Some(game) = game {
        url.query_pairs_mut()
            .append_pair("game", &game.slug)
            .append_pair(
                "subject",
                &format!("{} does not work after auto-fixer", game.title),
            );
    }
    url
}
#[tauri::command]
fn fixer_look() -> serde_json::Value {
    // Follow Desktop's local appearance; read only and ignore unrelated settings.
    let settings = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .map(|p| p.join("to.kryo.desktop/settings.json"));
    let value = settings
        .and_then(|p| kryoto_repair::detection::read_small(&p))
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .unwrap_or_default();
    serde_json::json!({ "palette": value["palette"].as_str().unwrap_or("monochrome"), "radius": value["radius"].as_str().unwrap_or("pill"), "font": value["font"].as_str().unwrap_or("teletext") })
}
fn main() {
    let context = tauri::generate_context!();
    if std::env::args().any(|arg| arg == "--self-check") {
        let embedded_ui = context.assets().get(&"index.html".into()).is_some();
        let embedded_font = context
            .assets()
            .get(&"fonts/teletext-regular.woff2".into())
            .is_some();
        println!(
            "{}",
            serde_json::json!({ "sources": release::sources(), "embedded_ui": embedded_ui, "embedded_font": embedded_font, "game_folder": game_root().and_then(|root| kryoto_repair::detection::scan(&root)).map(|_| "detected".to_owned()).unwrap_or_else(|e| e) })
        );
        if !embedded_ui || !embedded_font {
            std::process::exit(1)
        }
        return;
    }
    tauri::Builder::default()
        .manage(Fixer::default())
        .invoke_handler(tauri::generate_handler![
            fixer_sources,
            fixer_state,
            fixer_apply,
            fixer_undo,
            fixer_support,
            fixer_support_open,
            fixer_look
        ])
        .run(context)
        .expect("Could not start Kryoto Auto-fixer");
}
