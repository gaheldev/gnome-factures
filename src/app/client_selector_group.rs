use std::fmt;

use adw::prelude::*;
use relm4::prelude::*;
use relm4_components::simple_adw_combo_row::{SimpleComboRow, SimpleComboRowMsg};

use crate::app::{Client, ClientName};


pub(crate) struct ClientSelectorGroupModel {
    client_list: Vec<Client>,
    client: Client,
    client_row: Controller<SimpleComboRow<Client>>,
    current_index: Option<usize>,
}

#[derive(Debug)]
pub(crate) enum ClientSelectorGroupOutput {
    /// fired whenever a client is selected, created or modified
    Selected(Client),
    Create,
    Edit,
}

#[derive(Debug)]
pub(crate) enum ClientSelectorGroupInput {
    Selected(usize),
    Edited(Client),
    Created(Client),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for ClientSelectorGroupModel {
    type Init = Vec<Client>;
    type Input = ClientSelectorGroupInput;
    type Output = ClientSelectorGroupOutput;

    view! {
        // combo row to select and existing client
        #[name(combo_row_group)]
        adw::PreferencesGroup {
            set_title: "Client",
            #[wrap(Some)]
            set_header_suffix = &gtk::Box {
                append = &gtk::Button {
                    set_tooltip: "Modifier le client",
                    set_icon_name: "document-edit-symbolic",
                    set_focus_on_click: false,
                    set_has_frame: false,
                    set_has_tooltip: true,

                    add_css_class: "circular",

                    connect_clicked[sender] => move |_| {
                        sender.output(ClientSelectorGroupOutput::Edit).unwrap();
                    },
                },

                append = &gtk::Button {
                    set_tooltip: "Nouveau client",
                    set_icon_name: "contact-new-symbolic",
                    set_focus_on_click: false,
                    set_has_frame: false,
                    set_has_tooltip: true,

                    add_css_class: "circular",

                    connect_clicked[sender] => move |_| {
                        sender.output(ClientSelectorGroupOutput::Create).unwrap();
                    },
                },
            },

            add = model.client_row.widget(),
        },
    }

    fn init(
        client_list: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let active_index = None;

        let client_row: Controller<SimpleComboRow<Client>> =
        SimpleComboRow::builder()
            .launch(SimpleComboRow {
                variants: client_list.clone(),
                active_index,
            })
            .forward(sender.input_sender(), ClientSelectorGroupInput::Selected);


        let model = ClientSelectorGroupModel {
            client_list,
            client: Client::default(),
            client_row,
            current_index: active_index,
        };
        sender.input_sender().emit(ClientSelectorGroupInput::Selected(0));
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ClientSelectorGroupInput::Created(client) => {
                self.client_list.push(client);
                self.current_index = Some(self.client_list.len()-1);
                self.update_combo();
                // select the new client
                sender.input(ClientSelectorGroupInput::Selected(self.current_index.unwrap()));
            },
            ClientSelectorGroupInput::Edited(client) => {
                self.client = client;
                let index = self.current_index.expect("Shouldn't be able to edit when there's no client");
                self.client_list[index] = self.client.clone();
                self.update_combo();
                sender.output(ClientSelectorGroupOutput::Selected(self.client.clone())).unwrap();
            },
            ClientSelectorGroupInput::Selected(index) => {
                self.client = self.client_list[index].clone();
                self.current_index = Some(index);
                sender.output(ClientSelectorGroupOutput::Selected(self.client.clone())).unwrap();
            },
            // TODO: test with only one client

            // ClientSelectorGroupInput::Existing(value) => {
            //     self.existing = value;
            //     self.editing = !self.existing;
            //     // FIXME: this is kind of a hack to update edited client when there's a single
            //     // client in the client_list (can't select it in combobox)
            //     if self.existing && !self.client_list.is_empty() {
            //         if let Some(i) = self.client_row.model().active_index {
            //             sender.input(ClientSelectorGroupInput::ClientSelected(i));
            //         } else {
            //             self.client_row.widget().set_selected(0);
            //             sender.input(ClientSelectorGroupInput::ClientSelected(0));
            //         };
            //     };
            // },
        }

        self.client_row.widget().set_subtitle(&self.display_client());
    }
}

impl ClientSelectorGroupModel {
    fn display_client(&self) -> String {
        // TODO: replace \\ with newline
        let name = self.client.name.clone();
        let address = self.client.address.clone();
        let siret = if let Some(s) = self.client.siret.clone() {
            format!("\nSIRET: {s}")
        } else {
            "".to_string()
        };

        let code_ape = if let Some(s) = self.client.code_ape.clone() {
            format!("\nAPE: {s}")
        } else {
            "".to_string()
        };

        let tva = if let Some(s) = self.client.tva.clone() {
            format!("\nTVA: {s}")
        } else {
            "".to_string()
        };

        let tva_icc = if let Some(s) = self.client.tva_icc.clone() {
            format!("\nTVA ICC: {s}")
        } else {
            "".to_string()
        };

        let custom_field = if let Some(s) = self.client.custom_field.clone() {
            format!("\n{s}")
        } else {
            "".to_string()
        };

        format!("{name}\n{address}{siret}{code_ape}{tva}{tva_icc}{custom_field}")
    }

    fn update_combo(&self) {
        let updated_combo = SimpleComboRow {
            variants: self.client_list.clone(),
            active_index: self.current_index,
        };

        self.client_row
            .sender()
            .send(SimpleComboRowMsg::UpdateData(updated_combo))
            .unwrap();
    }
}
