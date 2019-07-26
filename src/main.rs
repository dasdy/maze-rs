extern crate image;
extern crate num_complex;
extern crate imageproc;

extern crate gtk;
extern crate gio;
extern crate gdk_pixbuf;

use gtk::prelude::*;
use gio::prelude::*;

mod grid;
mod polar;
mod rectangle;
mod generate;
mod solve;

use grid::{AbstractGrid, AbstractCell, CompassDirections};
use polar::CircularGrid;
use rectangle::RectangleGrid;
use generate::*;


use gtk::{Application, ApplicationWindow, DrawingArea};
use cairo::Context;
use crate::solve::DijkstraStep;
use std::f64::consts::PI;

#[allow(dead_code)]
fn draw_maze(w: &DrawingArea, cr: &Context, g: &RectangleGrid, cellsize: f64) {
    let scalex = w.get_allocated_width() as f64 / (g.width as f64 * cellsize);
    let scaley = w.get_allocated_height() as f64 / (g.height as f64 * cellsize);


    cr.scale(scalex, scaley);
    cr.set_line_width(1.0);
    for ix in 0..g.len() {
        let cur_cell = g.cell(ix);
        let draw_line =
            |item: &Option<usize>, end: (f64, f64)| {
                match item {
                    Some(r_idx) if !cur_cell.links().contains(r_idx) =>
                        cr.line_to(end.0, end.1),
                    _ => cr.move_to(end.0, end.1)
                }
            };
        let pixcoord = |ix: usize| -> f64 {
            ix as f64 * cellsize
        };
        let x1 = pixcoord(cur_cell.col());
        let x2 = pixcoord(cur_cell.col() + 1);
        let y1 = pixcoord(cur_cell.row());
        let y2 = pixcoord(cur_cell.row() + 1);
        cr.move_to(x1, y1);
        draw_line(&g.west_ix(ix), (x1, y2));
        draw_line(&g.south_ix(ix), (x2, y2));
        draw_line(&g.east_ix(ix), (x2, y1));
        draw_line(&g.north_ix(ix), (x1, y1));
        cr.stroke();
    }
}

#[allow(dead_code)]
fn draw_pathfind(w: &DrawingArea, cr: &Context, g: &RectangleGrid,
                 step_state: &DijkstraStep, cellsize: f64) {
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
        let row = g.cell(ix as usize).row();
        let col = g.cell(ix as usize).col();
        (pixcoord(col), pixcoord(row))
    };

    let line = |i1: i32, i2: i32| {
        let (x1, y1) = coords(i1);
        let (x2, y2) = coords(i2);

        cr.move_to(x1, y1);
        cr.line_to(x2, y2);
        cr.stroke();
    };

    let rect = |i: usize| {
        let row = g.cell(i).row() as f64;
        let col = g.cell(i).col() as f64;
        let (x1, y1) = (col * cellsize, row * cellsize);
        cr.rectangle(x1, y1, cellsize, cellsize);
        cr.fill();
    };

    let mut max_idx = 0;
    let mut min_idx = 0;
    let mut max_length = step_state.cell_weights[max_idx].path_length;
    let mut min_length = max_length;
    for (i, c) in step_state.cell_weights.iter().enumerate() {
        if c.path_length > max_length {
            max_length = c.path_length;
            max_idx = i;
        }
        if c.path_length < min_length {
            min_length = c.path_length;
            min_idx = i;
        }
    }


    let cur_cell = g.cell(min_idx);
    let end_cell = g.cell(max_idx);


    cr.set_line_width(6.0);
    for (i, c) in step_state.cell_weights.iter().enumerate() {
        let intensity= (max_length - c.path_length) as f64 / max_length as f64;
        let dark = intensity;
        let bright = 0.5 + intensity / 2.;
        cr.set_source_rgb(dark, bright, dark);
        rect(i);
    }


    let x1 = pixcoord(cur_cell.col());
    let x2 = pixcoord(end_cell.col());

    let y1 = pixcoord(cur_cell.row());
    let y2 = pixcoord(end_cell.row());
    cr.set_line_width(1.0);
    cr.set_source_rgb(0.,0.,0.);
    circle(x1,y1);
    cr.stroke();
    circle(x2,y2);
    cr.stroke();
    if step_state.cell_weights[max_idx].parent >= 0 {
        let mut cur_cell = max_idx as i32;
        cr.set_source_rgb(1., 0., 0.);
        cr.set_line_width(4.0);
        while cur_cell != (min_idx as i32) {
            line(cur_cell, step_state.cell_weights[cur_cell as usize].parent);
            cur_cell = step_state.cell_weights[cur_cell as usize].parent;
        }
    }
}

