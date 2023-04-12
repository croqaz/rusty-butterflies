use crate::map_common::*;

///
/// Player struct
///
#[derive(PartialEq, Eq)]
pub struct Player {
    pub t: AbstractThing,
    pub group: Vec<Group>,
}

///
/// Butterfly
///
#[derive(PartialEq, Eq)]
pub struct Butterfly {
    pub t: AbstractThing,
    pub group: Vec<Group>,
}

impl Default for Butterfly {
    fn default() -> Self {
        Self {
            t: AbstractThing {
                ch: 'B',
                name: "Butterfly".into(),
                // description: "A white butterfly"
            },
            group: vec![Group::Insect],
        }
    }
}

// impl Movement for Butterfly {
//     fn can_move(&self) -> bool {
//         true
//     }
//     fn try_move(&mut self) {
//         // check if there are items or creatures on the map first!
//         // if the map position is empty
//     }
// }

impl Behavior for Butterfly {
    fn can_behave(&self) -> bool {
        true
    }
    fn behave(&mut self) {
        // behave like a butterfly, eg: wander
        self.t.ch = '🦋';
    }
}

///
/// Cat meow meow
///
#[derive(PartialEq, Eq)]
pub struct Cat {
    pub t: AbstractThing,
    pub group: Vec<Group>,
}

impl Default for Cat {
    fn default() -> Self {
        Self {
            t: AbstractThing {
                ch: 'C',
                name: "Cat Mittens".into(),
                // description: "A black cat"
            },
            group: vec![Group::Pet],
        }
    }
}

// impl Movement for Cat {
//     fn can_move(&self) -> bool {
//         true
//     }
//     fn try_move(&mut self, p: &Point) {
//         // check if there are items or creatures on the map first!
//         // if the map position is empty, them:
//     }
// }

impl Behavior for Cat {
    fn can_behave(&self) -> bool {
        true
    }
    fn behave(&mut self) {
        // behave like a cat, eg: wander, chase butterflies
        self.t.ch = 'Č';
    }
}

///
/// Dog woof woof
///
#[derive(PartialEq, Eq)]
pub struct Dog {
    pub t: AbstractThing,
    pub group: Vec<Group>,
}

impl Default for Dog {
    fn default() -> Self {
        Self {
            t: AbstractThing {
                ch: 'D',
                name: "Dog Woofy".into(),
                // description: "A flully dog"
            },
            group: vec![Group::Pet],
        }
    }
}

// impl Movement for Dog {
//     fn can_move(&self) -> bool {
//         true
//     }
//     fn try_move(&mut self, p: &Point) {
//         // check if there are items or creatures on the map first!
//         // if the map position is empty, them:
//     }
// }

impl Behavior for Dog {
    fn can_behave(&self) -> bool {
        true
    }
    fn behave(&mut self) {
        // behave like a dog, eg: wander, chase cats
        self.t.ch = 'Đ';
    }
}
