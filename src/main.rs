extern crate image;
extern crate imageproc;
extern crate num_complex;

extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;

use gio::prelude::*;

mod generate;
mod grid;
mod polar;
mod rectangle;
mod solve;

use gtk::Application;

fn create_gtk_app() {
    let application = Application::new(Some("com.dasdy.mazes"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(polar::build_polar_ui);
    // application.connect_activate(rectangle::build_ui);

    application.run(&[]);
}

fn main() {
    create_gtk_app();
}
