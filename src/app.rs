use std::{fmt::{self, Display}, path::PathBuf};
use std::time::{Duration, Instant};
use chrono::Local;

use adw::prelude::*;
use billing_view::BillingInit;
use gtk::gio::Cancellable;
use relm4::{prelude::*, MessageBroker};

mod author_view;
mod product;
mod client_form;
mod products_view;
mod billing_view;
mod pdf_viewer;

pub use author_view::{Author, AuthorFormOutput, AuthorFormModel};
pub use product::Product;
use client_form::{ClientFormModel, ClientFormOutput};
pub use client_form::{Client, ClientName};
use products_view::ProductsModel;
pub use billing_view::{BillType, BillingModel, BillingOutput, BillingInput};
use crate::{latex::{InvoiceData, PdfFile, Template}, APP_NAME};
use pdf_viewer::{PdfViewerModel, PdfViewerMsg};


static DIALOG_BROKER: MessageBroker<BillingInput> = MessageBroker::new();

#[derive(Debug)]
pub(crate) enum AppMsg {
    AuthorEdited(Author),
    BillTypeChanged(BillType),
    BillNumberChanged(String),
    BillNature(String),
    Diffuseur(bool),
    DispenseSelected(Option<PathBuf>),
    PickDispense,
    ClientEdited(Client),
    ProductsEdited(Vec<Product>),

    OpenPdf,
    ShowPdf,
    HidePdf,
    Export,
    PdfCompiled(PdfFile),
    CompilationError(String),

    ResetShowDispenseDialog,
    /// does nothing
    Null,
}

#[derive(PartialEq, Eq)]
enum UpToDate {
    None,
    Pdf,
    All,
}

pub(crate) struct AppModel {
    app_config: crate::config::Config,
    max_parallel_jobs: u64,
    compile_count: u64,
    last_compilation: Instant,
    compile_cooldown: Duration,
    pending_compilation: bool,

    author_view: Controller<AuthorFormModel>,
    client_form: Controller<ClientFormModel>,
    products_view: Controller<ProductsModel>,
    billing_view: Controller<BillingModel>,
    pdf_viewer: Controller<PdfViewerModel>,
    is_form_valid: bool,
    status: UpToDate,
    show_dispense_dialog: bool,
    show_pdf: bool,
    pdf: Option<PdfFile>,

    pub(crate) author: Author,
    pub(crate) bill_type: BillType,
    pub(crate) number: String,
    pub(crate) client: Client,
    pub(crate) nature: String,
    pub(crate) diffuseur: bool,
    pub(crate) dispense: Option<PathBuf>,
    pub(crate) products: Vec<Product>,
}

