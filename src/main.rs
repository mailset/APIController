// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use api_controller::handlers::headers_setting::HeadersSettingHandler;
use api_controller::handlers::http_request::HttpRequestHandler;

use api_controller::ui::*;
use futures::executor::block_on;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use slint::{Model, ModelRc, ToSharedString, VecModel, language::StandardListViewItem};

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
    let mut headers_setting_handler = HeadersSettingHandler::new();
    headers_setting_handler.initialize_ui(&headers_dialog);

    // Open Headers Setting Dialog
    ui.on_headers_setting_clicked(move || {
        headers_dialog.show().unwrap();
    });

    // Send Requests
    let ui_model = ui.as_weak();
    ui.on_request_clicked(move |is_post| {
        // Start loading progress bar.
        let ui = ui_model.upgrade().unwrap();
        ui.set_loading(true);

        // Initialize Request Data
        let request_data = ui.global::<RequestData>();
        let headers_rc = request_data.get_request_headers();
        let mut headers = HeaderMap::new();
        let body = request_data.get_request_content().to_string();
        let user_agent = request_data.get_request_ua().to_string();
        let url = request_data.get_request_url().to_string();

        // Initializing http request handler
        let http_request_handler = HttpRequestHandler::new();

        // Generate Headers List which will recoginized by reqwest from slint's tabview
        if let Some(headers_raw) = headers_rc
            .as_any()
            .downcast_ref::<VecModel<ModelRc<StandardListViewItem>>>()
        {
            for row in headers_raw.iter() {
                let name = row.row_data(0).map(|item| item.text).unwrap_or_default();
                let value = row.row_data(1).map(|item| item.text).unwrap_or_default();
                headers.insert(
                    HeaderName::from_lowercase(name.to_lowercase().as_bytes()).unwrap(),
                    HeaderValue::from_str(value.as_str()).unwrap(),
                );
            }
        }

        // Initialize UI
        let result_dialog_weak = result_dialog.as_weak();
        let ui_weak = ui.as_weak();
        tokio::spawn(async move {
            let response = http_request_handler
                .send_request(headers, body, user_agent, url, is_post)
                .await;

            slint::invoke_from_event_loop(move || {
                let result_dialog = result_dialog_weak.unwrap();
                let ui = ui_weak.unwrap();
                match response {
                    Ok(response) => {
                        block_on(http_request_handler.initialize_ui(&result_dialog, response))
                            .unwrap();
                        result_dialog.show().unwrap();
                    }
                    Err(error) => {
                        let error_dialog = ErrorDialog::new().unwrap();
                        error_dialog.set_error_info(format!("{}", error).to_shared_string());
                        error_dialog.show().unwrap();
                    }
                }
                ui.set_loading(false);
            })
        });
    });

    ui.run()?;

    Ok(())
}
