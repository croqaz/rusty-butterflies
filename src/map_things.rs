use crate::map_actor::*;
use crate::map_common::*;
use crate::map_decor::*;

///
/// Wrapper for items & entities
///
#[derive(PartialEq, Eq)]
pub enum MapThing {
    None,
    Decor(MapDecor),
    Actor(MapActor),
}

impl MapThing {
    pub fn thing(&self) -> Option<&AbstractThing> {
        match self {
            MapThing::None => None,
            MapThing::Decor(a) => Some(a.thing()),
            MapThing::Actor(a) => Some(a.thing()),
        }
    }
}

///
/// Wrapper for items only
/// All items implement interaction
///
#[derive(PartialEq, Eq)]
pub enum MapDecor {
    Achest(Chest),
}

impl MapDecor {
    pub fn thing(&self) -> &AbstractThing {
        match self {
            MapDecor::Achest(a) => &a.t,
        }
    }
}

impl Interaction for MapDecor {
    fn can_interact(&self) -> bool {
        match self {
            MapDecor::Achest(a) => a.can_interact(),
        }
    }
    fn interact(&mut self) {
        match self {
            MapDecor::Achest(a) => a.interact(),
        }
    }
}

///
/// Wrapper for creatures only
/// All creatures implement some behavior
///
#[derive(PartialEq, Eq)]
pub enum MapActor {
    Abutterfly(Butterfly),
}

impl MapActor {
    pub fn thing(&self) -> &AbstractThing {
        match self {
            MapActor::Abutterfly(a) => &a.t,
        }
    }
}

impl Behavior for MapActor {
    fn can_behave(&self) -> bool {
        true
    }
    fn behave(&mut self) {
        //
    }
}
