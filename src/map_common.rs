use crate::geom::Point;

/// All items implement interaction
///
pub trait Interaction {
    fn can_interact(&self) -> bool;
    fn interact(&mut self);
}

/// Most of the creatures implement movement
/// Some won't need it, eg: trees, spawners
///
pub trait Movement {
    fn can_move(&self) -> bool;
    fn try_move(&mut self, p: &Point) {
        if p.x < 0 || p.y < 0 {
            return;
        }
    }
}

/// All creatures implement some behavior
/// Examples: wander, flee, hunt
///
pub trait Behavior {
    fn can_behave(&self) -> bool;
    fn behave(&mut self);
}

/// All entities need 1-2 groups
///
#[derive(Default, PartialEq, Eq)]
pub enum Group {
    #[default]
    None,
    Player,
    Human,
    Insect,
    Pet,
}

/// Base for all items & entities
///
#[derive(PartialEq, Eq)]
pub struct AbstractThing {
    /// visual char
    pub ch: char,
    /// short unique name
    pub name: String,
    // some description
    // descrip: String,
}

impl Default for AbstractThing {
    fn default() -> Self {
        Self {
            ch: '?',
            name: "Nothing".into(),
            // description: ...
        }
    }
}
