extern crate image;
extern crate num_complex;
extern crate imageproc;

extern crate gtk;
extern crate gio;
extern crate gdk_pixbuf;
extern crate palette;

use gtk::prelude::*;
use gio::prelude::*;

mod grid;
mod generate;
mod solve;

use grid::Grid;
use generate::*;


use gtk::{Application, ApplicationWindow, DrawingArea};
use cairo::Context;
use crate::solve::DijkstraStep;
use std::f64::consts::PI;
use palette::{LinSrgb, Lch, Srgb, Hue};

fn draw_maze(w: &DrawingArea, cr: &Context, g: &Grid, cellsize: f64) {
    let scalex = w.get_allocated_width() as f64 / (g.width as f64 * cellsize);
    let scaley = w.get_allocated_height() as f64 / (g.height as f64 * cellsize);


    cr.scale(scalex, scaley);
    cr.set_line_width(1.0);
    for ix in 0..g.cells.len() {
        let cur_cell = &g.cells[ix];
        let draw_line =
            |item: &Option<usize>, end: (f64, f64)| {
                match item {
                    Some(r_idx) if !cur_cell.linked(&(g.cells[*r_idx])) =>
                        cr.line_to(end.0, end.1),
                    _ => cr.move_to(end.0, end.1)
                }
            };
        let pixcoord = |ix: usize| -> f64 {
            ix as f64 * cellsize
        };
        let x1 = pixcoord(cur_cell.col);
        let x2 = pixcoord(cur_cell.col + 1);
        let y1 = pixcoord(cur_cell.row);
        let y2 = pixcoord(cur_cell.row + 1);
        cr.move_to(x1, y1);
        draw_line(&g.west_ix(cur_cell.row, cur_cell.col), (x1, y2));
        draw_line(&g.south_ix(cur_cell.row, cur_cell.col), (x2, y2));
        draw_line(&g.east_ix(cur_cell.row, cur_cell.col), (x2, y1));
        draw_line(&g.north_ix(cur_cell.row, cur_cell.col), (x1, y1));
        cr.stroke();
    }
}

fn draw_pathfind(w: &DrawingArea, cr: &Context, g: &Grid, step_state: &DijkstraStep, cellsize: f64,
                 start: usize) {
    let scalex = w.get_allocated_width() as f64 / (g.width as f64 * cellsize);
    let scaley = w.get_allocated_height() as f64 / (g.height as f64 * cellsize);
    cr.scale(scalex, scaley);
    cr.set_line_width(1.0);

    let pixcoord = |ix: usize| -> f64 {
        (ix as f64 + 0.5) * cellsize
    };

    let circle = |x: f64, y: f64| {
        cr.save();
        cr.translate(x, y);
        cr.arc(0.,0.,cellsize / 2.,0., 2. * PI);
        cr.restore();
    };

    let coords = |ix: i32| {
        let row = g.cells[ix as usize].row;
        let col = g.cells[ix as usize].col;
        (pixcoord(col), pixcoord(row))
    };

    let line = |i1: i32, i2: i32| {
        let (x1, y1) = coords(i1);
        let (x2, y2) = coords(i2);

        cr.move_to(x1, y1);
        cr.line_to(x2, y2);
        cr.stroke();
    };

    let end = g.cells.len() - 1;
    let cur_cell = &g.cells[start];
    let end_cell = &g.cells[end];
    let x1 = pixcoord(cur_cell.col);
    let x2 = pixcoord(end_cell.col);

    let y1 = pixcoord(cur_cell.row);
    let y2 = pixcoord(end_cell.row);
    circle(x1,y1);
    cr.stroke();
    circle(x2,y2);
    cr.stroke();

    cr.set_line_width(3.0);
    for (i, c) in step_state.cell_weights.iter().enumerate() {
        if c.parent >= 0 {
            let base_color: Lch = Srgb::new(0.8, 0.2, 0.1).into();
            let new_color =
                LinSrgb::from(base_color.shift_hue((c.path_length as f32) * 10.));


            cr.set_source_rgb(new_color.red as f64, new_color.green as f64, new_color.blue as f64);

            line(i as i32, c.parent);
        }
    }


    if step_state.cell_weights[end].parent > 0 {
        let mut cur_cell = end as i32;
        cr.set_source_rgb(1., 1., 0.);
        cr.set_line_width(4.0);
        while cur_cell != (start as i32) {
            line(cur_cell, step_state.cell_weights[cur_cell as usize].parent);
            cur_cell = step_state.cell_weights[cur_cell as usize].parent;
        }
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.set_default_size(400, 400);

    let img = gtk::DrawingArea::new();
    vbox.add(&img);
    let mut g = Grid::new(25, 25);
    let mut rng = rand::thread_rng();

//    sidewinder(&mut g, &mut rng);
//    binary_tree(&mut g, &mut rng);
//    aldous_broder(&mut g, &mut rng);
//    hunt_and_kill(&mut g, &mut rng);
    recursive_backtracker(&mut g, &mut rng);

    img.set_vexpand(true);
    img.set_hexpand(true);
    let g_copy = g.clone();
    let cellsize = 10.;
    img.connect_draw(move |w, cr| {
        draw_maze(w, cr, &g_copy, cellsize);
        gtk::Inhibit(false)
    });

    let start = g._ix(24, 0);
    let mut step_state = DijkstraStep::initial(&g, start);
    while !step_state.lookup_queue.is_empty() {
        step_state = step_state.next_step(&g);
    }

    img.connect_draw(move |w, cr| {
        draw_pathfind(w, cr, &g, &step_state, cellsize, start);
        gtk::Inhibit(false)
    });

    window.add(&vbox);

    window.show_all();
}

fn create_gtk_app() {
    let application = Application::new("com.github.gtk-rs.examples.basic",
                                       Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(build_ui);

    application.run(&[]);
}

fn main() {
    create_gtk_app();
}