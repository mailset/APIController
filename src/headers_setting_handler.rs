use std::rc::Rc;

use slint::{ComponentHandle, Model, ModelRc, VecModel, language::StandardListViewItem};

use crate::ui::{HeadersSettingDialog, RequestData};

pub struct HeadersSettingHandler {}

impl HeadersSettingHandler {
    pub fn new() -> Self {
        HeadersSettingHandler {}
    }

    pub fn initialize_ui(&mut self, dialog: &HeadersSettingDialog) {
        // Add A HTTP Header
        let dialog_model = dialog.as_weak();
        dialog.on_add_row(move || {
            let dialog = dialog_model.upgrade().unwrap();
            let request_data = dialog.global::<RequestData>();
            let headers_rc = request_data.get_request_headers();
            if let Some(headers) = headers_rc
                .as_any()
                .downcast_ref::<VecModel<ModelRc<StandardListViewItem>>>()
            {
                headers.push(ModelRc::from(Rc::new(VecModel::from(vec![
                    StandardListViewItem::from(dialog.get_name_edit()),
                    StandardListViewItem::from(dialog.get_value_edit()),
                ]))));
                log::info!(
                    "Added New HTTP Header: {}: {}",
                    dialog.get_name_edit(),
                    dialog.get_name_edit()
                );
            } else {
                log::error!(
                    "Error Adding A HTTP Header: {}: {}",
                    dialog.get_name_edit(),
                    dialog.get_value_edit()
                );
            }
        });

        // Delete A HTTP Header
        let dialog_model = dialog.as_weak();
        dialog.on_delete_row(move |current_row| {
            let dialog = dialog_model.upgrade().unwrap();
            let request_data = dialog.global::<RequestData>();
            let headers_rc = request_data.get_request_headers();
            if let Some(headers) = headers_rc
                .as_any()
                .downcast_ref::<VecModel<ModelRc<StandardListViewItem>>>()
            {
                if (current_row as usize) < headers.row_count() {
                    headers.remove(current_row as usize);
                    log::info!("Removed HTTP Header: Index {}", current_row);
                }
            } else {
                log::error!("Error Removing A HTTP Header: Index {}", current_row);
            }
        });

        // Modify A HTTP Header
        let dialog_model = dialog.as_weak();
        dialog.on_modify_row(move |current_row| {
            let dialog = dialog_model.upgrade().unwrap();
            let request_data = dialog.global::<RequestData>();
            let headers_rc = request_data.get_request_headers();
            if let Some(headers) = headers_rc
                .as_any()
                .downcast_ref::<VecModel<ModelRc<StandardListViewItem>>>()
            {
                if (current_row as usize) < headers.row_count() {
                    headers.set_row_data(
                        current_row as usize,
                        ModelRc::from(Rc::new(VecModel::from(vec![
                            StandardListViewItem::from(dialog.get_name_edit()),
                            StandardListViewItem::from(dialog.get_value_edit()),
                        ]))),
                    );
                    log::info!(
                        "Modified HTTP Header: Index {} As {}: {}",
                        current_row,
                        dialog.get_name_edit(),
                        dialog.get_value_edit()
                    );
                }
            } else {
                log::error!(
                    "Error Modifing HTTP Header: Index {} As {}: {}",
                    current_row,
                    dialog.get_name_edit(),
                    dialog.get_value_edit()
                );
            }
        });
    }
}
