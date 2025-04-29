use adw::prelude::*;
use relm4::prelude::*;
use relm4::factory::FactoryView;

use crate::app::Product;

pub struct Model {
    pub product: Product,
}

#[derive(Debug)]
pub enum ProductInput {
    Edited(Product),
}

#[derive(Debug)]
pub enum ProductOutput {
    Delete(DynamicIndex),
    Edit(DynamicIndex),
}

pub struct ProductInit {}

#[relm4::factory(pub)]
impl FactoryComponent for Model {
    type ParentWidget = adw::PreferencesGroup;
    type Input = ProductInput;
    type Output = ProductOutput;
    type Init = ProductInit;
    type CommandOutput = ();

    view! {
        #[root]
        add = &adw::ActionRow {
            #[watch]
            set_title: &self.product.name,
            #[watch]
            set_subtitle: &self.product.description,

            set_focusable: false,

            add_suffix = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_top: 8,
                set_margin_bottom: 6,
                set_margin_end: 10,
                set_homogeneous: true,

                #[name(cost)]
                append = &gtk::Label {
                    add_css_class: "caption-heading",
                    inline_css: "font-size: medium",
                    #[watch]
                    set_label: &format!{"{} €", self.product.total}.to_string(),
                },

                // #[name(detail)]
                gtk::Label {
                    add_css_class: "caption",
                    add_css_class: "dimmed",
                    inline_css: "font-size: small",
                    set_sensitive: false,
                    // EditableExt::set_alignment: 1.0,
                    #[watch]
                    set_label: &format!{ "{} x {}",
                        self.product.price,
                        self.product.quantity
                    }.to_string(),
                },
            },

            add_suffix = &gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_end: 5,
                set_margin_start: 5,
            },

            add_suffix = &gtk::Button {
                set_tooltip: "Edit",
                set_icon_name: "document-edit-symbolic",
                set_focus_on_click: false,
                set_has_frame: false,
                set_has_tooltip: true,

                add_css_class: "circular",
                set_margin_top: 8,
                set_margin_bottom: 8,

                connect_clicked[sender, index] => move |_| {
                    sender.output(ProductOutput::Edit(index.clone())).unwrap()
                }
            },

            add_suffix = &gtk::Button {
                set_tooltip: "Delete",
                set_icon_name: "user-trash-symbolic",
                add_css_class: "destructive-action",
                set_focus_on_click: false,
                set_has_frame: false,
                set_has_tooltip: true,

                add_css_class: "circular",
                set_margin_top: 8,
                set_margin_bottom: 8,

                connect_clicked[sender, index] => move |_| {
                    sender.output(ProductOutput::Delete(index.clone())).unwrap()
                }
            },
        }
    }

    fn init_model(
        init: Self::Init,
        index: &DynamicIndex,
        sender: FactorySender<Self>,
    ) -> Self {
        Self {
            product: Product {
                name: "".to_owned(),
                description: "".to_owned(),
                price: 0.0,
                quantity: 1,
                total: 0.0,
            }
        }
    }

    fn init_widgets(
        &mut self,
        index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
        match message {
            ProductInput::Edited(product) => {
                println!("Product edited");
                self.product = product;
            },
        };
    }
}
