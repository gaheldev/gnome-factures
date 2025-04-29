use gtk::prelude::*;
use poppler::Document;
use std::rc::Rc;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

// Model to store PDF viewer state
pub struct PdfViewerModel {
    document: Option<Rc<Document>>,
    scale: f64,
    initial_zoom: f64,
}

// Messages our component will handle
#[derive(Debug)]
pub enum PdfViewerMsg {
    LoadPdf(String),
    StartZoom,
    Zoom(f64),
}

#[relm4::component(pub)]
impl SimpleComponent for PdfViewerModel {
    type Init = ();
    type Input = PdfViewerMsg;
    type Output = ();
    
    view! {
        #[root]
        gtk::Box {
            gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,

                #[name(area)]
                gtk::DrawingArea {
                    // FIXME: always zooms with respect to (0,0)
                    // Add gesture for pinch-to-zoom
                    add_controller = gtk::GestureZoom {
                        connect_begin[sender] => move |_,_| {
                            sender.input(PdfViewerMsg::StartZoom)
                        },
                        connect_scale_changed[sender] => move |_gesture, scale| {
                            sender.input(PdfViewerMsg::Zoom(scale));
                        },
                    },
                }
            }
        }
    }

    fn post_view() {
        let document = model.document.clone();
        let scale = model.scale;

        area.set_draw_func(move |_, cr, width, _height| {
            // Get document reference
            if let Some(doc) = &document {
                let n_pages = doc.n_pages();
                let mut page_offset = 0.0;

                // Save cairo state before applying transformations
                cr.save().expect("Failed to save Cairo state");

                cr.scale(scale, scale);

                for current_page in 0..n_pages {
                    // Get the page
                    if let Some(page) = doc.page(current_page) {
                        // Get page dimensions
                        let (w, _h) = page.size();

                        // FIXME: we should calculate all pages width in a first pass to center for
                        // any number of pages, works fine for now
                        // center first page
                        if n_pages == 1 {
                            page_offset = (width as f64 - w * scale) / 2.0;
                        }

                        cr.translate(page_offset * scale, 0.0);

                        // Render the page
                        page.render(cr);

                        // y offset for next page
                        page_offset = w / scale;
                    }
                }

                // Restore Cairo state
                cr.restore().expect("Failed to restore Cairo state");

            }
        });

        // Update drawing area size based on document and scale
        if let Some(doc) = &model.document {
            let mut width = 0.0;
            let mut height = 0.0;
            for current_page in 0..doc.n_pages() {
                if let Some(page) = doc.page(current_page) {
                    let (w, h) = page.size();
                    height = h;
                    width += w;
                }
            }
            area.set_content_width((width * scale) as i32);
            area.set_content_height((height * scale) as i32);
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>
    ) -> ComponentParts<Self> {

        // Create model
        let model = PdfViewerModel {
            document: None,
            scale: 1.0,
            initial_zoom: 1.0,
        };

        let widgets = view_output!();
        
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PdfViewerMsg::LoadPdf(path) => {
                match Document::from_file(&uri(&path), None) {
                    Ok(doc) => self.document = Some(Rc::new(doc)),
                    Err(err) => println!("Pdf couldn't be loaded: {}", err),
                }
            }
            PdfViewerMsg::StartZoom => {
                self.initial_zoom = self.scale;
            }
            PdfViewerMsg::Zoom(scale) => {
                self.scale = self.initial_zoom * scale;
            }
        }
    }
}

fn uri(absolute_path: &str) -> String {
    format!("file://{}", &absolute_path)
}
