use crate::game::Game;
use crate::geom::Point;
use crate::map_common::*;

///
/// Player struct
///
#[derive(PartialEq, Eq)]
pub struct Player {
    pub t: AbstractThing,
    pub group: Vec<Group>,
    found_net: bool,
    butterflies: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            t: AbstractThing {
                ch: '@',
                name: "You".into(),
            },
            group: vec![Group::Player, Group::Human],
            found_net: false,
            butterflies: 0,
        }
    }
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

impl Movement for Butterfly {
    fn can_move(&self) -> bool {
        true
    }
    fn try_move(&mut self, p: &Point, g: &mut Game) -> bool {
        if !p.is_valid(g) {
            return false;
        }
        // check if there are items or creatures on the map!
        // if the map position is NOT empty, cannot move
        if g.things.contains_key(&p) {
            return false;
        };
        // TODO: check if map tile is walkable
        //
        true
    }
}

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

impl Behavior for Dog {
    fn can_behave(&self) -> bool {
        true
    }
    fn behave(&mut self) {
        // behave like a dog, eg: wander, chase cats
        self.t.ch = 'Đ';
    }
}
