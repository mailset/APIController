use reqwest::{
    Client, Error, Response,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use slint::{Model, ModelRc, ToSharedString, VecModel, language::StandardListViewItem};

use crate::ui::{RequestData, ResultDialog};

pub struct HttpRequestHandler {}

impl HttpRequestHandler {
    pub fn new() -> Self {
        HttpRequestHandler {}
    }

    pub async fn initialize_ui(
        &self,
        dialog: &ResultDialog,
        response: Response,
    ) -> Result<(), Error> {
        // clone headers to avoid borrowing `response` across the async await point below
        let headers = response.headers().clone();
        let status = response.status();
        let body = response.text().await?;
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

        dialog.set_request_result_body(body.to_shared_string());
        dialog.set_request_result_status(status_str.to_shared_string());
        dialog.set_request_result_head(headers_string.to_shared_string());
        Ok(())
    }

    pub async fn send_request(
        &self,
        request_data: RequestData<'_>,
        is_post: bool,
    ) -> Result<Response, Error> {
        // Initializing values that request needs.
        let mut headers = HeaderMap::new();
        let headers_rc = request_data.get_request_headers();
        let body = request_data.get_request_content().to_string();
        let user_agent = request_data.get_request_ua();
        let url = request_data.get_request_url();

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

        // Generate Client
        let client = Client::builder()
            .user_agent(user_agent.as_str())
            .default_headers(headers)
            .build()
            .unwrap();
        // TODO: Error Catching(Such as non-existing domain handler) needed here.
        // Send Requests.
        let res = if is_post {
            client.post(url.as_str()).body(body).send().await?
        } else {
            client.get(url.as_str()).body(body).send().await?
        };

        Ok(res)
    }
}
