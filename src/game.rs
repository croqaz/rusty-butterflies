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

    /// randon nr generator
    rng: Rng,
}
