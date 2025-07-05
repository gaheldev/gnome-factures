use adw::prelude::*;
use relm4::prelude::*;

use crate::app::Client;


#[derive(Clone)]
enum Mode {
    Editing,
    Creating,
}

#[derive(Debug, Clone)]
enum Field {
    Name(String),
    NumberAndStreet(String),
    Postcode(String),
    City(String),
    Siret(String),
    Ape(String),
    Tva(String),
    TvaIcc(String),
    Custom(String),
}

#[derive(Clone)]
pub(crate) struct ClientFormModel {
    client: Client,
    mode: Mode,
    is_valid: bool,
    has_been_edited: bool,
    initializing: bool,
}

#[derive(Debug)]
pub(crate) enum ClientFormOutput {
    Edited(Client),
    Created(Client),
    Hiding,
}

#[derive(Debug)]
pub(crate) enum ClientFormInput {
    Edit(Client),
    Create,
    Edited(Field), // FIXME: should be private
    Validated, // FIXME: should be private
}

#[relm4::component(pub(crate))]
impl SimpleComponent for ClientFormModel {
    type Init = ();
    type Input = ClientFormInput;
    type Output = ClientFormOutput;

    view! {
        add = &adw::NavigationPage {
            set_title: "Edition du client",

            connect_hiding[sender] => move |_| sender.output(ClientFormOutput::Hiding).unwrap(),

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    set_show_end_title_buttons: false,
                    pack_end = &gtk::Button {
                        #[watch] set_sensitive: model.is_valid && model.has_been_edited,
                        #[watch] set_css_classes: if !model.has_been_edited { &[""] } else { match model.is_valid {
                                true => match model.mode {
                                    Mode::Creating => &["suggested-action"],
                                    Mode::Editing => &["destructive-action"],
                                },
                                false => &[""],
                            }
                        },
                        #[watch] set_label: match model.mode {
                            Mode::Editing => "Modifier",
                            Mode::Creating => "Ajouter",
                        },
                        connect_clicked => ClientFormInput::Validated,
                    },
                },

                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    add = &adw::PreferencesGroup {
                        #[name(name)]
                        add = &adw::EntryRow {
                            set_title: "Nom *",

                            #[track(model.initializing)]
                            #[block_signal(name_handler)]
                            set_text: &model.client.name,

                            #[watch] set_css_classes: if model.client.name.is_empty() { &["error"] } else { &[""] },

                            connect_changed[sender] => move |entry_row| {
                                sender.input(ClientFormInput::Edited(Field::Name(entry_row.property("text"))));
                            } @name_handler,
                        },

                        #[name(address)]
                        add = &adw::EntryRow {
                            set_title: "Addresse *",

                            #[track(model.initializing)]
                            #[block_signal(number_and_street_handler)]
                            set_text: &model.client.address.number_and_street,

                            #[watch] set_css_classes: if model.client.address.number_and_street.is_empty() { &["error"] } else { &[""] },

                            connect_changed[sender] => move |entry_row| {
                                sender.input(ClientFormInput::Edited(Field::NumberAndStreet(entry_row.property("text"))));
                            } @number_and_street_handler,
                        },

                        #[name(postcode)]
                        add = &adw::EntryRow {
                            set_title: "Code postal *",

                            #[track(model.initializing)]
                            #[block_signal(postcode_handler)]
                            set_text: &model.client.address.postcode,

                            #[watch] set_css_classes: if model.client.address.postcode.is_empty() { &["error"] } else { &[""] },

                            connect_changed[sender] => move |entry_row| {
                                sender.input(ClientFormInput::Edited(Field::Postcode(entry_row.property("text"))));
                            } @postcode_handler,
                        },

                        #[name(city)]
                        add = &adw::EntryRow {
                            set_title: "Ville *",

                            #[track(model.initializing)]
                            #[block_signal(city_handler)]
                            set_text: &model.client.address.city,

                            #[watch] set_css_classes: if model.client.address.city.is_empty() { &["error"] } else { &[""] },

                            connect_changed[sender] => move |entry_row| {
                                sender.input(ClientFormInput::Edited(Field::City(entry_row.property("text"))));
                            } @city_handler,
                        },

                        // add = &adw::EntryRow {
                        //     set_title: "Country",
                        //     connect_changed[sender] => move |entry_row| {
                        //         sender.input(ClientFormInput::CountryEdited(entry_row.property("text")));
                        //     }
                        // },
                    },

                    add = &adw::PreferencesGroup {
                        #[name(siret)]
                        add = &adw::EntryRow {
                            set_title: "SIRET",
                            #[track(model.initializing)]
                            #[block_signal(siret_handler)]
                            set_text: if let Some(siret) = &model.client.siret { siret } else { "" },

                            connect_changed[sender] => move |entry_row| {
                                sender.input(ClientFormInput::Edited(Field::Siret(entry_row.property("text"))));
                            } @siret_handler,
                        },

                        #[name(code_ape)]
                        add = &adw::EntryRow {
                            set_title: "Code APE",
                            #[track(model.initializing)]
                            #[block_signal(code_ape_handler)]
                            set_text: if let Some(code_ape) = &model.client.code_ape { code_ape } else { "" },

                            connect_changed[sender] => move |entry_row| {
                                sender.input(ClientFormInput::Edited(Field::Ape(entry_row.property("text"))));
                            } @code_ape_handler,
                        },

                        #[name(tva)]
                        add = &adw::EntryRow {
                            set_title: "TVA",
                            #[track(model.initializing)]
                            #[block_signal(tva_handler)]
                            set_text: if let Some(tva) = &model.client.tva { tva } else { "" },

                            connect_changed[sender] => move |entry_row| {
                                sender.input(ClientFormInput::Edited(Field::Tva(entry_row.property("text"))));
                            } @tva_handler,
                        },

                        #[name(tva_icc)]
                        add = &adw::EntryRow {
                            set_title: "TVA intracommunautaire",
                            #[track(model.initializing)]
                            #[block_signal(tva_icc_handler)]
                            set_text: if let Some(tva_icc) = &model.client.tva_icc { tva_icc } else { "" },

                            connect_changed[sender] => move |entry_row| {
                                sender.input(ClientFormInput::Edited(Field::TvaIcc(entry_row.property("text"))));
                            } @tva_icc_handler,
                        },
                    },

                    add = &adw::PreferencesGroup {
                        set_title: "Informations additionnelles",

                        #[name(custom_field)]
                        add = &gtk::TextView {
                            set_height_request: 200,
                            set_wrap_mode: gtk::WrapMode::Word,
                            inline_css: "border-radius: 14px; padding: 10px",

                            #[wrap(Some)]
                            set_buffer = &gtk::TextBuffer {
                                #[track(model.initializing)]
                                set_text: if let Some(custom) = &model.client.custom_field { custom } else { "" },

                                connect_end_user_action[sender] => move |entry_row| {
                                    sender.input(ClientFormInput::Edited(Field::Custom(entry_row.property("text"))));
                                },
                            }
                        },
                    },

                    add = &adw::PreferencesGroup {
                        add = &adw::ButtonRow {
                            #[watch] set_activatable: model.is_valid,
                            #[watch] set_css_classes: match model.is_valid {
                                true => match model.mode {
                                    Mode::Creating => &["suggested-action"],
                                    Mode::Editing => &["destructive-action"],
                                },
                                false => &[""],
                            },
                            #[watch] set_title: match model.mode {
                                Mode::Editing => "Valider les modifications",
                                Mode::Creating => "Ajouter aux clients",
                            },
                            connect_activated => ClientFormInput::Validated,
                        },
                    },
                },
            },
        },

    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {


        let model = ClientFormModel {
            client: Client::default(),
            mode: Mode::Creating,
            is_valid: false,
            has_been_edited: false,
            initializing: false,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ClientFormInput::Edit(client) => {
                self.initializing = true;
                self.has_been_edited = false;
                self.client = client;
                self.mode = Mode::Editing;
            },
            ClientFormInput::Create => {
                self.initializing = true;
                self.has_been_edited = false;
                self.client =  Client::default();
                self.mode = Mode::Creating;
            },
            ClientFormInput::Edited(field) => {
                self.initializing = false; // all entry rows were initialized if necessary
                self.has_been_edited = true;
                match field {
                    Field::Name(value) => self.client.name = value,
                    Field::NumberAndStreet(value) => self.client.address.number_and_street = value,
                    Field::Postcode(value) => self.client.address.postcode = value,
                    Field::City(value) => self.client.address.city = value,
                    Field::Siret(value) => self.client.siret = if value.is_empty() { None } else { Some(value) },
                    Field::Ape(value) => self.client.code_ape = if value.is_empty() { None } else { Some(value) },
                    Field::Tva(value) => self.client.tva = if value.is_empty() { None } else { Some(value) },
                    Field::TvaIcc(value) => self.client.tva_icc = if value.is_empty() { None } else { Some(value) },
                    Field::Custom(value) => self.client.custom_field = if value.is_empty() { None } else { Some(value) },
                };
                self.is_valid = self.client.valid();
            },
            ClientFormInput::Validated => {
                match self.mode {
                    Mode::Creating => sender.output(ClientFormOutput::Created(self.client.clone())).unwrap(),
                    Mode::Editing => sender.output(ClientFormOutput::Edited(self.client.clone())).unwrap(),
                }
                self.initializing = false;
            },
        }
    }
}
