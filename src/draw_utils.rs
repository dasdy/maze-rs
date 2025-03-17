use crate::grid::{AbstractCell, AbstractGrid};
use crate::gtk::prelude::WidgetExt;
use crate::solve::DijkstraStep;
use crate::{delta, generate, hexagonal, polar, rectangle, solve};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait GtkDrawable {
    fn draw_pathfind(
        &self,
        w: &gtk::DrawingArea,
        cr: &gtk::cairo::Context,
        step_state: &DijkstraStep,
        cellsize: f64,
    );
    fn draw_maze(&self, w: &gtk::DrawingArea, cr: &gtk::cairo::Context, cellsize: f64);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MazeType {
    Regular,
    Circular,
    Hexagonal,
    Delta,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GenerationType {
    RecursiveBacktracker,
    AldousBroder,
    SimplifiedPrim,
    TruePrim,
}

pub struct Settings {
    pub maze_type: MazeType,
    pub generation_type: GenerationType,
    pub braid_chance: u8,
    pub version: u128,
    pub step: DijkstraStep,
    pub drawn: bool,
    pub grid: Box<dyn GtkDrawable>,
}

unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

pub fn make_tha_maze<C: AbstractCell, T: AbstractGrid<C>>(
    generation_type: &GenerationType,
    grid: &mut T,
    braid_chance: u8,
) {
    let mut rng = rand::thread_rng();
    match generation_type {
        GenerationType::RecursiveBacktracker => generate::recursive_backtracker(grid, &mut rng),
        GenerationType::AldousBroder => generate::aldous_broder(grid, &mut rng),
        GenerationType::SimplifiedPrim => generate::simplified_prim(grid, &mut rng),
        GenerationType::TruePrim => generate::true_prim(grid, &mut rng),
    }

    if braid_chance > 0 {
        generate::braid(grid, &mut rng, braid_chance)
    }
}
impl Settings {
    pub fn make_maze(&mut self) {
        self.grid = match self.maze_type {
            MazeType::Regular => {
                let mut g1 = Box::new(rectangle::RegularGrid::new(70, 70));

                make_tha_maze(&self.generation_type, &mut *g1, self.braid_chance);
                let step_state = solve::solve_with_longest_path(&*g1);
                self.step = step_state;
                g1
            }
            MazeType::Circular => {
                let mut g1 = Box::new(polar::CircularGrid::new(40));
                make_tha_maze(&self.generation_type, &mut *g1, self.braid_chance);
                let step_state = solve::solve_with_longest_path(&*g1);
                self.step = step_state;
                g1
            }
            MazeType::Hexagonal => {
                let mut g1 = Box::new(hexagonal::HexagonalGrid::new(50, 50));

                make_tha_maze(&self.generation_type, &mut *g1, self.braid_chance);
                let step_state = solve::solve_with_longest_path(&*g1);
                self.step = step_state;
                g1
            }
            MazeType::Delta => {
                let mut g1 = Box::new(delta::DeltaGrid::new(45, 70));
                make_tha_maze(&self.generation_type, &mut *g1, self.braid_chance);
                let step_state = solve::solve_with_longest_path(&*g1);
                self.step = step_state;
                g1
            }
        };
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        self.version = since_the_epoch.as_millis();
        self.drawn = false;
    }
}

pub fn draw_grid_mutex(
    img: &gtk::DrawingArea,
    g: Arc<RwLock<Settings>>,
    generation_type: GenerationType,
    maze_type: MazeType,
    braid_chance: u8,
    version: u128,
) {
    let cellsize = 10.;
    img.connect_draw(move |w, cr| {
        {
            let mut data = g.write().unwrap();
            if data.maze_type == maze_type
                && data.generation_type == generation_type
                && data.braid_chance == braid_chance
                && data.version == version
            // TODO: This is intended to fix re-drawing without need. For some reason does not work
            // really
            // && !data.drawn
            {
                let settings = &*data;
                // println!("- draw pathfind - ");
                settings.grid.draw_pathfind(w, cr, &settings.step, cellsize);
                // println!("- draw maze - ");
                settings.grid.draw_maze(w, cr, cellsize);
                // println!("- draw: moving on - ");
                data.drawn = true
            } else {
                println!(
                    "No redraw yay: {:?}, {:?}, {:}, {:}, {:}",
                    data.maze_type,
                    data.generation_type,
                    data.braid_chance,
                    data.version,
                    data.drawn
                )
            }
        }
        cairo::glib::Propagation::Proceed
    });
}
