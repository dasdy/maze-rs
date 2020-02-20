use std::collections::HashSet;

pub trait AbstractGrid<T: AbstractCell> {
    fn neighbours(&self, ix: usize) -> Vec<usize>;
    fn links(&self, ix: usize) -> HashSet<usize>;
    fn len(&self) -> usize;
    fn link(&mut self, ix1: usize, ix2: usize);
    fn cell(&self, ix: usize) -> &T;
}

pub trait CompassDirections {
    fn north_ix(&self, ix: usize) -> Option<usize>;
    fn east_ix(&self, ix: usize) -> Option<usize>;
    fn south_ix(&self, ix: usize) -> Option<usize>;
    fn west_ix(&self, ix: usize) -> Option<usize>;
}

pub trait CompassGrid<T: AbstractCell>: AbstractGrid<T> + CompassDirections {}

pub trait AbstractCell {
    fn row(&self) -> usize;
    fn col(&self) -> usize;
    fn links(&self) -> HashSet<usize>;
    fn link(&mut self, ix: usize);
}
