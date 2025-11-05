use adw::prelude::*;
use relm4::prelude::*;

use crate::app::client_selector_group::{ClientSelectorGroupModel, ClientSelectorGroupInput, ClientSelectorGroupOutput};
use crate::app::{ClientFormModel, ClientFormOutput};
use crate::app::Client;

use super::client_form::ClientFormInput;


pub(crate) struct ClientViewModel {
    clients: Vec<Client>,
    client_selector: Controller<ClientSelectorGroupModel>,
    client_form: Controller<ClientFormModel>,
    current_client: Option<Client>,
    show_edit: bool,
}

#[derive(Debug)]
pub(crate) enum ClientViewOutput {
    ClientListEdited(Vec<Client>),
    Selected(Client),
}

#[derive(Debug)]
pub(crate) enum ClientViewInput {
    Create,
    Edit,
    ClientListEdited(Vec<Client>),
    Selected(Client),
    Edited(Client),
    Created(Client),
    ClosedForm,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for ClientViewModel {
    type Init = Vec<Client>;
    type Input = ClientViewInput;
    type Output = ClientViewOutput;

    view! {
        #[name(navigation_view)]
        adw::NavigationView {
            #[name(selection_page)]
            add = &adw::NavigationPage {
                set_title: "Selection du client",

                #[wrap(Some)]
                set_child = &adw::PreferencesPage {
                    add = model.client_selector.widget(),
                },
            },
        },
    }

    fn pre_view() {
        match model.show_edit {
            true => widgets.navigation_view.push(model.client_form.widget()),
            false => { widgets.navigation_view.pop(); },
        };
    }

    fn init(
        clients: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let client_selector = ClientSelectorGroupModel::builder()
            .launch(clients.clone())
            .forward(sender.input_sender(), |output| match output {
                ClientSelectorGroupOutput::ClientListEdited(client_list) => ClientViewInput::ClientListEdited(client_list),
                ClientSelectorGroupOutput::Selected(client) => ClientViewInput::Selected(client),
                ClientSelectorGroupOutput::Edit => ClientViewInput::Edit,
                ClientSelectorGroupOutput::Create => ClientViewInput::Create,
            });

        let client_form: Controller<ClientFormModel> =
        ClientFormModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                ClientFormOutput::Hiding => ClientViewInput::ClosedForm,
                ClientFormOutput::Edited(client) => ClientViewInput::Edited(client),
                ClientFormOutput::Created(client) => ClientViewInput::Created(client),
            });

        let model = ClientViewModel {
            clients,
            client_selector,
            client_form,
            current_client: None,
            show_edit: false,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ClientViewInput::ClosedForm => {
                self.show_edit = false;
            },
            ClientViewInput::ClientListEdited(client_list) => {
                // FIXME: use a singleton instead?
                self.clients = client_list.clone();
                sender.output(ClientViewOutput::ClientListEdited(client_list)).unwrap();
            },
            ClientViewInput::Edited(client) => {
                self.show_edit = false;
                self.client_selector.sender().emit(ClientSelectorGroupInput::Edited(client));
            },
            ClientViewInput::Created(client) => {
                self.show_edit = false;
                self.client_selector.sender().emit(ClientSelectorGroupInput::Created(client));
            },
            ClientViewInput::Selected(client) => {
                self.current_client = Some(client.clone());
                sender.output(ClientViewOutput::Selected(client)).unwrap();
            },
            ClientViewInput::Create => {
                self.show_edit = true;
                self.client_form.emit(ClientFormInput::Create);
            },
            ClientViewInput::Edit => {
                if let Some(client) = self.current_client.clone() {
                    self.show_edit = true;
                    self.client_form.emit(ClientFormInput::Edit(client));
                };
            },
        }
    }
}

