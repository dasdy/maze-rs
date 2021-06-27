extern crate image;
extern crate imageproc;
extern crate num_complex;

extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button};

mod delta;
mod draw_utils;
mod generate;
mod grid;
mod hexagonal;
mod polar;
mod rectangle;
mod solve;
use gtk::Application;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

fn make_tha_maze<C: grid::AbstractCell, T: grid::AbstractGrid<C>>(g: &mut T) {
    let mut rng = rand::thread_rng();
    // generate::recursive_backtracker(g, &mut rng);
    // generate::aldous_broder(g, &mut rng);
    // generate::simplified_prim(g, &mut rng);
    generate::true_prim(g, &mut rng);
    // generate::braid(g, &mut rng, 25);
}

fn add_maze_option<
    C: grid::AbstractCell,
    T: 'static + grid::AbstractGrid<C> + Clone + draw_utils::GtkDrawable,
>(
    g: T,
    img: gtk::DrawingArea,
    signal_handler: Arc<AtomicUsize>,
    container: &gtk::Box,
    switch_val: usize,
    button_name: &str,
) {
    let mut rect_grid = g.clone();
    let img_clone = img.clone();
    make_tha_maze(&mut rect_grid);
    let step_state = solve::solve_with_longest_path(&rect_grid);
    let rect_guard = Arc::new(Mutex::new((rect_grid, step_state)));
    draw_utils::draw_grid_mutex(&img, signal_handler.clone(), rect_guard.clone(), switch_val);
    let button = Button::with_label(button_name);
    button.connect_clicked(move |_| {
        let mut rect_grid = g.clone();
        make_tha_maze(&mut rect_grid);
        let step_state = solve::solve_with_longest_path(&rect_grid);
        {
            let mut data = rect_guard.lock().unwrap();
            *data = (rect_grid, step_state);
        }
        signal_handler.store(switch_val, Ordering::Relaxed);
        img_clone.queue_draw();
    });
    container.add(&button);
}

fn create_gtk_app() {
    let application = Application::new(Some("com.dasdy.mazes"), Default::default());

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let img = gtk::DrawingArea::new();
        img.set_size_request(400, 400);
        container.add(&img);

        img.set_vexpand(true);
        img.set_hexpand(true);
        let signal_handler: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        add_maze_option(
            rectangle::RegularGrid::new(70, 70),
            img.clone(),
            signal_handler.clone(),
            &container,
            0,
            "draw rectangle maze"
        );
        add_maze_option(
            polar::CircularGrid::new(40),
            img.clone(),
            signal_handler.clone(),
            &container,
            1,
            "draw polar maze"
        );
        add_maze_option(
            hexagonal::HexagonalGrid::new(50, 50),
            img.clone(),
            signal_handler.clone(),
            &container,
            2,
            "draw hex maze"
        );
        add_maze_option(
            delta::DeltaGrid::new(45, 70),
            img,
            signal_handler,
            &container,
            3,
            "draw delta maze"
        );

        window.add(&container);
        window.show_all();
    });
    application.run();
}

fn main() {
    create_gtk_app();
}
