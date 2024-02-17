#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Item {
    pub name: String,
    pub price: usize,
}

impl Item {
    pub fn new(name: &str, price: usize) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }
}
