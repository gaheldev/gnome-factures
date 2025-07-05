use adw::prelude::*;
use relm4::prelude::*;
use relm4_components::simple_combo_box::{SimpleComboBox, SimpleComboBoxMsg};

use crate::app::Client;

pub(crate) struct EditableComboRow {
    list: Vec<Client>,
    selected: Option<usize>,
    combo_box: Controller<SimpleComboBox<Client>>,
}

#[derive(Debug)]
pub(crate) enum EditableComboOutput {
    Selected(Client),
    Edit(Client),
    Add,
}

#[derive(Debug)]
pub(crate) enum EditableComboInput {
    Selected(usize), // FIXME: this one shouldn't be public
    EditRequest, // FIXME: this one shouldn't be public
    Edited(Client),
    Add(Client),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for EditableComboRow {
    type Init = Vec<Client>;
    type Input = EditableComboInput;
    type Output = EditableComboOutput;

    view! {
        #[root]
        adw::ActionRow {
            // set_title: "Client",
            // #[watch]
            // set_subtitle: &{
            //     if let Some(index) = model.selected {
            //         format!("{:?}", &model.list[index])
            //     } else {
            //         "".to_string()
            //     }
            // },

            add_suffix = model.combo_box.widget(),

            add_suffix = &gtk::Separator {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_end: 5,
                set_margin_start: 5,
            },

            add_suffix = &gtk::Button {
                #[watch]
                set_visible: !model.list.is_empty(),
                set_tooltip: "Editer",
                set_icon_name: "document-edit-symbolic",
                set_focus_on_click: false,
                set_has_frame: false,
                set_has_tooltip: true,

                add_css_class: "circular",
                set_margin_top: 8,
                set_margin_bottom: 8,

                connect_clicked[sender] => move |_| {
                    sender.input(EditableComboInput::EditRequest)
                }
            },

            add_suffix = &gtk::Button {
                set_tooltip: "Nouveau",
                set_icon_name: "list-add_symbolic",
                // add_css_class: "destructive-action",
                set_focus_on_click: false,
                set_has_frame: false,
                set_has_tooltip: true,

                add_css_class: "circular",
                set_margin_top: 8,
                set_margin_bottom: 8,

                connect_clicked[sender] => move |_| {
                    sender.output(EditableComboOutput::Add).unwrap()
                }
            },
        }
    }

    fn init(
        list: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let combo_box: Controller<SimpleComboBox<Client>> =
        SimpleComboBox::builder()
            .launch(SimpleComboBox {
                variants: list.clone(),
                active_index: None
            })
            .forward(sender.input_sender(), EditableComboInput::Selected);

        let model = EditableComboRow {
            list,
            selected: None,
            combo_box,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            EditableComboInput::EditRequest => {
                if let Some(index) = self.selected {
                    sender.output(EditableComboOutput::Edit(self.list[index].clone())).unwrap();
                }
            },
            EditableComboInput::Selected(usize) => {
                self.selected = Some(usize);
                sender.output(EditableComboOutput::Selected(self.list[usize].clone())).unwrap();
            },
            EditableComboInput::Edited(client) => {
                // update selected client
                self.list[self.selected.unwrap()] = client;
                self.update_combo();
            },
            EditableComboInput::Add(client) => {
                // add client to list and select it
                self.list.push(client);
                self.selected = Some(self.list.len());
                self.update_combo();
            },
        }
    }
}

impl EditableComboRow {
    fn update_combo(&self) {
        let updated_combo = SimpleComboBox {
            variants: self.list.clone(),
            active_index: self.selected,
        };

        self.combo_box
            .sender()
            .send(SimpleComboBoxMsg::UpdateData(updated_combo))
            .unwrap();
    }
}
