use adw::prelude::*;
use gtk::InputPurpose;
use relm4::prelude::*;

use crate::app::Product;

#[derive(Default,Clone)]
pub struct Model {
    pub product: Product,
    pub editing: bool,
}

#[derive(Debug)]
pub enum Input {
    NameChanged(String),
    DescriptionChanged(String),
    QuantityChanged(u32),
    PriceChanged(Result<f64,()>),
    Edit(Product),
    Shown,
    Hiding,
    Validate,
}

#[derive(Debug)]
pub enum Output {
    CloseRequest,
    Hiding(Product),
}

pub struct Init {
    pub product: Option<Product>,
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = Input;
    type Output = Output;

    view! {
        #[root]
        #[name(navigation_page)]
        adw::NavigationPage {
            set_title: "Edit Product",

            connect_shown => Input::Shown,
            connect_hiding => Input::Hiding,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {

                add_top_bar = &adw::HeaderBar {
                    set_show_end_title_buttons: false,
                    set_margin_top: 22, // align with add button
                },

                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    set_title: "Edit product",

                    add = &adw::PreferencesGroup {
                        #[name(name)]
                        add = &adw::EntryRow {
                            set_title: "Name",

                            #[track(!model.editing)]
                            #[block_signal(name_handler)]
                            set_text: &model.product.name,

                            #[watch] set_css_classes: if model.product.name.is_empty() { &["error"] } else { &[""] },

                            connect_changed[sender] => move |row| {
                                sender.input(Input::NameChanged(row.property("text")));
                            } @name_handler,
                        },

                        // #[name(description)]
                        // add = &adw::EntryRow {
                        //     set_title: "Description",
                        //
                        //     #[track(!model.editing)]
                        //     #[block_signal(description_handler)]
                        //     set_text: &model.product.description,
                        //
                        //     connect_changed[sender] => move |row| {
                        //         sender.input(Input::DescriptionChanged(row.property("text")));
                        //     } @description_handler
                        // },
                    },

                    add = &adw::PreferencesGroup {
                        set_title: "Description",

                        add = &gtk::TextView {
                            set_height_request: 200,
                            set_wrap_mode: gtk::WrapMode::Word,
                            inline_css: "border-radius: 14px; padding: 10px",

                            #[wrap(Some)]
                            set_buffer = &gtk::TextBuffer {
                                // TODO: check that this works without track and block signal
                                // #[track(!model.editing)]
                                // #[block_signal(description_handler)]
                                set_text: &model.product.description,

                                connect_changed[sender] => move |entry| {
                                    sender.input(Input::DescriptionChanged(entry.property("text")));
                                } @description_handler
                            },
                        },
                    },

                    add = &adw::PreferencesGroup {
                        #[name(quantity)]
                        add = &adw::SpinRow {
                            set_title: "Quantity",

                            #[track(!model.editing)]
                            #[block_signal(quantity_handler)]
                            set_adjustment: Some(&gtk::Adjustment::builder()
                                .lower(1.0)
                                .upper(100.0)
                                .value(model.product.quantity.into())
                                .step_increment(1.0)
                                .page_increment(10.0)
                                .build()),

                            connect_changed[sender] => move |row| {
                                sender.input(Input::QuantityChanged(row.value() as u32));
                            } @quantity_handler,
                        },

                        #[name(price)]
                        add = &adw::ActionRow {
                            set_title: "Price",
                            set_focusable: false,

                            add_suffix = &gtk::Box {
                                gtk::Entry {
                                    set_margin_top: 8,
                                    set_margin_bottom: 8,
                                    set_max_width_chars: 11,
                                    set_max_length: 10, // not working
                                    set_input_purpose: InputPurpose::Number,

                                    // disambiguation, because multiple traits in scope currently implement the same method
                                    EntryExt::set_alignment: 1.0,

                                    #[track(!model.editing)]
                                    #[block_signal(price_handler)]
                                    set_buffer: &gtk::EntryBuffer::builder()
                                    .text(model.product.price.to_string())
                                    .max_length(10)
                                    .build(),

                                    // TODO: select all when grabing focus with mouse

                                    // TODO: improve handling so only numbers are possible to enter
                                    connect_text_notify[sender] => move |buffer| {
                                        sender.input(Input::PriceChanged(
                                            match buffer.text().parse::<f64>() {
                                                Ok(value) => Ok(value),
                                                Err(_value) => Err(()),
                                            }
                                        ));
                                    } @price_handler,

                                    // set_buffer = &gtk::EntryBuffer {
                                    //     #[track(!model.editing)]
                                    //     #[block_signal(price_handler)]
                                    //     set_text: model.product.price.to_string(),
                                    //
                                    //     connect_text_notify[sender] => move |buffer| {
                                    //         sender.input(Input::PriceChanged(buffer.text().parse::<f64>().unwrap()));
                                    //     } @price_handler,
                                    // },
                                },

                                gtk::Label {
                                    set_label: "€",
                                    set_margin_start: 10,
                                    set_margin_end: 10,
                                    inline_css: "font-size: x-large",
                                },
                            },
                        },
                    },

                    add = &adw::PreferencesGroup {
                        #[name(validation_button)]
                        add = &gtk::Button {
                            set_label: "Validate",
                            set_halign: gtk::Align::End,

                            add_css_class: "frame",

                            #[watch]
                            set_sensitive: model.requirements_fullfilled(),

                            #[watch]
                            set_class_active: ("accent", model.requirements_fullfilled()),

                            connect_clicked => Input::Validate,

                        },
                    },
                },
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let init_product = match init.product {
            Some(product) => product,
            None => Product {
                name: "".to_owned(),
                description: "".to_owned(),
                price: 0.0,
                quantity: 1,
                total: 0.0,
            },
        };
        let model = Model {
            product: init_product,
            editing: false,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        let total = self.product.price * (self.product.quantity as f64);
        // make sure to use 2 decimal point precision
        self.product.total = (100.0*total).round() / 100.0;
        match message {
            Input::NameChanged(value) => {
                self.product.name = value;
            },
            Input::DescriptionChanged(value) => {
                self.product.description = value;
            },
            Input::QuantityChanged(value) => {
                self.product.quantity = value;
            },
            Input::PriceChanged(value) => {
                if let Ok(value) = value { self.product.price = value };
            },
            Input::Edit(product) => {
                println!("editing {:?}", product);
                self.product = product;
            },
            Input::Hiding => {
                sender.output(Output::Hiding(self.product.clone())).unwrap();
                self.editing = false;
            },
            Input::Shown => {
                self.editing = true;
            },
            Input::Validate => {
                sender.output(Output::CloseRequest).unwrap();
            }
        }
    }
}

impl Model {
    fn requirements_fullfilled(&self) -> bool {
        if self.product.name.is_empty() {
            return false;
        }
        // Price is a valid number: only digits with a single . or ,
        // should be implemented at the source to prevent from entering any wrong character
        // right now it will always be valid but maybe wrong

        // NB: Quantity is always a valid number by widget design

        true
    }
}
