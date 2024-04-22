use crate::champion::Champion;
use crate::item::Item;

pub struct Build {
    pub(crate) champion: Champion,
    pub(crate) items: Vec<Item>,
    // runes: Vec<Rune>,
}

impl Build {
    pub fn new(champion: &Champion, items: Vec<Item>) -> Build {
        Build {
            champion: champion.clone(),
            items,
        }
    }
}