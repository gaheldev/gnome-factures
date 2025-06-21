use std::fmt;

use adw::prelude::*;
use relm4::prelude::*;
use relm4_components::simple_adw_combo_row::SimpleComboRow;

// FIXME: use different tabs for existing and new client

pub type ClientName = String;

#[derive(Debug,Clone,Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Address {
    pub number_and_street: String,
    pub postcode: String,
    pub city: String,
    // country: String,
}
impl Address {
    pub(crate) fn valid(&self) -> bool {
        if self.number_and_street.is_empty() { return false; }
        if self.postcode.is_empty() { return false; }
        if self.city.is_empty() { return false; }
        true
    }
}

#[derive(Debug,Clone,Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Client {
    pub name: ClientName,
    pub address: Address,
    pub siret: Option<String>,
    pub tva: Option<String>,
    pub tva_icc: Option<String>,
    pub custom_field: Option<String>,
}
impl Client {
    pub(crate) fn valid(&self) -> bool {
        if self.name.is_empty() { return false; }
        if !self.address.valid() { return false; }
        true
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{}", self.name)
    }
}

pub(crate) struct ClientFormModel {
    client_list: Vec<Client>,
    edited_client: Client,
    client_row: Controller<SimpleComboRow<Client>>,
    editing: bool,
    /// If current client exists already or not
    existing: bool,
}

#[derive(Debug)]
pub(crate) enum ClientFormOutput {
    ClientEdited(Client),
    // AddClient(Client),
    // SelectClient(ClientName),
}

#[derive(Debug)]
pub(crate) enum ClientFormInput {
    ClientSelected(usize),
    AllowEditing,
    BlockEditing,
    Existing(bool),
    NameEdited(ClientName),
    StreetEdited(String),
    PostcodeEdited(String),
    CityEdited(String),
    // CountryEdited(String),
    SiretEdited(String),
    TVAEdited(String),
    TVAICCEdited(String),
    CustomFieldEdited(String),
}

// FIXME: sometimes crash when clicking existing client on load
#[relm4::component(pub(crate))]
impl SimpleComponent for ClientFormModel {
    type Init = Vec<Client>;
    type Input = ClientFormInput;
    type Output = ClientFormOutput;

    view! {
        adw::PreferencesPage {
            add = &adw::PreferencesGroup {

                // homemade toggle button between two states
                // FIXME: disallow clicking a toggled button
                // -> use tabs and make form a dedicated component
                gtk::Box {
                    add_css_class: "linked",
                    set_homogeneous: true,

                    gtk::ToggleButton {
                        set_label: "Existing Client",

                        #[watch]
                        #[block_signal(existing_handler)]
                        set_active: model.existing,

                        // prevent setting button to false by clicking on it
                        connect_clicked => move |button| {
                            if !model.existing && !button.is_active() {
                                button.set_active(true);
                            }
                        },

                        connect_toggled[sender] => move |button| {
                            if button.is_active() {
                                sender.input(ClientFormInput::Existing(true));
                            }
                        } @existing_handler,
                    },

                    gtk::ToggleButton {
                        set_label: "New Client",

                        #[watch]
                        #[block_signal(new_handler)]
                        set_active: !(model.existing),

                        // prevent setting button to false by clicking on it
                        connect_clicked => move |button| {
                            if !model.existing && !button.is_active() {
                                button.set_active(true);
                            }
                        },

                        connect_toggled[sender] => move |button| {
                            if button.is_active() {
                                sender.input(ClientFormInput::Existing(false));
                            }
                        } @new_handler,
                    },
                },
            },

            // combo row to select and existing client
            #[name(combo_row_group)]
            adw::PreferencesGroup {
                set_visible: model.existing, // initialization
                add = model.client_row.widget(),
            },

            #[name(edit_group)]
            add = &adw::PreferencesGroup {
                // TODO: manually implement headerbox and set its visibility
                #[watch]
                set_title: match model.existing {
                    true => "Edit",
                    false => "",
                },

                #[wrap(Some)]
                set_header_suffix = &gtk::Switch {
                    set_valign: gtk::Align::Center,

                    #[watch]
                    #[block_signal(editing_handler)]
                    set_active: model.editing,

                    #[watch]
                    set_visible: model.existing,

                    connect_active_notify[sender] => move |switch| {
                        match switch.is_active() {
                            true => sender.input(ClientFormInput::AllowEditing),
                            false => sender.input(ClientFormInput::BlockEditing),
                        }
                    } @editing_handler,
                },

                add = &adw::EntryRow {
                    set_title: "Name *",

                    #[watch]
                    set_editable: model.editing && !model.existing,

                    #[track(!model.editing)]
                    #[block_signal(name_handler)]
                    set_text: &model.edited_client.name,

                    #[watch] set_css_classes: if model.edited_client.name.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(ClientFormInput::NameEdited(entry_row.property("text")));
                    } @name_handler
                },

                add = &adw::EntryRow {
                    set_title: "Address *",

                    #[watch]
                    set_editable: model.editing,

                    #[track(!model.editing)]
                    #[block_signal(address_handler)]
                    set_text: &model.edited_client.address.number_and_street,

                    #[watch] set_css_classes: if model.edited_client.address.number_and_street.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(ClientFormInput::StreetEdited(entry_row.property("text")));
                    } @address_handler
                },

                add = &adw::EntryRow {
                    set_title: "Postcode *",

                    #[watch]
                    set_editable: model.editing,

                    #[track(!model.editing)]
                    #[block_signal(postcode_handler)]
                    set_text: &model.edited_client.address.postcode,

                    #[watch] set_css_classes: if model.edited_client.address.postcode.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(ClientFormInput::PostcodeEdited(entry_row.property("text")));
                    } @postcode_handler
                },

