use crate::generate::{recursive_backtracker, braid};
use crate::grid::{AbstractCell, AbstractGrid};
use crate::gtk::WidgetExt;
use crate::solve::{solve_with_longest_path, DijkstraStep};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub trait GtkDrawable<T: AbstractCell>: AbstractGrid<T> {
    fn draw_pathfind(
        &self,
        w: &gtk::DrawingArea,
        cr: &cairo::Context,
        step_state: &DijkstraStep,
        cellsize: f64,
    );
    fn draw_maze(&self, w: &gtk::DrawingArea, cr: &cairo::Context, cellsize: f64);
}

pub fn draw_grid<C: AbstractCell, T: 'static + GtkDrawable<C> + Clone>(
    img: &gtk::DrawingArea,
    signal_handler: Arc<AtomicUsize>,
    g: &mut T,
    on_value: usize,
) {
    let mut rng = rand::thread_rng();
    recursive_backtracker(g, &mut rng);
    braid(g, &mut rng);

    let g_1 = g.clone();
    let cellsize = 10.;

    let step_state = solve_with_longest_path(g);

    img.connect_draw(move |w, cr| {
        if signal_handler.load(Ordering::Relaxed) == on_value {
            g_1.draw_pathfind(w, cr, &step_state, cellsize);
            g_1.draw_maze(w, cr, cellsize);
        }
        gtk::Inhibit(false)
    });
}
