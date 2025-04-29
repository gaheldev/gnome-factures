use relm4::prelude::*;

mod app;
mod latex;
mod config;


const APP_NAME: &str = "gnome-factures";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::load_with_check(APP_NAME)?;

    // create app
    adw::init().expect("Failed to initialize libadwaita");

    // css for custom close button
    relm4::set_global_css(
        "

        .transparent-header {
            background-color: transparent;
            box-shadow: none;
            border-bottom: none;
        }
            
        "
    );

    let app = RelmApp::new(APP_NAME);

    // run app main loop
    app.run::<app::AppModel>(cfg);

    Ok(())
}
