use std::fmt::{self, Display};

#[derive(Debug,Default,Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub price: f64,
    pub total: f64,
}

impl Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {} x {} € = {} €\n - {}",
            self.name,
            self.quantity,
            self.price,
            self.total,
            self.description,
        )
    }
}
