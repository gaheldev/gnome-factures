use std::fmt;

use adw::prelude::*;
use relm4::prelude::*;


pub type AuthorName = String;

// TODO: move address to its own file to avoid duplication
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
pub struct Author {
    /// large display name
    pub title: String,
    /// legal name - can be multiline
    pub name: AuthorName,
    pub address: Address,
    pub siret: String,
    pub ape: String,
    pub email: Option<String>,
    pub iban: Option<String>,
    pub signature_file_name: Option<String>,
    pub comptes_a_jour_file_name: Option<String>,
}
impl Author {
    pub(crate) fn valid(&self) -> bool {
        !self.title.is_empty() &&
        !self.name.is_empty()  &&
        self.address.valid()   &&
        !self.siret.is_empty() &&
        !self.ape.is_empty()
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{}", self.name)
    }
}

pub struct AuthorFormModel {
    author: Author,
}

#[derive(Debug)]
pub enum AuthorFormOutput {
    AuthorEdited(Author),
    PickSignature,
    PickComptesAJour,
}

#[derive(Debug)]
pub enum AuthorFormInput {
    TitleEdited(String),
    NameEdited(AuthorName),
    StreetEdited(String),
    PostcodeEdited(String),
    CityEdited(String),
    // CountryEdited(String),
    SiretEdited(String),
    ApeEdited(String),
    EmailEdited(String),
    // TVAEdited(String),
    IbanEdited(String),
    Signature(Option<String>),
    ComptesAJour(Option<String>),
}

/// entry_row!("Name", NameEdited)
macro_rules! entry_row {
    ($name:expr, $signal:expr) => (
        add = &adw::EntryRow {
            set_title: "$name",

            connect_changed[sender] => move |entry_row| {
                sender.input(AuthorFormInput::$signal(entry_row.property("text")));
            }
        }
    )
}

#[relm4::component(pub)]
impl SimpleComponent for AuthorFormModel {
    type Init = Option<Author>;
    type Input = AuthorFormInput;
    type Output = AuthorFormOutput;

    view! {
        adw::PreferencesPage {

            #[name(edit_group)]
            add = &adw::PreferencesGroup {
                add = &adw::EntryRow {
                    set_title: "Nom *",
                    set_text: &model.author.title,
                    #[watch] set_css_classes: if model.author.title.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(AuthorFormInput::TitleEdited(entry_row.property("text")));
                    }
                },
                add = &adw::EntryRow {
                    set_title: "Raison Sociale *",
                    set_text: &model.author.name,
                    #[watch] set_css_classes: if model.author.name.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(AuthorFormInput::NameEdited(entry_row.property("text")));
                    }
                },
                add = &adw::EntryRow {
                    set_title: "Rue *",
                    set_text: &model.author.address.number_and_street,
                    #[watch] set_css_classes: if model.author.address.number_and_street.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(AuthorFormInput::StreetEdited(entry_row.property("text")));
                    }
                },
                add = &adw::EntryRow {
                    set_title: "Code Postal *",
                    set_text: &model.author.address.postcode,
                    #[watch] set_css_classes: if model.author.address.postcode.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(AuthorFormInput::PostcodeEdited(entry_row.property("text")));
                    }
                },
                add = &adw::EntryRow {
                    set_title: "Ville *",
                    set_text: &model.author.address.city,
                    #[watch] set_css_classes: if model.author.address.city.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(AuthorFormInput::CityEdited(entry_row.property("text")));
                    }
                },
            },
            add = &adw::PreferencesGroup {
                add = &adw::EntryRow {
                    set_title: "SIRET *",
                    set_text: &model.author.siret,
                    #[watch] set_css_classes: if model.author.siret.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(AuthorFormInput::SiretEdited(entry_row.property("text")));
                    }
                },
                add = &adw::EntryRow {
                    set_title: "Code APE *",
                    set_text: &model.author.ape,
                    #[watch] set_css_classes: if model.author.ape.is_empty() { &["error"] } else { &[""] },

                    connect_changed[sender] => move |entry_row| {
                        sender.input(AuthorFormInput::ApeEdited(entry_row.property("text")));
                    }
                },
                add = &adw::EntryRow {
                    set_title: "Mail",
                    set_text: &model.author.email.clone().unwrap_or_default(),
                    connect_changed[sender] => move |entry_row| {
                        sender.input(AuthorFormInput::EmailEdited(entry_row.property("text")));
                    }
                },
                add = &adw::EntryRow {
                    set_title: "IBAN",
                    set_text: &model.author.iban.clone().unwrap_or_default(),
                    connect_changed[sender] => move |entry_row| {
                        sender.input(AuthorFormInput::IbanEdited(entry_row.property("text")));
                    }
                },
            },
            add = &adw::PreferencesGroup {
                add = &adw::ActionRow {
                        set_title: "Signature",
                        #[watch]
                        set_subtitle: if let Some(path) = &model.author.signature_file_name { path } else { "" },
                        // #[watch]
                        // set_css_classes: if model.author.signature_file_name.is_some() { &["error"] } else { &[""] },
                        add_suffix = &gtk::Button {
                            set_margin_all: 10,
                            set_icon_name: "document-open-symbolic",
                            connect_clicked[sender] => move |_| sender.output(AuthorFormOutput::PickSignature).unwrap(),
                        }
                },

                add = &adw::ActionRow {
                        set_title: "Attestation de Comptes à Jour",
                        #[watch]
                        set_subtitle: if let Some(path) = &model.author.comptes_a_jour_file_name { path } else { "" },
                        // #[watch]
                        // set_css_classes: if model.author.comptes_a_jour_file_name.is_some() { &["error"] } else { &[""] },
                        add_suffix = &gtk::Button {
                            set_margin_all: 10,
                            set_icon_name: "document-open-symbolic",
                            connect_clicked[sender] => move |_| sender.output(AuthorFormOutput::PickComptesAJour).unwrap(),
                        }
                },
            },
        },
    }

    fn init(
        author: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let model = AuthorFormModel {
            author: author.unwrap_or_default(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AuthorFormInput::TitleEdited(value) => self.author.title = value,
            AuthorFormInput::NameEdited(value) => self.author.name = value,
            AuthorFormInput::StreetEdited(value) => self.author.address.number_and_street = value,
            AuthorFormInput::PostcodeEdited(value) => self.author.address.postcode = value,
            AuthorFormInput::CityEdited(value) => self.author.address.city = value,
            AuthorFormInput::SiretEdited(value) => self.author.siret = value,
            AuthorFormInput::ApeEdited(value) => self.author.ape = value,
            AuthorFormInput::EmailEdited(value) => {
                self.author.email = if value.is_empty() { None } else {Some(value) }
            }
            AuthorFormInput::IbanEdited(value) => {
                self.author.iban = if value.is_empty() { None } else { Some(value) }
            }
            AuthorFormInput::Signature(signature) => self.author.signature_file_name = signature,
            AuthorFormInput::ComptesAJour(comptes_a_jour) => self.author.comptes_a_jour_file_name = comptes_a_jour,
        }
        sender.output(AuthorFormOutput::AuthorEdited(self.author.clone())).unwrap();
    }
}
