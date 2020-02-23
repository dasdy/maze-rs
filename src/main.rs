extern crate image;
extern crate imageproc;
extern crate num_complex;

extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button};

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
        let img_clone = img.clone();
        
        
        img.set_vexpand(true);
        img.set_hexpand(true);
        let signal_handler: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

        rectangle::draw_rectangle_grid(&img, signal_handler.clone(), 0);
        let signal_handler_1_clone = signal_handler.clone();
        let button = Button::new_with_label("draw rectangle maze");
        button.connect_clicked(move |_| {
            signal_handler_1_clone.store(0, Ordering::Relaxed);
            img_clone.queue_draw();
        });

        let img_clone_2 = img.clone();
        polar::draw_polar_grid(&img_clone_2, signal_handler.clone(), 1);
        let button_polar = Button::new_with_label("draw polar maze");
        let signal_handler_2_clone = signal_handler.clone();
        button_polar.connect_clicked(move |_| {
            signal_handler_2_clone.store(1, Ordering::Relaxed);
            img_clone_2.queue_draw();
        });

        let img_clone_3 = img.clone();
        let button_hex = Button::new_with_label("draw hex maze");
        hexagonal::draw_hex_grid(&img_clone_3, signal_handler.clone(), 2);
        button_hex.connect_clicked(move |_| {
            signal_handler.store(2, Ordering::Relaxed);
            img_clone_3.queue_draw();
        });

        container.add(&img);
        container.add(&button);
        container.add(&button_polar);
        container.add(&button_hex);
        window.add(&container);
        window.show_all();
    });
    application.run(&[]);
}

fn main() {
    create_gtk_app();
}
