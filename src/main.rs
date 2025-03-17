use draw_utils::{GenerationType, MazeType, Settings};
use gtk::{prelude::*, Entry, RadioButton};
use gtk::{ApplicationWindow, Button};
use crate::gtk::prelude::ApplicationExt;
use crate::gtk::prelude::ApplicationExtManual;

extern crate image;
extern crate imageproc;
extern crate num_complex;

extern crate cairo;
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;


mod delta;
mod draw_utils;
mod generate;
mod grid;
mod hexagonal;
mod polar;
mod rectangle;
mod solve;
use gtk::Application;
use solve::DijkstraStep;
use std::sync::{Arc, RwLock};
use std::time::Instant;


fn add_maze_option(
    img: gtk::DrawingArea,
    container: &gtk::Box,
    switch_val: MazeType,
    button_name: &str,
    settings: Arc<RwLock<Settings>>,
) {
    let bname_copy = String::from(button_name);
    let button = Button::with_label(button_name);
    button.connect_clicked(move |_| {
        let now = Instant::now();
        {
            let mut real_settings = settings.write().unwrap();
            real_settings.maze_type = switch_val.clone();
            real_settings.make_maze();
            draw_utils::draw_grid_mutex(
                &img,
                settings.clone(),
                real_settings.generation_type.clone(),
                real_settings.maze_type.clone(),
                real_settings.braid_chance,
                real_settings.version,
            );
        }
        img.queue_draw();
        let new_now = Instant::now();
        println!(
            "Click {:?} processed: {:?}",
            bname_copy,
            new_now.duration_since(now)
        );
    });
    container.add(&button);
}

fn add_maze_generator(
    img: gtk::DrawingArea,
    container: &gtk::Box,
    radio_group: &gtk::RadioButton,
    settings: Arc<RwLock<Settings>>,
    switch_val: GenerationType,
    button_name: &str,
    selected: bool,
) {
    let radio = RadioButton::with_label_from_widget(radio_group, button_name);

    let bname_copy = String::from(button_name);

    radio.connect_clicked(move |_| {
        let now = Instant::now();
        {
            let mut real_settings = settings.write().unwrap();
            real_settings.generation_type = switch_val.clone();
            real_settings.make_maze();
            draw_utils::draw_grid_mutex(
                &img,
                settings.clone(),
                real_settings.generation_type.clone(),
                real_settings.maze_type.clone(),
                real_settings.braid_chance,
                real_settings.version,
            );
        }
        img.queue_draw();
        let new_now = Instant::now();
        println!(
            "Click {:?} processed: {:?}",
            bname_copy,
            new_now.duration_since(now)
        );
    });
    radio.set_active(selected);
    container.add(&radio)
}

fn create_gtk_app() {
    let application = Application::new(Some("com.dasdy.mazes"), Default::default());

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let buttons_container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let maze_buttons = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let radio_container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let radio_button = gtk::RadioButton::new();

        let img = gtk::DrawingArea::new();
        img.set_size_request(400, 400);
        container.add(&img);
        container.add(&buttons_container);
        buttons_container.add(&maze_buttons);
        buttons_container.add(&radio_container);

        img.set_vexpand(true);
        img.set_hexpand(true);

        let f = || Box::new(rectangle::RegularGrid::new(70, 70));
        let settings = Arc::new(RwLock::new(Settings {
            maze_type: MazeType::Regular,
            generation_type: GenerationType::RecursiveBacktracker,
            braid_chance: 0,
            step: DijkstraStep::initial(&rectangle::RegularGrid::new(70, 70), 0),
            version: 0,
            drawn: false,
            grid: f(),
        }));

        add_maze_option(
            img.clone(),
            &maze_buttons,
            MazeType::Regular,
            "draw rectangle maze",
            settings.clone(),
        );
        add_maze_option(
            img.clone(),
            &maze_buttons,
            MazeType::Circular,
            "draw polar maze",
            settings.clone(),
        );
        add_maze_option(
            img.clone(),
            &maze_buttons,
            MazeType::Hexagonal,
            "draw hex maze",
            settings.clone(),
        );
        add_maze_option(
            img.clone(),
            &maze_buttons,
            MazeType::Delta,
            "draw delta maze",
            settings.clone(),
        );

        add_maze_generator(
            img.clone(),
            &radio_container,
            &radio_button,
            settings.clone(),
            GenerationType::RecursiveBacktracker,
            "Recursive Backtracker",
            true,
        );
        add_maze_generator(
            img.clone(),
            &radio_container,
            &radio_button,
            settings.clone(),
            GenerationType::AldousBroder,
            "Aldous Broder",
            false,
        );
        add_maze_generator(
            img.clone(),
            &radio_container,
            &radio_button,
            settings.clone(),
            GenerationType::SimplifiedPrim,
            "Simplified Prim",
            false,
        );
        add_maze_generator(
            img.clone(),
            &radio_container,
            &radio_button,
            settings.clone(),
            GenerationType::TruePrim,
            "True Prim",
            false,
        );

        let entry = Entry::new();
        entry.set_text("0");
        entry.set_max_length(3);

        let img_clone = img.clone();
        let s_clone = settings.clone();
        entry.connect_changed(move |w| {
            let now = Instant::now();
            let num = w.text().parse::<u8>().unwrap_or_default();
            {
                let mut real_settings = s_clone.write().unwrap();

                real_settings.braid_chance = num;

                real_settings.make_maze();
                draw_utils::draw_grid_mutex(
                    &img_clone,
                    s_clone.clone(),
                    real_settings.generation_type.clone(),
                    real_settings.maze_type.clone(),
                    real_settings.braid_chance,
                    real_settings.version,
                );
            }
            img_clone.queue_draw();
            let new_now = Instant::now();
            println!("Braid chance  processed: {:?}", new_now.duration_since(now));
        });

        {
            let mut real_settings = settings.write().unwrap();
            real_settings.make_maze();
            draw_utils::draw_grid_mutex(
                &img.clone(),
                settings.clone(),
                real_settings.generation_type.clone(),
                real_settings.maze_type.clone(),
                real_settings.braid_chance,
                real_settings.version,
            );
        }
        // entry.(b)
        radio_container.add(&entry);

        window.add(&container);
        window.show_all();
    });
    application.run();
}

fn main() {
    create_gtk_app();
}
