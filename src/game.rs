use crate::rng::Rng;

pub struct Game {
    /// viewPort width
    pub v_width: i16,
    /// viewPort height
    pub v_height: i16,

    /// map width
    pub m_width: i16,
    /// map height
    pub m_height: i16,

    /// map background tiles
    /// used for rendering on position
    tiles: Vec<Vec<char>>,

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

        let g = Game {
            v_width: view_w,
            v_height: view_h,
            m_width: map_w,
            m_height: map_h,
            tiles: Vec::new(),
            rng: Rng::from_seed(seed),
        };

        Some(g)
    }

    /// Slide viewport window by delta-X & delta-Y
    ///
    pub fn slide_view(&mut self, d_x: i16, d_y: i16) {
        //
    }

    /// Render view for external use (JSX)
    ///
    pub fn render(&mut self) {
        //
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
}
