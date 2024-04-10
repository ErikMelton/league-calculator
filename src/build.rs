use crate::champion::Champion;
use crate::item::Item;

pub struct Build {
    champion: Champion,
    items: Vec<Item>,
}