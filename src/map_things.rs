use crate::map_actor::*;
use crate::map_common::*;
use crate::map_decor::*;

///
/// Wrapper for items & entities
///
#[derive(PartialEq, Eq)]
pub enum MapThing {
    Decor(MapDecor),
    Actor(MapActor),
}

impl MapThing {
    pub fn thing(&self) -> &AbstractThing {
        match self {
            MapThing::Decor(a) => a.thing(),
            MapThing::Actor(a) => a.thing(),
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
    Acat(Cat),
    Adog(Dog),
}

impl MapActor {
    pub fn thing(&self) -> &AbstractThing {
        match self {
            MapActor::Acat(a) => &a.t,
            MapActor::Adog(a) => &a.t,
            MapActor::Abutterfly(a) => &a.t,
        }
    }
}

impl Behavior for MapActor {
    fn can_behave(&self) -> bool {
        match self {
            MapActor::Acat(a) => a.can_behave(),
            MapActor::Adog(a) => a.can_behave(),
            MapActor::Abutterfly(a) => a.can_behave(),
        }
    }
    fn behave(&mut self) {
        match self {
            MapActor::Acat(a) => a.behave(),
            MapActor::Adog(a) => a.behave(),
            MapActor::Abutterfly(a) => a.behave(),
        }
    }
}
