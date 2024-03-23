#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows_sys::Win32::Graphics::Dwm::*;
use windows_sys::Win32::Foundation::*;
use tauri::{Theme, WebviewWindow};

#[tauri::command]
fn set_theme(window:WebviewWindow,theme: &str) {
    if theme=="dark" {
        set_window_theme(&window,Theme::Dark);
    }
    else {
        set_window_theme(&window,Theme::Light);
    }
}

pub fn set_window_theme(window:&WebviewWindow,theme:Theme) {
    unsafe {
        let handle = window.hwnd().unwrap().0;
        let value: BOOL = if theme == Theme::Dark {1} else {0};
        let attribute = DWMWA_USE_IMMERSIVE_DARK_MODE;
        DwmSetWindowAttribute(handle, attribute as u32, &value as *const _ as *const _, std::mem::size_of::<BOOL>() as u32);
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![set_theme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
