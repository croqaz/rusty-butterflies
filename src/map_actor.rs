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
