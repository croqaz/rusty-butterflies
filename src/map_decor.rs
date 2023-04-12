use crate::map_common::*;

/// A chest item from the map
///
#[derive(PartialEq, Eq)]
pub struct Chest {
    pub t: AbstractThing,
    pub open: bool,
    pub has_net: bool,
}

impl Default for Chest {
    fn default() -> Self {
        Self {
            t: AbstractThing {
                ch: '▣', // Black square unicode
                name: "Chest".into(),
                // description: "A wooden chest"
            },
            open: false,
            has_net: false,
        }
    }
}

impl Interaction for Chest {
    fn can_interact(&self) -> bool {
        !self.open
    }
    fn interact(&mut self) {
        self.open = true;
        self.t.ch = '▨'; // Square with fill
    }
}
