use serde::Serialize;
use std::cmp;
use std::collections::HashMap;

use crate::geom::Point;
use crate::map_actor::*;
use crate::map_common::*;
use crate::map_decor::*;
use crate::map_things::*;
use crate::rng::Rng;
// use crate::util::*;

pub struct Game {
    /// viewPort width
    pub v_width: i16,
    /// viewPort height
    pub v_height: i16,

    /// map width
    pub m_width: i16,
    /// map height
    pub m_height: i16,

    /// viewport object
    pub vw: ViewPort,

    /// player object
    player: Player,

    /// map background tiles
    /// used for rendering on position
    tiles: Vec<Vec<char>>,

    /// all things: actors & decor
    /// used for turns & listing all things from the map
    things: HashMap<Point, MapThing>,

    /// randon nr generator
    rng: Rng,
}

impl Game {
    pub fn new(view_w: i16, view_h: i16, map_w: i16, map_h: i16, seed: i64) -> Option<Self> {
        if view_w < 4 || view_h < 4 {
            // console_err!("View WxH too small: {view_w}x{view_h}");
            return None;
        }
        if view_w % 2 != 0 || view_h % 2 != 0 {
            // console_err!("View WxH not even: {view_w}x{view_h}");
            return None;
        }

        if map_w < 8 || map_h < 8 {
            // console_err!("Map WxH too small: {map_w}x{map_h}");
            return None;
        }

        let player = Player {
            t: AbstractThing {
                ch: '@',
                name: "You".into(),
            },
            group: vec![Group::Player, Group::Human],
        };

        let mut g = Game {
            v_width: view_w,
            v_height: view_h,
            m_width: map_w,
            m_height: map_h,
            tiles: Vec::new(),
            things: HashMap::new(),
            vw: ViewPort::default(),
            rng: Rng::from_seed(seed),
            player,
        };

        g.generate_map_lvl_1();

        Some(g)
    }

    /// Slide viewport window by delta-X & delta-Y
    ///
    pub fn slide_view(&mut self, d_x: i16, d_y: i16) -> bool {
        let new_x = self.vw.center.x + d_x;
        let new_y = self.vw.center.y + d_y;

        let new_p = Point { x: new_x, y: new_y };
        if !new_p.is_valid(&self) {
            return false;
        }

        // console_log!("Slide view by {d_x}x{d_y}, to {new_x}x{new_y}");
        self.vw.center = new_p;

        match self.things.get_mut(&new_p) {
            Some(t) => match t {
                MapThing::Decor(t) => {
                    if t.can_interact() {
                        t.interact();
                    }
                }
                MapThing::Actor(t) => {
                    if t.can_behave() {
                        t.behave();
                    }
                }
            },
            _ => {}
        };

        true
    }

    /// Render view for external use (JSX)
    ///
    pub fn render(&mut self) -> Vec<Vec<Vnode>> {
        {
            let half_width = self.v_width / 2;
            let half_height = self.v_height / 2;

            // Make sure the x-axis doesn't go to the left of the left bound
            let top_left_x = cmp::max(0, self.vw.center.x - half_width);
            // Make sure we still have enough space to fit an entire game screen
            self.vw.top_left_x = cmp::min(top_left_x, self.m_width - self.v_width);
            // Make sure the y-axis doesn't above the top bound
            let top_left_y = cmp::max(0, self.vw.center.y - half_height);
            // Make sure we still have enough space to fit an entire game screen
            self.vw.top_left_y = cmp::min(top_left_y, self.m_height - self.v_height);

            self.vw.bot_right_x = self.vw.top_left_x + self.v_width;
            self.vw.bot_right_y = self.vw.top_left_y + self.v_height;
        };

        // TODO: reuse window instead of allocating new
        let mut window: Vec<Vec<Vnode>> = Vec::with_capacity(self.v_height as usize);

        // Iterate through all visible map cells
        for d_y in self.vw.top_left_y..self.vw.bot_right_y {
            let mut line: Vec<Vnode> = Vec::with_capacity(self.v_width as usize);
            for d_x in self.vw.top_left_x..self.vw.bot_right_x {
                // Fetch the glyph for tile and render it to screen at offset position
                let tile = self.tiles[d_y as usize][d_x as usize];
                // Should export AS MUCH INFO as possible
                // enough for the front-end to decide what to do with the map
                // EG: creature groups, name, description...
                line.push(Vnode {
                    name: "span".into(),
                    props: Vec::new(),
                    children: tile.into(),
                });
            }
            window.push(line);
        }

        // Render all things & entities ...
        for (xy, mt) in &self.things {
            if !xy.is_visible(&self) {
                continue;
            }
            let t = mt.thing();
            window[(xy.y - self.vw.top_left_y) as usize][(xy.x - self.vw.top_left_x) as usize]
                .children = t.ch.into();
        }

        // Render Player at ViewPort center
        window[(self.vw.center.y - self.vw.top_left_y) as usize]
            [(self.vw.center.x - self.vw.top_left_x) as usize]
            .children = self.player.t.ch.into();

        window
    }

