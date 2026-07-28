// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use api_controller::handlers::headers_setting::HeadersSettingHandler;
use api_controller::handlers::http_request::HttpRequestHandler;

use api_controller::ui::*;
use futures::executor::block_on;
use slint::ToSharedString;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize Logger
    env_logger::Builder::default()
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Info
        } else {
            log::LevelFilter::Info
        })
        .init();

    // Initialize Windows and Dialogs
    let ui = AppWindow::new()?;
    let headers_dialog = HeadersSettingDialog::new()?;
    let result_dialog = ResultDialog::new()?;

    //Initialize handlers
    let http_request_handler = HttpRequestHandler::new();
    let mut headers_setting_handler = HeadersSettingHandler::new();
    headers_setting_handler.initialize_ui(&headers_dialog);

    // Open Headers Setting Dialog
    ui.on_headers_setting_clicked(move || {
        headers_dialog.show().unwrap();
    });

    // Send Requests
    let ui_model = ui.as_weak();
    ui.on_request_clicked(move |is_post| {
        let ui = ui_model.unwrap();
        ui.set_loading(true);
        let request_data = ui.global::<RequestData>();

        // TODO: Non-blocking method needed here.
        match block_on(http_request_handler.send_request(request_data, is_post)) {
            Ok(response) => {
                block_on(http_request_handler.initialize_ui(&result_dialog, response)).unwrap();
                result_dialog.show().unwrap();
            }
            Err(error) => {
                let error_dialog = ErrorDialog::new().unwrap();
                error_dialog.set_error_info(format!("{}", error).to_shared_string());
                error_dialog.show().unwrap();
            }
        }

        ui.set_loading(false);
    });

    ui.run()?;

    Ok(())
}
