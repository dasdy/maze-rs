use std::collections::HashSet;

pub trait AbstractGrid<T: AbstractCell> {
    fn neighbours(&self, ix: usize) -> Vec<usize>;
    fn len(&self) -> usize;
    fn cell_mut(&mut self, ix: usize) -> &mut T;
    fn cell(&self, ix: usize) -> &T;
    fn link(&mut self, ix1: usize, ix2: usize) {
        (self.cell_mut(ix1)).link(ix2);
        (self.cell_mut(ix2)).link(ix1);
    }
    fn links(&self, ix: usize) -> HashSet<usize> {
        self.cell(ix).links().iter().cloned().collect()
    }
}

pub trait CompassDirections {
    fn north_ix(&self, ix: usize) -> Option<usize>;
    fn east_ix(&self, ix: usize) -> Option<usize>;
    fn south_ix(&self, ix: usize) -> Option<usize>;
    fn west_ix(&self, ix: usize) -> Option<usize>;
}

pub trait CompassGrid<T: AbstractCell>: AbstractGrid<T> + CompassDirections {}

pub trait RectangularGrid {
    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn ix(&self, row: usize, col: usize) -> usize {
        col + row * self.width()
    }

    fn ix_opt(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.height() || col >= self.width() {
            return None;
        }
        Some(self.ix(row, col))
    }
}

pub trait AbstractCell {
    fn row(&self) -> usize;
    fn col(&self) -> usize;
    fn links(&self) -> HashSet<usize>;
    fn link(&mut self, ix: usize);
}
