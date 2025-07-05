use std::fmt;

#[derive(Debug,Clone,Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Address {
    pub number_and_street: String,
    pub postcode: String,
    pub city: String,
    // country: String,
}
impl Address {
    pub(crate) fn valid(&self) -> bool {
        if self.number_and_street.is_empty() { return false; }
        if self.postcode.is_empty() { return false; }
        if self.city.is_empty() { return false; }
        true
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{}\n{} {}", self.number_and_street, self.postcode, self.city)
    }
}
