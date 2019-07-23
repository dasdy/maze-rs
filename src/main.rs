extern crate image;
extern crate num_complex;
extern crate imageproc;

extern crate gtk;
extern crate gio;
extern crate gdk_pixbuf;

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

fn draw_pathfind(w: &DrawingArea, cr: &Context, g: &Grid,
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

    let rect = |i: usize| {
        let row = g.cells[i].row as f64;
        let col = g.cells[i].col as f64;
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


    let cur_cell = &g.cells[min_idx];
    let end_cell = &g.cells[max_idx];


    cr.set_line_width(6.0);
    for (i, c) in step_state.cell_weights.iter().enumerate() {
        let intensity= (max_length - c.path_length) as f64 / max_length as f64;
        let dark = intensity;
        let bright = 0.5 + intensity / 2.;
        cr.set_source_rgb(dark, bright, dark);
        rect(i);
    }


    let x1 = pixcoord(cur_cell.col);
    let x2 = pixcoord(end_cell.col);

    let y1 = pixcoord(cur_cell.row);
    let y2 = pixcoord(end_cell.row);
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

fn solve_with_longest_path(g: &Grid) -> DijkstraStep {
    let start = 0;
    // solve initially from random point
    let mut result = DijkstraStep::initial(&g, start);
    while !result.lookup_queue.is_empty() {
        result = result.next_step(&g);
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
        result = DijkstraStep::initial(&g, start);
        while !result.lookup_queue.is_empty() {
            result = result.next_step(&g);
        }
    }
    result
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

fn build_polar_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.set_default_size(400, 400);

    let img = gtk::DrawingArea::new();
    vbox.add(&img);

//    sidewinder(&mut g, &mut rng);
//    binary_tree(&mut g, &mut rng);
//    aldous_broder(&mut g, &mut rng);
//    hunt_and_kill(&mut g, &mut rng);
    

    img.set_vexpand(true);
    img.set_hexpand(true);

    let mut g = Grid::new(10, 10);
    let mut rng = rand::thread_rng();
    recursive_backtracker(&mut g, &mut rng);
    println!("{}", g);
    let ring_height = 20;
    let g_polar = grid::CircularGrid::from_rect_grid(&g, ring_height);


    img.connect_draw(move |w, cr| {
        cr.set_line_width(1.0);
        let center_x = w.get_allocated_width() as f64 / 2.;
        let center_y = w.get_allocated_height() as f64 / 2.;
        
        cr.arc(center_x, center_y, (ring_height * g.height) as f64, 0., 2.*PI);
        for cell in g_polar.cells.iter() {
            // if cell.row > 1 {
            //     break;
            // }
            // print!("({}, {}) ->", cell.row, cell.col);
            // for (r, c) in cell.links.iter() {
            //     print!("({}, {}), ", r, c);
            // }
            
            // print!(", North: ({}), East: ({})", 
            //     match g_polar.north_ix(cell.row, cell.col) {
            //         Some(_ix) => format!("{}, {}", g_polar.cells[_ix].row, g_polar.cells[_ix].col),
            //         _ => format!("None")
            //     },
            //     match g_polar.east_ix(cell.row, cell.col) {
            //         Some(_ix) => format!("{}, {}", g_polar.cells[_ix].row, g_polar.cells[_ix].col),
            //         _ => format!("None")
            //     }
            //     );
            
            let north = g_polar.north_ix(cell.row, cell.col);
            
            match north {
                Some(ix)  => {
                    if !cell.links.contains(&(g_polar.cells[ix].row, g_polar.cells[ix].col)) {
                        cr.set_line_width(1.0);
                        // print!("north!");
                        cr.arc(center_x, center_y, cell.inner_r, cell.theta_cw, cell.theta_ccw);
                        cr.stroke();
                    } else {
                        cr.set_line_width(0.3);
                    }
                    
                }
                _ => {}
            }

            let east = g_polar.east_ix(cell.row, cell.col).unwrap();
            
            if !cell.links.contains(&(g_polar.cells[east].row, g_polar.cells[east].col)) { 
                cr.set_line_width(1.0);
                // print!(" east! ");
                let cx = center_x + cell.inner_r * cell.theta_ccw.cos();
                let dx = center_x + cell.outer_r * cell.theta_ccw.cos();
                let cy = center_x + cell.inner_r * cell.theta_ccw.sin();
                let dy = center_x + cell.outer_r * cell.theta_ccw.sin();
                cr.move_to(cx, cy);
                cr.line_to(dx, dy);
                cr.stroke();
            } else {
                cr.set_line_width(0.3);
            }
            
            println!();
        }
        gtk::Inhibit(false)
    });

    window.add(&vbox);
    window.show_all();
}

fn create_gtk_app() {
    let application = Application::new("com.github.gtk-rs.examples.basic",
                                       Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(build_polar_ui);

    application.run(&[]);
}

fn main() {
    create_gtk_app();
}