                add = &adw::EntryRow {
                    set_title: "City *",

                    #[watch]
                    set_editable: model.editing,

                    #[track(!model.editing)]
                    #[block_signal(city_handler)]
                    set_text: &model.edited_client.address.city,

                    #[watch] set_css_classes: if model.edited_client.address.city.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(ClientFormInput::CityEdited(entry_row.property("text")));
                    } @city_handler
                },

                // add = &adw::EntryRow {
                //     set_title: "Country",
                //     connect_changed[sender] => move |entry_row| {
                //         sender.input(ClientFormInput::CountryEdited(entry_row.property("text")));
                //     }
                // },
            },

            #[name(edit_group_2)]
            add = &adw::PreferencesGroup {
                add = &adw::EntryRow {
                    set_title: "SIRET",

                    #[watch]
                    set_editable: model.editing,

                    #[track(!model.editing)]
                    #[block_signal(siret_handler)]
                    set_text: if let Some(siret) = &model.edited_client.siret { siret } else { "" },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(ClientFormInput::SiretEdited(entry_row.property("text")));
                    } @siret_handler
                },

                add = &adw::EntryRow {
                    set_title: "TVA",

                    #[watch]
                    set_editable: model.editing,

                    #[track(!model.editing)]
                    #[block_signal(tva_handler)]
                    set_text: if let Some(tva) = &model.edited_client.tva { tva } else { "" },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(ClientFormInput::TVAEdited(entry_row.property("text")));
                    } @tva_handler
                },

                add = &adw::EntryRow {
                    set_title: "TVA intracommunautaire",

                    #[watch]
                    set_editable: model.editing,

                    #[track(!model.editing)]
                    #[block_signal(tva_icc_handler)]
                    set_text: if let Some(tva_icc) = &model.edited_client.tva_icc { tva_icc } else { "" },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(ClientFormInput::TVAICCEdited(entry_row.property("text")));
                    } @tva_icc_handler
                },
            },
            add = &adw::PreferencesGroup {
                set_title: "Informations additionnelles",

                add = &gtk::TextView {
                    set_height_request: 200,
                    set_wrap_mode: gtk::WrapMode::Word,
                    inline_css: "border-radius: 14px; padding: 10px",

                    #[watch]
                    set_editable: model.editing,

                    #[wrap(Some)]
                    set_buffer = &gtk::TextBuffer {
                        connect_end_user_action[sender] => move |entry| {
                            sender.input(ClientFormInput::CustomFieldEdited(entry.property("text")));
                        },

                        #[track(!model.editing)]
                        set_text: if let Some(custom) = &model.edited_client.custom_field { custom } else { "" },
                    }
                },
            },
        },

    }

    fn pre_view() {
        if self.existing != widgets.combo_row_group.is_visible() {
            widgets.combo_row_group.set_visible(self.existing);
        }
    }

    fn init(
        client_list: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let client_row: Controller<SimpleComboRow<Client>> =
        SimpleComboRow::builder()
            .launch(SimpleComboRow {
                variants: client_list.clone(),
                active_index: None
            })
            .forward(sender.input_sender(), ClientFormInput::ClientSelected);

        client_row.widget().set_title("Select client");

        let model = ClientFormModel {
            client_list,
            edited_client: Client::default(),
            client_row,
            editing: true,
            existing: false,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ClientFormInput::NameEdited(value) => {
                self.edited_client.name = value;
                let _ = sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone()));
            }
            ClientFormInput::StreetEdited(value) => {
                self.edited_client.address.number_and_street = value;
                let _ = sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone()));
            }
            ClientFormInput::PostcodeEdited(value) => {
                self.edited_client.address.postcode = value;
                let _ = sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone()));
            }
            ClientFormInput::CityEdited(value) => {
                self.edited_client.address.city = value;
                let _ = sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone()));
            }
            // ClientFormInput::CountryEdited(value) => {
            //     self.edited_client.address.country = value;
            //     let _ = sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone()));
            // }
            ClientFormInput::SiretEdited(value) => {
                self.edited_client.siret = if value.is_empty() { None } else { Some(value) };
                let _ = sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone()));
            }
            ClientFormInput::TVAEdited(value) => {
                self.edited_client.tva = if value.is_empty() { None } else { Some(value) };
                sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone())).unwrap();
            }
            ClientFormInput::TVAICCEdited(value) => {
                self.edited_client.tva_icc = if value.is_empty() { None } else { Some(value) };
                sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone())).unwrap();
            }
            ClientFormInput::CustomFieldEdited(value) => {
                if value.is_empty() {
                    self.edited_client.custom_field = None;
                } else {
                    self.edited_client.custom_field = Some(value);
                }
                sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone())).unwrap();
            },
            ClientFormInput::ClientSelected(index) => {
                self.edited_client = self.client_list[index].clone();
                self.existing = true;
                self.editing = false;
                sender.output(ClientFormOutput::ClientEdited(self.edited_client.clone())).unwrap();
            },
            ClientFormInput::AllowEditing => self.editing = true,
            ClientFormInput::BlockEditing => self.editing = false,
            ClientFormInput::Existing(value) => {
                self.existing = value;
                self.editing = !self.existing;
                // FIXME: this is kind of a hack to update edited client when there's a single
                // client in the client_list (can't select it in combobox)
                if self.existing && !self.client_list.is_empty() {
                    if let Some(i) = self.client_row.model().active_index {
                        sender.input(ClientFormInput::ClientSelected(i));
                    } else {
                        self.client_row.widget().set_selected(0);
                        sender.input(ClientFormInput::ClientSelected(0));
                    };
                };
            },
        }
    }
}