#[allow(dead_code)]
fn solve_with_longest_path<T: AbstractCell>(g: &AbstractGrid<T>) -> DijkstraStep {
    let start = 0;
    // solve initially from random point
    let mut result = DijkstraStep::initial(g, start);
    while !result.lookup_queue.is_empty() {
        result = result.next_step(g);
    }

    let mut max_length = 0;
    let mut max_idx = 0;
    for (i, c) in result.cell_weights.iter().enumerate() {
        if c.path_length > max_length {
            max_length = c.path_length;
            max_idx = i;
        }
    }

    if max_idx != 0 {
        result = DijkstraStep::initial(g, start);
        while !result.lookup_queue.is_empty() {
            result = result.next_step(g);
        }
    }
    result
}

#[allow(dead_code)]
fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.set_default_size(400, 400);

    let img = gtk::DrawingArea::new();
    vbox.add(&img);
    let mut g = RectangleGrid::new(25, 25);
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


    let step_state= solve_with_longest_path(&g);

    img.connect_draw(move |w, cr| {
        draw_pathfind(w, cr, &g, &step_state, cellsize);
        gtk::Inhibit(false)
    });

    img.connect_draw(move |w, cr| {
        draw_maze(w, cr, &g_copy, cellsize);
        gtk::Inhibit(false)
    });

    window.add(&vbox);
    window.show_all();
}

fn draw_polar_maze(w: &DrawingArea, cr: &Context, g_polar: &CircularGrid, actual_ring_height: usize) {
    let scalex = w.get_allocated_width() as f64 / (g_polar.height * actual_ring_height * 2) as f64;
    let scaley = w.get_allocated_height() as f64 / (g_polar.height * actual_ring_height * 2) as f64;
    cr.scale(scalex, scaley);
    cr.set_line_width(1.0);

    let center_x = g_polar.height as f64 * actual_ring_height as f64;
    let center_y = center_x;
    let ring_height = actual_ring_height as f64;
    
    cr.arc(center_x, center_y, ring_height * g_polar.height as f64, 0., 2.*PI);
    cr.stroke();
    for i in 0..g_polar.len(){
        if i == 0 {
            continue;
        }
        let cell = g_polar.cell(i);
        let inward = g_polar.inward_ix(i).unwrap();
        let theta = 2.* PI/(cell.columns as f64);
        let inner_r = ring_height * cell.row() as f64;
        let outer_r = ring_height * (cell.row() + 1) as f64;
        let theta_cw = theta * (cell.col() as f64);
        let theta_ccw = theta * ((cell.col() + 1) as f64);
        if !cell.links().contains(&inward) {
            cr.arc(center_x, center_y, inner_r, theta_cw, theta_ccw);
            cr.stroke();
        }

        let east = g_polar.cw_ix(i);
        
        if !cell.links.contains(&east) { 
            let cx = center_x + inner_r * theta_ccw.cos();
            let dx = center_x + outer_r * theta_ccw.cos();
            let cy = center_x + inner_r * theta_ccw.sin();
            let dy = center_x + outer_r * theta_ccw.sin();
            cr.move_to(cx, cy);
            cr.line_to(dx, dy);
            cr.stroke();
        }
    }
}

