use std::collections::HashSet;

pub trait AbstractGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize>;
    fn links(&self, ix: usize) -> HashSet<usize>;
    fn len(&self) -> usize;
    fn link(&mut self, ix1: usize, ix2: usize);
}