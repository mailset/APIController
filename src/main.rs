// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use crate::headers_setting_handler::HeadersSettingHandler;

pub mod ui {
    slint::include_modules!();
}

mod headers_setting_handler;

use ui::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize Logger
    env_logger::Builder::default()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    // Initialize Windows and Dialogs
    let ui = AppWindow::new()?;
    let headers_dialog = HeadersSettingDialog::new()?;
    let result_dialog = ResultDialog::new()?;

    let mut headers_setting_handler = HeadersSettingHandler::new();
    headers_setting_handler.initialize_ui(&headers_dialog);

    ui.on_headers_setting_clicked(move || {
        headers_dialog.show().unwrap();
    });

    ui.run()?;

    Ok(())
}

// fn send_request(headers: HeaderMap, is_post: bool) {}
