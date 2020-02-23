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
mod polar;
mod rectangle;
mod solve;
use std::sync::{Mutex, Arc};
use gtk::Application;

fn create_gtk_app() {
    let application = Application::new(Some("com.dasdy.mazes"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let button = Button::new_with_label("draw rectangle maze");
        let button_polar = Button::new_with_label("draw polar maze");
        
        let img = gtk::DrawingArea::new();
        img.set_size_request(400, 400);
        let img_clone = img.clone();
        let img_clone_2 = img.clone();
        img.set_vexpand(true);
        img.set_hexpand(true);
        let signal_handler: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
        
        rectangle::draw_rectangle_grid(&img, signal_handler.clone(), true);
        polar::draw_polar_grid(&img_clone_2, signal_handler.clone(), false);
        let signal_handler_1_clone = signal_handler.clone();
        button.connect_clicked(move |_| {
            let mut val = signal_handler_1_clone.lock().unwrap();
            *val = true;
            img_clone.queue_draw();
        });


        button_polar.connect_clicked(move |_| {
            let mut val = signal_handler.lock().unwrap();
            *val = false;
            img_clone_2.queue_draw();
        });

        container.add(&img);
        container.add(&button);
        container.add(&button_polar);
        window.add(&container);
        window.show_all();
    });
    application.run(&[]);
}

fn main() {
    create_gtk_app();
}