impl Display for AppModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} (diffuseur={})\nfor {:?}\nnature: {}\n\nproducts:\n\t{:?}", self.bill_type, self.number, self.diffuseur, self.client, self.nature, self.products)
    }
}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = crate::config::Config;

    view! {
        #[root]
        #[name(window)]
        adw::Window {
            set_title: Some("Facture"),
            // FIXME: use request size to support different dpis
            set_default_width: 1300,
            set_default_height: 700,

            #[wrap(Some)]
            set_content = &gtk::Overlay {
                
                add_overlay = &adw::HeaderBar {
                    #[watch] set_visible: model.show_pdf,
                    set_show_title: false,
                    add_css_class: "transparent-header",
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::Start,
                    set_decoration_layout: Some(":close"),
                },

                add_overlay = &gtk::Box {
                    set_halign: gtk::Align::End,
                    set_margin_all: 10,
                    set_valign: gtk::Align::End,
                    add_css_class: "linked",

                    append = &gtk::Button {
                        set_label: "Export",
                        add_css_class: "pill",
                        // add_css_class: "opaque",
                        #[watch] set_class_active: ("accent", model.is_form_valid && model.status != UpToDate::All),
                        #[watch] set_class_active: ("success", model.is_form_valid && model.status == UpToDate::All),
                        #[watch] set_sensitive: model.is_form_valid,
                        connect_clicked => AppMsg::Export,
                    },
                },

                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,

                    append = &adw::ToolbarView {
                        #[name(stack)]
                        #[wrap(Some)]
                        set_content = &adw::ViewStack {
                            add_titled_with_icon[Some("author"), "Auteur", "avatar-default-symbolic"] = model.author_view.widget(),
                            add_titled_with_icon[Some("bill"), "Facture", "document-edit-symbolic"] = model.billing_view.widget(),
                            add_titled_with_icon[Some("client"), "Client", "user-info-symbolic"] = model.client_form.widget(),
                            add_titled_with_icon[Some("products"), "Produits", "view-list-bullet-symbolic"] = model.products_view.widget(),
                        },


                        add_top_bar = &adw::HeaderBar {
                            #[watch]
                            set_decoration_layout: Some(if model.show_pdf { "" } else { ":close" }),
                            inline_css: " background-color: transparent; ",

                            #[name(switcher)]
                            #[wrap(Some)]
                            set_title_widget = &adw::ViewSwitcher {
                                set_policy: adw::ViewSwitcherPolicy::Narrow,
                                set_stack:  Some(&stack),
                            },

                            pack_end = &gtk::Box {
                                add_css_class: "linked",

                                append = &gtk::ToggleButton {
                                    set_icon_name: "view-dual-symbolic",
                                    set_tooltip: "Aperçu pdf",
                                    // add_css_class: "frame",
                                    #[block_signal(splitview_handler)] #[watch] set_active: model.show_pdf,
                                    connect_toggled[sender] => move |button| {
                                        sender.input(match button.is_active() {
                                            true => AppMsg::ShowPdf,
                                            false => AppMsg::HidePdf,
                                        });
                                    } @splitview_handler,
                                },
                                append = &gtk::Button {
                                    // set_label: "View",
                                    set_icon_name: "view-paged-rtl-symbolic",
                                    set_tooltip: "Ouvrir le pdf",
                                    // add_css_class: "frame",
                                    #[watch] set_sensitive: model.is_form_valid,
                                    connect_clicked => AppMsg::OpenPdf,
                                },
                            },
                        },

                    },

                    append = &gtk::Box {
                        #[watch]
                        set_visible: model.show_pdf,
                        set_width_request: 600,
                        set_height_request: 855,

                        append = &gtk::Separator,
                        append = model.pdf_viewer.widget(),
                    },
                },

            },

        }
    }

    fn post_view() {
        // TODO: make dialog a component in its own file
        if model.show_dispense_dialog {
            sender.input(AppMsg::ResetShowDispenseDialog);
            let dialog = gtk::FileDialog::builder()
                .title("Pick dispense file")
                .modal(true)
                .build();

            dialog.open(Some(&widgets.window),
                Some(&Cancellable::new()),
                move |file| {
                    sender.input(AppMsg::DispenseSelected(
                        match file {
                            Ok(gtk_file) => Some(gtk_file.path().unwrap()),
                            Err(_) => None,
                    }
                    ));
                },
            );
        }
    }

    /// Initialize the UI and model.
    fn init(
        cfg: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {

        let author_view: Controller<AuthorFormModel> =
        AuthorFormModel::builder()
            .launch(cfg.author.clone())
            .forward(sender.input_sender(), |msg| match msg {
                AuthorFormOutput::AuthorEdited(author) => AppMsg::AuthorEdited(author),
            });

        let billing_init = BillingInit {
            dispense_name: match &cfg.last_dispense {
                Some(file_path) => file_path.to_str().unwrap().to_string(),
                None => "".to_string(),
            }
        };

        let billing_view: Controller<BillingModel> =
        BillingModel::builder()
            .launch_with_broker(billing_init, &DIALOG_BROKER)
            .forward(sender.input_sender(), |msg| match msg {
                BillingOutput::Type(bill_type) => AppMsg::BillTypeChanged(bill_type),
                BillingOutput::Number(number) => AppMsg::BillNumberChanged(number),
                BillingOutput::Nature(nature) => AppMsg::BillNature(nature),
                BillingOutput::Diffuseur(is_diffuseur) => AppMsg::Diffuseur(is_diffuseur),
                BillingOutput::PickDispense => AppMsg::PickDispense,
            });

        let client_form: Controller<ClientFormModel> =
        ClientFormModel::builder()
            .launch(cfg.clients.values().cloned().collect())
            .forward(sender.input_sender(), |msg| match msg {
                ClientFormOutput::ClientEdited(client) => AppMsg::ClientEdited(client),
            });

        let products_view: Controller<ProductsModel> =
        ProductsModel::builder()
            .launch(())
            .forward(sender.input_sender(), AppMsg::ProductsEdited);

        let pdf_viewer = PdfViewerModel::builder()
            .launch(())
            .forward(sender.input_sender(), |_| { AppMsg::Null });

        let model = AppModel {
            app_config: cfg.clone(),
            // TODO: make it configurable
            max_parallel_jobs: 5,
            compile_count: 0,
            last_compilation: Instant::now(),
            // TODO: make it configurable
            compile_cooldown: Duration::from_millis(100),
            pending_compilation: false,

            author_view,
            billing_view,
            client_form,
            products_view,
            pdf_viewer,
            is_form_valid: false,
            status: UpToDate::None,
            show_dispense_dialog: false,
            show_pdf: true,
            pdf: None,

            author: cfg.author.unwrap_or_default(),
            bill_type: BillType::Facture,
            // TODO: use cfg number
            number: "000".to_string(),
            diffuseur: false,
            dispense: cfg.last_dispense,
            nature: "".to_string(),
            client: Client::default(),
            products: Vec::new(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::AuthorEdited(author) => {
                self.status = UpToDate::None;
                self.author = author;
                self.app_config.author = Some(self.author.clone());
                confy::store(APP_NAME, None, self.app_config.clone()).unwrap();
            }
            AppMsg::ClientEdited(client) => {
                self.status = UpToDate::None;
                self.client = client;
            }
            AppMsg::BillTypeChanged(bill_type) => {
                self.status = UpToDate::None;
                self.bill_type = bill_type;
            }
            AppMsg::BillNumberChanged(number) => {
                self.status = UpToDate::None;
                self.number = number;
            }
            AppMsg::BillNature(nature) => {
                self.status = UpToDate::None;
                self.nature = nature;
            }
            AppMsg::Diffuseur(is_diffuseur) => {
                self.status = UpToDate::None;
                self.diffuseur = is_diffuseur;
            }
            AppMsg::DispenseSelected(filepath) => {
                self.status = UpToDate::None;
                self.dispense = filepath;
                self.app_config.last_dispense = self.dispense.clone();
                confy::store(APP_NAME, None, self.app_config.clone()).unwrap();

                let filename = match &self.dispense {
                    Some(filepath) => filepath.to_str().unwrap().to_string(),
                    None => "".to_string(),
                };
                DIALOG_BROKER.send(BillingInput::Dispense(filename));
            }
            AppMsg::PickDispense => {
                self.show_dispense_dialog = true;
            }
            AppMsg::ResetShowDispenseDialog => {
                self.show_dispense_dialog = false;
            }
            AppMsg::ProductsEdited(products) => {
                self.status = UpToDate::None;
                self.products = products;
            }
            AppMsg::Export => {
                // add client to the list of clients
                self.app_config.clients.insert(self.client.name.clone(), self.client.clone());
                confy::store(APP_NAME, None, self.app_config.clone()).unwrap();

                // In case the pdf wasn't already compiled in background
                if let UpToDate::None = self.status {
                    self.pdf = Some(Template::new()
                        .fill(self.invoice())
                        .expect("Error filling template")
                        .to_file(&self.app_config.tex_output_path)
                        .expect("Error writing tex file")
                        .export(&self.app_config.pdf_output_path)
                        .expect("Error exporting pdf file")
                    );
                    self.status = UpToDate::Pdf;
                }
                if let UpToDate::Pdf = self.status {
                    self.pdf.as_ref()
                        .unwrap()
                        .export(&self.app_config.pdf_output_path)
                        .expect("Error copying pdf file");
                    Template::new()
                        .fill(self.invoice())
                        .expect("Error filling template")
                        .to_file(&self.app_config.tex_output_path)
                        .expect("Error exporting tex file");
                    self.status = UpToDate::All;
                }
            }
            AppMsg::OpenPdf => {
                // In case the pdf wasn't already compiled in background
                if let UpToDate::None = self.status {
                    self.pdf = Some(Template::new()
                        .fill(self.invoice())
                        .expect("Error filling template")
                        .compile()
                        .expect("Error compiling tmp pdf file")
                    );
                    self.status = UpToDate::Pdf;
                }
                self.pdf.as_ref()
                    .expect("tmp pdf file should be compiled")
                    .open().expect("Error opening tmp pdf file");

                self.pdf_viewer.sender().emit(
                    PdfViewerMsg::LoadPdf(
                        self.pdf.clone()
                            .expect("Pdf should exist when form is valid")
                            .path
                    )
                );
            }
            AppMsg::ShowPdf => self.show_pdf = true,
            AppMsg::HidePdf => self.show_pdf = false,
            AppMsg::Null => (),

            AppMsg::PdfCompiled(pdf_file) => {
                self.pdf = Some(pdf_file);
                self.status = UpToDate::Pdf;
                self.compile_count -= 1;
                
                if let Some(pdf) = &self.pdf {
                    self.pdf_viewer.sender().emit(
                        PdfViewerMsg::LoadPdf(pdf.path.clone())
                    );
                }
            },

            AppMsg::CompilationError(error_msg) => {
                self.compile_count -= 1;
                println!("PDF compilation error: {}", error_msg);
            },
        }

        self.is_form_valid = self.form_valid();

        if self.is_form_valid && self.show_pdf {
            if self.status == UpToDate::None {
                self.queue_compilation(sender.clone());
            } else if let Some(pdf) = &self.pdf {
                self.pdf_viewer.sender().emit(
                    PdfViewerMsg::LoadPdf(pdf.path.clone())
                );
            }

            if self.pending_compilation {
                self.queue_compilation(sender.clone());
            }
        }
    }
}

impl AppModel {
    fn form_valid(&self) -> bool {
        if !self.author.valid() { return false; }
        if !self.client.valid() { return false; }
        if self.products.is_empty() { return false; }
        if self.products.iter().any(|prod| prod.name.is_empty()) { return false; }
        if self.diffuseur && self.dispense.is_none() { return false; }
        true
    }

    fn invoice(&self) -> InvoiceData {
        InvoiceData {
            author: self.author.clone(),
            is_devis: matches!(self.bill_type, BillType::Devis),
            number: self.number.clone(),
            client: self.client.clone(),
            nature: self.nature.clone(),
            diffuseur: self.diffuseur,
            dispense_path: match &self.dispense {
                Some(path_buf) => path_buf.to_str().unwrap().to_string(),
                None => "".to_string(),
            },
            products: self.products.clone(),
            date: Local::now().date_naive().format("%d/%m/%Y").to_string(),
        }
    }

    fn queue_compilation(&mut self, sender: ComponentSender<Self>) {
        dbg!(self.last_compilation.elapsed().as_secs());
        dbg!(self.compile_cooldown.as_secs());
        if self.compile_count < self.max_parallel_jobs && self.last_compilation.elapsed() > self.compile_cooldown {
            self.compile_count += 1;
            self.pending_compilation = false;
            self.last_compilation = Instant::now();
            self.compile_pdf_in_background(sender);
        } else {
            self.pending_compilation = true;
        }
    }

    // FIXME: queue compile orders and only run the last one of the queue
    //        once the previous one is finished
    fn compile_pdf_in_background(&self, sender: ComponentSender<Self>) {
        let invoice_data = self.invoice();

        std::thread::spawn(move || {
            match Template::new()
                .fill(invoice_data)
                .and_then(|template| template.compile()) {
                    Ok(pdf_file) => sender.input(AppMsg::PdfCompiled(pdf_file)),
                    Err(err) => sender.input(AppMsg::CompilationError(err.to_string())),
                }
        });
    }
}
