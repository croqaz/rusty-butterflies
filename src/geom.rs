use std::cmp;

use crate::game::Game;

#[allow(dead_code)]
pub enum Direction {
    N,
    Up,
    S,
    Down,
    E,
    Right,
    W,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[allow(dead_code)]
impl Point {
    pub fn is_valid(&self, g: &Game) -> bool {
        return self.x >= 0 && self.x < g.m_width && self.y >= 0 && self.y < g.m_height;
    }

    pub fn is_visible(&self, g: &Game) -> bool {
        let vw = &g.vw;
        return self.x >= vw.top_left_x
            && self.x <= vw.bot_right_x
            && self.y >= vw.top_left_y
            && self.y <= vw.bot_right_y;
    }

    pub fn dist(&self, p: Point) -> i16 {
        let dx = (p.x - self.x) as f32;
        let dy = (p.y - self.y) as f32;
        f32::sqrt(dx * dx + dy * dy).round() as i16
    }

    pub fn dist8(&self, p: Point) -> i16 {
        let dx = p.x - self.x;
        let dy = p.y - self.y;
        cmp::max(dx.abs(), dy.abs())
    }
}