    /// clear all map bg & fg
    ///
    fn clear_map(&mut self, c: char) {
        let h = self.m_height as usize;
        let w = self.m_width as usize;
        if self.tiles.capacity() == h {
            self.tiles.clear();
        } else {
            self.tiles = Vec::with_capacity(h);
        }
        for _ in 0..self.tiles.capacity() {
            let mut row = Vec::with_capacity(w);
            for _ in 0..row.capacity() {
                row.push(c);
            }
            self.tiles.push(row);
        }
    }

    /// generate LVL#1 map
    ///
    fn generate_map_lvl_1(&mut self) {
        // clear everything
        self.clear_map(',');

        let max_w = self.m_width as i64 - 2;
        let max_h = self.m_height as i64 - 2;

        // draw grass
        for _ in 0..25 {
            let x = self.rng.get_uniform_int(2, max_w) as usize;
            let y = self.rng.get_uniform_int(2, max_h) as usize;
            self.tiles[y][x] = '"';
        }
        for _ in 0..25 {
            let x = self.rng.get_uniform_int(2, max_w) as usize;
            let y = self.rng.get_uniform_int(2, max_h) as usize;
            self.tiles[y][x] = '\'';
        }

        // draw small flowers
        for _ in 0..25 {
            let x = self.rng.get_uniform_int(2, max_w) as usize;
            let y = self.rng.get_uniform_int(2, max_h) as usize;
            self.tiles[y][x] = 'o';
        }
        // draw big flowers
        for _ in 0..25 {
            let x = self.rng.get_uniform_int(2, max_w) as usize;
            let y = self.rng.get_uniform_int(2, max_h) as usize;
            self.tiles[y][x] = '0';
        }

        self.register_entity(
            Point { x: 9, y: 9 },
            MapThing::Decor(MapDecor::Achest(Chest::default())),
        );

        self.register_entity(
            Point { x: 3, y: 3 },
            MapThing::Actor(MapActor::Abutterfly(Butterfly::default())),
        );

        self.register_entity(
            Point { x: 6, y: 6 },
            MapThing::Actor(MapActor::Acat(Cat::default())),
        );

        self.register_entity(
            Point { x: 7, y: 7 },
            MapThing::Actor(MapActor::Adog(Dog::default())),
        );
    }

    /// Engine logic
    ///
    fn register_entity(&mut self, xy: Point, t: MapThing) -> bool {
        if !xy.is_valid(&self) {
            return false;
        }
        self.things.insert(xy, t);
        true
    }

    #[allow(dead_code)]
    fn remove_entity(&mut self, t: &MapThing) -> bool {
        let found: Vec<Point> = self
            .things
            .iter()
            .filter(|&(_, v)| v == t)
            .map(|(xy, _)| *xy) // copy Point
            .collect();
        for xy in found {
            self.things.remove(&xy);
        }
        true
    }
}

/// ViewPort Window
///
#[derive(Default)]
pub struct ViewPort {
    pub center: Point,
    pub top_left_x: i16,
    pub top_left_y: i16,
    pub bot_right_x: i16,
    pub bot_right_y: i16,
}

/// External Virtual node (used for JSX)
///
#[derive(Serialize)]
pub struct Vnode {
    #[serde(rename = "type")]
    pub name: String,
    pub props: Vec<String>,
    pub children: String,
}
