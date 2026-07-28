use reqwest::{Client, Error, Response, StatusCode, header::HeaderMap};
use slint::ToSharedString;

use crate::ui::ResultDialog;

pub struct HttpRequestHandler {}

impl HttpRequestHandler {
    pub fn new() -> Self {
        HttpRequestHandler {}
    }

    pub fn initialize_ui(
        &self,
        dialog: &ResultDialog,
        headers: HeaderMap,
        status: StatusCode,
        body: String,
    ) {
        // clone headers to avoid borrowing `response` across the async await point below

        let mut headers_string = String::from("");
        for (header_name, header_value) in headers.iter() {
            headers_string.push_str(
                format!(
                    "{}: {}\n",
                    header_name.as_str(),
                    header_value.to_str().unwrap()
                )
                .as_str(),
            );
        }
        let status_str = format!("{} {}", status.as_u16(), status.canonical_reason().unwrap());

        // TODO: Copy to clipboard
        dialog.set_request_result_body(body.to_shared_string());
        dialog.set_request_result_status(status_str.to_shared_string());
        dialog.set_request_result_head(headers_string.to_shared_string());
    }

    pub async fn send_request(
        &self,
        headers: HeaderMap,
        body: String,
        user_agent: String,
        url: String,
        is_post: bool,
    ) -> Result<Response, Error> {
        // Generate Client
        let client = Client::builder()
            .user_agent(user_agent.as_str())
            .default_headers(headers)
            .build()
            .unwrap();
        // Send Requests.
        if is_post {
            client.post(url.as_str()).body(body).send().await
        } else {
            client.get(url.as_str()).body(body).send().await
        }
    }
}
