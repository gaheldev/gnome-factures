use adw::prelude::*;
use relm4::prelude::*;

use crate::CFG;


#[derive(Default,Clone)]
pub struct BillingModel {
    bill_type: BillType,
    number: String,
    last_number: Option<String>,
    nature: String,
    diffuseur: bool,
    dispense_file_name: String,
}

#[derive(Debug,Default,Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BillType {
    #[default]
    Facture,
    Devis,
}

#[derive(Debug)]
pub struct BillingInit {
    /// dispense file name
    pub dispense_name: String,
}

#[derive(Debug)]
pub enum BillingInput {
    Type(BillType),
    Diffuseur(bool),
    /// dispense file name
    Dispense(String),
}

#[derive(Debug)]
pub enum BillingOutput {
    Type(BillType),
    Number(String),
    Nature(String),
    Diffuseur(bool),
    PickDispense,
}

#[relm4::component(pub)]
impl SimpleComponent for BillingModel {
    type Init = BillingInit;
    type Input = BillingInput;
    type Output = BillingOutput;

    view! {
        adw::PreferencesPage {

            add = &adw::PreferencesGroup {
                gtk::Box {
                    set_homogeneous: true,
                    add_css_class: "linked",
                    set_orientation: gtk::Orientation::Horizontal,

                    #[name(facture_toggle)]
                    gtk::ToggleButton {
                        set_label: "Facture",
                        set_active: true,
                        connect_toggled[sender] => move |button| {
                            if button.is_active() {
                                sender.input(BillingInput::Type(BillType::Facture));
                            }
                        }
                    },
                    #[name(devis_toggle)]
                    gtk::ToggleButton {
                        set_label: "Devis",
                        set_group: Some(&facture_toggle),
                        connect_toggled[sender] => move |button| {
                            if button.is_active() {
                                sender.input(BillingInput::Type(BillType::Devis));
                            }
                        }
                    },
                },
            },

            add = &adw::PreferencesGroup {
                set_title: "Informations",

                // TODO: for adwaita 1.7
                // gtk::Box {
                //     set_orientation: gtk::Orientation::Horizontal,
                //     set_spacing: 5,
                //     set_margin_all: 5,
                //
                    // adw::ToggleGroup {
                    //     add = adw::Toggle {
                    //         set_label: String("Facture"),
                    //     },
                    //     add = adw::Toggle {
                    //         set_label: String("Devis"),
                    //     },
                    //     connect_active_name_notify[sender] => move |group| {
                    //         match group.active_name() {
                    //             "Facture" => sender.output(BillingOuput::Type(Bill::Facture));
                    //             "Devis" => sender.output(BillingOuput::Type(Bill::Devis));
                    //         }
                    //     }
                    // },

                    // adw::EntryRow {
                    //     set_title: "N°",
                    //     connect_changed[sender] => move |entry_row| {
                    //         let _ = sender.output(BillingOuput::Number(entry_row.property("text")));
                    //     }
                    // },
                // },

                add = &adw::EntryRow {
                    set_title: "N°",
                    set_tooltip: &{
                        if let Some(last) = &model.last_number {
                            format!("dernière facture: {}", last)
                        } else {
                            "utiliser 001 ou <prefix>001".to_string()
                        }
                    },
                    connect_changed[sender] => move |entry_row| {
                        let _ = sender.output(BillingOutput::Number(entry_row.property("text")));
                    },
                },
            },

            add = &adw::PreferencesGroup {
                set_margin_top: 25,
                add = &adw::SwitchRow {
                    set_title: "Contributions Diffuseur",
                    connect_active_notify[sender] => move |switch| {
                        sender.input(BillingInput::Diffuseur(switch.is_active()));
                    }
                },
                add = &adw::ActionRow {
                        set_title: "Dispense de précompte",
                        #[watch]
                        set_subtitle: &model.dispense_file_name,
                        #[watch]
                        set_css_classes: if model.dispense_file_name.is_empty() { &["error"] } else { &[""] },
                        #[watch]
                        set_visible: model.diffuseur,
                        add_suffix = &gtk::Button {
                            set_margin_all: 10,
                            set_icon_name: "document-open-symbolic",
                            connect_clicked[sender] => move |_| sender.output(BillingOutput::PickDispense).unwrap(),
                        }
                },
            },

            add = &adw::PreferencesGroup {
                set_title: "Nature",

                add = &gtk::TextView {
                    set_height_request: 200,
                    set_wrap_mode: gtk::WrapMode::Word,
                    inline_css: "border-radius: 14px; padding: 10px",

                    #[wrap(Some)]
                    set_buffer = &gtk::TextBuffer {
                        connect_changed[sender] => move |entry| {
                            let _ = sender.output(BillingOutput::Nature(entry.property("text")));
                        }
                    }
                },
            },
        }
    }

    fn init(
        params: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = BillingModel {
            dispense_file_name: params.dispense_name,
            last_number: CFG.lock().unwrap().last_facture.clone(),
            ..BillingModel::default()
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            BillingInput::Type(bill_type) => {
                self.bill_type = bill_type.clone();
                sender.output(BillingOutput::Type(bill_type)).unwrap();
            }
            BillingInput::Diffuseur(value) => {
                self.diffuseur = value;
                sender.output(BillingOutput::Diffuseur(self.diffuseur)).unwrap();
            }
            BillingInput::Dispense(filename) => self.dispense_file_name = filename,
        }
    }
}
