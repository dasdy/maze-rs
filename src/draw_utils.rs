use crate::gtk::prelude::WidgetExt;
use crate::solve::{DijkstraStep};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub trait GtkDrawable {
    fn draw_pathfind(
        &self,
        w: &gtk::DrawingArea,
        cr: &gtk::cairo::Context,
        step_state: &DijkstraStep,
        cellsize: f64,
    );
    fn draw_maze(&self, w: &gtk::DrawingArea, cr: &gtk::cairo::Context, cellsize: f64);
}


pub fn draw_grid_mutex<T: 'static + GtkDrawable + Clone>(
    img: &gtk::DrawingArea,
    signal_handler: Arc<AtomicUsize>,
    g: Arc<Mutex<(T, DijkstraStep)>>,
    on_value: usize,
) {
    let cellsize = 10.;
    img.connect_draw(move |w, cr| {
        if signal_handler.load(Ordering::Relaxed) == on_value {
            let data = g.lock().unwrap();
            let (graph, step_state) = &*data;
            graph.draw_pathfind(w, cr, step_state, cellsize);
            graph.draw_maze(w, cr, cellsize);
        }
        gtk::Inhibit(false)
    });
}
