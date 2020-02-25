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
use std::sync::Arc;

fn create_gtk_app() {
    let application = Application::new(Some("com.dasdy.mazes"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let img = gtk::DrawingArea::new();
        img.set_size_request(400, 400);
        container.add(&img);

        let img_clone = img.clone();

        img.set_vexpand(true);
        img.set_hexpand(true);
        let signal_handler: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

        draw_utils::draw_grid(
            &img,
            signal_handler.clone(),
            &mut rectangle::RectangleGrid::new(70, 70),
            0,
        );
        let signal_handler_1_clone = signal_handler.clone();
        let button = Button::new_with_label("draw rectangle maze");
        button.connect_clicked(move |_| {
            signal_handler_1_clone.store(0, Ordering::Relaxed);
            img_clone.queue_draw();
        });
        container.add(&button);

        let img_clone_2 = img.clone();
        draw_utils::draw_grid(
            &img_clone_2,
            signal_handler.clone(),
            &mut polar::CircularGrid::new(40),
            1,
        );
        let button_polar = Button::new_with_label("draw polar maze");
        let signal_handler_2_clone = signal_handler.clone();
        button_polar.connect_clicked(move |_| {
            signal_handler_2_clone.store(1, Ordering::Relaxed);
            img_clone_2.queue_draw();
        });
        container.add(&button_polar);

        let img_clone_3 = img.clone();
        let button_hex = Button::new_with_label("draw hex maze");
        draw_utils::draw_grid(
            &img_clone_3,
            signal_handler.clone(),
            &mut hexagonal::HexagonalGrid::new(50, 50),
            2,
        );
        let signal_handler_3_clone = signal_handler.clone();
        button_hex.connect_clicked(move |_| {
            signal_handler_3_clone.store(2, Ordering::Relaxed);
            img_clone_3.queue_draw();
        });
        container.add(&button_hex);

        let img_clone_4 = img;
        let button_delta = Button::new_with_label("draw delta maze");
        draw_utils::draw_grid(
            &img_clone_4,
            signal_handler.clone(),
            &mut delta::DeltaGrid::new(45, 60),
            3,
        );
        button_delta.connect_clicked(move |_| {
            signal_handler.store(3, Ordering::Relaxed);
            img_clone_4.queue_draw();
        });
        container.add(&button_delta);

        window.add(&container);
        window.show_all();
    });
    application.run(&[]);
}

fn main() {
    create_gtk_app();
}
