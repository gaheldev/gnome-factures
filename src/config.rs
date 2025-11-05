use crate::app::{Author, Client, ClientName};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

// TODO: add last_devis
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    // TODO: create gui to set output paths
    pub author: Option<Author>,
    pub tex_output_path: String,
    pub pdf_output_path: String,
    pub clients: HashMap<ClientName, Client>,
    pub last_facture: Option<String>,
    pub last_dispense: Option<PathBuf>,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            author: None,
            // TODO: make it configurable
            tex_output_path: "/home/gael/code/tools/compta/facture-gahel/facture.tex".to_string(),
            pdf_output_path: "/home/gael/code/tools/compta/facture-gahel/facture.pdf".to_string(),
            clients: HashMap::new(),
            last_facture: None,
            last_dispense: None,
        }
    }
}

impl Config {
    pub fn load_with_check(app_name: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut cfg: Config = confy::load(app_name, None)?;
        if let Some(dispense) = &cfg.last_dispense {
            if !dispense.exists() {
                cfg.last_dispense = None;
            }
        }

        Ok(cfg)
    }

    pub fn set_clients(&mut self, client_list: Vec<Client>) {
        self.clients = {
            let mut map = HashMap::new();
            for client in client_list {
                map.insert(client.name.clone(), client);
            }
            map
        };
    }
}

// fn vec_to_map(
//     client_list: Vec<Client>,
// ) -> std::collections::HashMap<std::string::String, app::client_form::Client> {
//     let mut map = HashMap::new();
//     for client in client_list {
//         map.insert(client.name, client);
//     }
//     map
// }
