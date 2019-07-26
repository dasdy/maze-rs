extern crate image;
extern crate num_complex;
extern crate imageproc;

extern crate gtk;
extern crate gio;
extern crate gdk_pixbuf;

use gio::prelude::*;

mod grid;
mod polar;
mod rectangle;
mod generate;
mod solve;


use gtk::{Application};

use polar::build_polar_ui;


fn create_gtk_app() {
    let application = Application::new("com.dasdy.mazes", Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(build_polar_ui);
    // application.connect_activate(build_ui);

    application.run(&[]);
}

fn main() {
    create_gtk_app();
}