fn draw_polar_pathfind(w: &DrawingArea, cr: &Context, g: &CircularGrid, step_state: &DijkstraStep, cellsize: usize) {
    let scalex = w.get_allocated_width() as f64 / (g.height * cellsize * 2) as f64;
    let scaley = w.get_allocated_height() as f64 / (g.height * cellsize * 2) as f64;
    cr.scale(scalex, scaley);
    cr.set_line_width(cellsize as f64 + 1.); // 1. to not create gaps between rows
    let center_x = g.height as f64 * cellsize as f64;
    let center_y = center_x;

    let mut max_idx = 0;
    let mut min_idx = 0;
    let mut max_length = step_state.cell_weights[max_idx].path_length;
    let mut min_length = max_length;
    for (i, c) in step_state.cell_weights.iter().enumerate() {
        if c.path_length > max_length {
            max_length = c.path_length;
            max_idx = i;
        }
        if c.path_length < min_length {
            min_length = c.path_length;
            min_idx = i;
        }
    }



    // returns inner radius, theta1, theta2
    let pixcoord = |ix: usize| -> (f64, f64, f64) {
        let row = g.cell(ix).row() as f64;
        let col = g.cell(ix).col() as f64;
        let total_cols = g.cell(ix).columns as f64;
        let theta = 2.* PI/total_cols;
        let inner_r = (cellsize as f64) * (row + 0.5);

        (inner_r, theta * col, theta * (col + 1.01)) // 1.01 to not create gaps between clockwise neighbours
    };

    for (i, c) in step_state.cell_weights.iter().enumerate() {
        let intensity= (max_length - c.path_length) as f64 / max_length as f64;
        let dark = intensity;
        let bright = 0.5 + intensity / 2.;
        cr.set_source_rgb(dark, bright, dark);
        // cr.set_source_rgb(1., 1., 0.);
        let (r, theta1, theta2) = pixcoord(i);
        cr.arc(center_x, center_y, r, theta1, theta2);
        cr.stroke();
    }

    let connect = |ix1: usize, ix2: usize| {
        let r1 = g.cell(ix1).row();
        let r2 = g.cell(ix2).row();
        let theta = 2.* PI/(g.cell(ix1).columns as f64);
        
        if r1 == r2 {
            let col1 = g.cell(ix1).col;
            let col2 = g.cell(ix2).col;
            let total = g.cell(ix1).columns;

            let a1 = theta * (0.5 + col1 as f64);
            let a2 = theta * (0.5 + col2 as f64);
            // when last and first columns are connected, should draw counter-clockwise instead of clockwise
            let (a_from, a_to) = if usize::min(col1, col2) == 0 && usize::max(col1, col2) == total - 1 {
                (f64::max(a2, a1), f64::min(a2, a1))
            } else  {
                (f64::min(a2, a1), f64::max(a2, a1))
            };
            
            cr.arc(center_x, center_y, (r1 as f64 + 0.5) * (cellsize as f64), a_from, a_to);
            cr.stroke();
        } else {
            let start_r = (0.5 + r1 as f64) * (cellsize as f64);
            let end_r = (0.5 + r2 as f64) * (cellsize as f64);
            let theta2 = 2.* PI/(g.cell(ix2).columns as f64);
            let a = theta * (0.5 + g.cell(ix1).col as f64);
            let a2 = theta2 * (0.5 + g.cell(ix2).col as f64);
            let cx = center_x + start_r * a.cos();
            let dx = center_x + end_r * a2.cos();
            let cy = center_x + start_r * a.sin();
            let dy = center_x + end_r * a2.sin();
            cr.move_to(cx,cy);
            cr.line_to(dx, dy);
            cr.stroke();
        }
    };

    if step_state.cell_weights[max_idx].parent >= 0 {
        let mut cur_cell = max_idx as i32;
        cr.set_source_rgb(1., 0., 0.);
        cr.set_line_width(4.0);
        while cur_cell != (min_idx as i32) {
            connect(cur_cell as usize, step_state.cell_weights[cur_cell as usize].parent as usize);
            cur_cell = step_state.cell_weights[cur_cell as usize].parent;
        }
    }
}

#[allow(dead_code)]
fn build_polar_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.set_default_size(400, 400);

    let img = gtk::DrawingArea::new();
    vbox.add(&img);

    img.set_vexpand(true);
    img.set_hexpand(true);

    let mut rng = rand::thread_rng();
    let actual_ring_height = 20;
    let mut g_polar = CircularGrid::new(10);
    recursive_backtracker(&mut g_polar, &mut rng);
    let step_state= solve_with_longest_path(&g_polar);

    let clone = g_polar.clone();
    img.connect_draw(move |w, cr| {
        draw_polar_pathfind(w,cr,&clone,&step_state, actual_ring_height);
        gtk::Inhibit(false)
    });

    img.connect_draw(move |w, cr| {
        draw_polar_maze(w,cr,&g_polar,actual_ring_height);
        gtk::Inhibit(false)
    });

    window.add(&vbox);
    window.show_all();
}

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