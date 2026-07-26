// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        "application/json".parse().unwrap(),
    );

    let ui_handle = ui.as_weak();
    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });
    ui.on_request_clicked(move |is_post| {
        let ui = ui_handle.unwrap();
    });

    ui.run()?;

    Ok(())
}

fn send_request(headers: HeaderMap, is_post: bool) {}
