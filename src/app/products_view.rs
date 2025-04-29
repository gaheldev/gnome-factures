use adw::prelude::*;
use relm4::prelude::*;

mod edit_page;
mod row;
use crate::app::Product;


pub(crate) struct ProductsModel {
    products: FactoryVecDeque<row::Model>,
    edit: Controller<edit_page::Model>,
    current_index: Option<DynamicIndex>,
    show_edit: bool,
}


#[derive(Debug)]
pub(crate) enum ProductsInput {
    Add,
    Delete(DynamicIndex),
    Edit(DynamicIndex),
    Edited(Product),
    HandleEditPageClosingRequest,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for ProductsModel {
    type Init = ();
    type Input = ProductsInput;
    type Output = Vec<Product>;

    view! {
        #[name(navigation_view)]
        adw::NavigationView {
            add = &adw::NavigationPage {
                set_title: "Product list",

                #[wrap(Some)]
                set_child = &adw::PreferencesPage {

                    #[local_ref]
                    add = products_box -> adw::PreferencesGroup {

                        #[watch]
                        set_title: &format!{"Total: {} €",
                            model.products.iter()
                                .map(|row| row.product.total)
                                .sum::<f64>()
                                .abs() // avoid -0
                        }.to_string(),

                        #[wrap(Some)]
                        set_header_suffix = &gtk::Button {
                            set_tooltip: "add product",
                            set_icon_name: "list-add-symbolic",
                            connect_clicked => ProductsInput::Add,
                        },
                    },
                },
            },
            add = model.edit.widget(),
        },
    }

    fn post_view() {
        // TODO: safer mechanism to access headerbox ?
        widgets.products_box
            .first_child().expect("This should be a box, already set by initializing products_box")
            .first_child().expect("In post_view, headerbox should be initialized already by initializing products_box")
            .set_margin_bottom(20);
    }

    fn pre_view() {
        match model.show_edit {
            true => widgets.navigation_view.push(model.edit.widget()),
            false => { widgets.navigation_view.pop(); },
        };
    }

    fn init(
        _params: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let product_rows = FactoryVecDeque::builder()
            .launch(adw::PreferencesGroup::default())
            .forward(sender.input_sender(), |output| match output {
                row::ProductOutput::Delete(index) => ProductsInput::Delete(index),
                row::ProductOutput::Edit(index) => ProductsInput::Edit(index),
            });

        let edit: Controller<edit_page::Model> =
        edit_page::Model::builder()
            .launch(edit_page::Init { product: None })
            .forward(sender.input_sender(), |msg| match msg {
                edit_page::Output::Hiding(product) => ProductsInput::Edited(product),
                edit_page::Output::CloseRequest => ProductsInput::HandleEditPageClosingRequest,
            });

        let model = ProductsModel {
            products: product_rows,
            edit,
            current_index: None,
            show_edit: false,
        };

        let products_box = model.products.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ProductsInput::Add => {
                let mut products_guard = self.products.guard();
                let index = products_guard.push_back(row::ProductInit{});
                sender.input_sender().emit(ProductsInput::Edit(index));
            }
            ProductsInput::Delete(index) => {
                let mut products_guard = self.products.guard();
                products_guard.remove(index.current_index());
            }
            ProductsInput::Edit(index) => {
                self.current_index = Some(index.clone());

                let product = self.products
                    .get(self.current_index.clone().unwrap().current_index())
                    .unwrap().product.clone();

                println!("Edit product {:?}:", index);
                println!("\t{:?}:", product);
                if !self.show_edit {
                    self.edit.sender().emit(edit_page::Input::Edit(product.clone()));
                    self.show_edit = true;
                }
            }
            ProductsInput::Edited(product) => {
                let mut products_guard = self.products.guard();
                let row = products_guard.get_mut(
                    self.current_index
                        .clone()
                        .expect("current index should be set by during ProductsInput::Edit")
                        .current_index()
                );
                // TODO: use Edited message instead?
                row.unwrap().product = product;

                self.current_index = None;
                self.show_edit = false;
            }
            ProductsInput::HandleEditPageClosingRequest => {
                self.show_edit = false;
            }
        }
        sender.output(self.get_products()).unwrap();
    }
}

impl ProductsModel {
    fn get_products(&self) -> Vec<Product> {
        self.products.iter().map(|x| x.product.clone()).collect()
    }
}
