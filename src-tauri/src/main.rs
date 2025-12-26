// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Create the application menu
            let file_menu = Submenu::with_items(
                app,
                "File",
                true,
                &[
                    &MenuItem::with_id(app, "open", "Open XML...", true, Some("CmdOrCtrl+O"))?,
                    &MenuItem::with_id(app, "save", "Save", true, Some("CmdOrCtrl+S"))?,
                    &MenuItem::with_id(app, "save_as", "Save As...", true, Some("CmdOrCtrl+Shift+S"))?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(app, "close", "Close Window", true, Some("CmdOrCtrl+W"))?,
                ],
            )?;

            let edit_menu = Submenu::with_items(
                app,
                "Edit",
                true,
                &[
                    &PredefinedMenuItem::undo(app, Some("Undo"))?,
                    &PredefinedMenuItem::redo(app, Some("Redo"))?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::cut(app, Some("Cut"))?,
                    &PredefinedMenuItem::copy(app, Some("Copy"))?,
                    &PredefinedMenuItem::paste(app, Some("Paste"))?,
                    &PredefinedMenuItem::select_all(app, Some("Select All"))?,
                ],
            )?;

            let view_menu = Submenu::with_items(
                app,
                "View",
                true,
                &[
                    &MenuItem::with_id(app, "expand_all", "Expand All", true, Some("CmdOrCtrl+E"))?,
                    &MenuItem::with_id(app, "collapse_all", "Collapse All", true, Some("CmdOrCtrl+Shift+E"))?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(app, "toggle_tree", "Toggle Tree Panel", true, Some("CmdOrCtrl+T"))?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::fullscreen(app, Some("Toggle Fullscreen"))?,
                ],
            )?;

            let help_menu = Submenu::with_items(
                app,
                "Help",
                true,
                &[
                    &MenuItem::with_id(app, "about", "About XML Navigator", true, None::<&str>)?,
                    &MenuItem::with_id(app, "check_updates", "Check for Updates...", true, None::<&str>)?,
                ],
            )?;

            #[cfg(target_os = "macos")]
            let menu = Menu::with_items(
                app,
                &[
                    &Submenu::with_items(
                        app,
                        "XML Navigator",
                        true,
                        &[
                            &PredefinedMenuItem::about(app, Some("About XML Navigator"), None)?,
                            &PredefinedMenuItem::separator(app)?,
                            &PredefinedMenuItem::services(app, Some("Services"))?,
                            &PredefinedMenuItem::separator(app)?,
                            &PredefinedMenuItem::hide(app, Some("Hide XML Navigator"))?,
                            &PredefinedMenuItem::hide_others(app, Some("Hide Others"))?,
                            &PredefinedMenuItem::show_all(app, Some("Show All"))?,
                            &PredefinedMenuItem::separator(app)?,
                            &PredefinedMenuItem::quit(app, Some("Quit XML Navigator"))?,
                        ],
                    )?,
                    &file_menu,
                    &edit_menu,
                    &view_menu,
                    &help_menu,
                ],
            )?;

            #[cfg(not(target_os = "macos"))]
            let menu = Menu::with_items(
                app,
                &[&file_menu, &edit_menu, &view_menu, &help_menu],
            )?;

            app.set_menu(menu)?;

            // Handle menu events
            app.on_menu_event(|app, event| {
                let window = app.get_webview_window("main").unwrap();
                match event.id().as_ref() {
                    "open" => {
                        let _ = window.eval("window.tauriOpenFile && window.tauriOpenFile()");
                    }
                    "save" => {
                        let _ = window.eval("window.tauriSaveFile && window.tauriSaveFile()");
                    }
                    "save_as" => {
                        let _ = window.eval("window.tauriSaveFileAs && window.tauriSaveFileAs()");
                    }
                    "close" => {
                        let _ = window.close();
                    }
                    "expand_all" => {
                        let _ = window.eval("window.expandAllNodes && window.expandAllNodes()");
                    }
                    "collapse_all" => {
                        let _ = window.eval("window.collapseAllNodes && window.collapseAllNodes()");
                    }
                    "toggle_tree" => {
                        let _ = window.eval("window.toggleTreePanel && window.toggleTreePanel()");
                    }
                    "about" => {
                        let _ = window.eval("window.showAboutDialog && window.showAboutDialog()");
                    }
                    "check_updates" => {
                        let _ = window.eval("window.checkForUpdates && window.checkForUpdates()");
                    }
                    _ => {}
                }
            });

            // Create system tray
            let tray_menu = Menu::with_items(
                app,
                &[
                    &MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?,
                    &MenuItem::with_id(app, "tray_open", "Open XML File...", true, None::<&str>)?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?,
                ],
            )?;

            let _tray = TrayIconBuilder::new()
                .menu(&tray_menu)
                .tooltip("XML Navigator")
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "tray_open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = window.eval("window.tauriOpenFile && window.tauriOpenFile()");
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

