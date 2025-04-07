pub mod grid;
use std::hash::Hash;
/// # TitleCoords
/// Helper structure to store the location of title in the grid space (x,y)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TitleCoords {
    pub x: usize,
    pub y: usize,
}

/// # Title
/// Title type
#[derive(PartialEq, Debug)]
pub enum Title {
    Normal { was_visited: bool },
    Start,
    End,
    Obstacle,
    Path,
    Process,
}
