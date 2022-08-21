//! # Overview
//!
//! This crate provides rust implementation of a maze generator. It is possible to generate
//! mazes using different graph algorithms. One can switch between these algorithms and observe
//! the differences among the results. Each algorithm provides a unique maze structure.
//!
//! The generated maze can be saved as an image, and used to solve it manually or using
//! an automated maze solver. The raw format of the maze is also available, and is intended
//! to be used by the later.
//!
//! # Public API
//!
//! ## The basics
//!
//! Build mazes using the default algorithm [`Algo::RecursiveBacktracking`] and of default size
//! of `20x20`:
//!
//! ```no_run
//! use rs_mazegen as m;
//!
//! m::build();
//! ```
//!
//! And save them using [`save_as_img`] method:
//!
//! ```ignore
//! m::save_as_img("maze_one");
//! ```
//!
//! This will save the maze to `maze_one.png` with the default file format of PNG.
//!
//! ## Customisation
//!
//! Change the maze size using [`init`] method (e.g. to `15x50`):
//!
//! ```ignore
//! m::init(15, 50);
//! m::build();
//! ```
//!
//! Change the algorithm using [`build_with`] instead of [`build`]:
//!
//! ```ignore
//! use rs_mazegen::Algo;
//!
//! m::build_with(Algo::Sidewinder);
//! ```
//!
//! Change the *cell size* and *wall thickness* of the maze when saving it into an image using
//! [`img_config`] as follows:
//!
//! ```ignore
//! m::img_config(20, 20);
//! m::save_as_img("img_config");
//! ```
//!
//! The above code will create an image of the maze with the same cell and wall size.


use std::panic::panic_any;
use std::cell::RefCell;
use std::rc::Rc;

mod maze;
mod presenter;
mod algos;


pub enum Algo {
    BinaryTree,
    Sidewinder,
    RecursiveBacktracking,
    Prim,
    Kruskal,
    Eller,
    HuntAndKill,
    AldousBroder,
    Wilson,
    RecursiveDivision,
    GrowingTree,
}


thread_local!(static GRID: Rc<RefCell<Option<maze::grid::Grid>>> = Rc::new(RefCell::new(None)));
thread_local!(static IMG: RefCell<Option<presenter::to_img::Image>> = RefCell::new(None));


/// Returns the maze in its raw format - an object representing the maze internally
/// in the crate.
pub fn get_maze() -> Rc<RefCell<Option<maze::grid::Grid>>> {
    GRID.with(|grid| {
        return Rc::clone(&grid);
    })
}


/// Initialises the maze geometry.
pub fn init(width: u16, height: u16) {

    GRID.with(|grid| {
        *RefCell::borrow_mut(grid) = Some(maze::grid::Grid::new(width, height));
        RefCell::borrow_mut(grid).as_mut().unwrap().init_grid();
    });

}


// Resets the maze, if maze is already initialised, otherwise initialises it.
fn reinit() {
    GRID.with(|grid| {
        if RefCell::borrow(grid).is_none() {
            init(20, 20);
        } else {
            let w = RefCell::borrow(grid).as_ref().unwrap().width;
            let h = RefCell::borrow(grid).as_ref().unwrap().height;
            init(w, h);
        }
    });

}


/// Generates a random maze using the default algorithm.
/// The default algorithm is [`Algo::RecursiveBacktracking`].
///
/// # Examples
///
/// ```
/// use rs_mazegen as maze;
///
/// maze::build();
///
/// assert_eq!(20u16, (&*maze::get_maze()).borrow().as_ref().unwrap().height);
/// ```
pub fn build() {
    reinit();
    build_with(Algo::RecursiveBacktracking);
}


/// Generates a random maze using one of the selected algorithms.
///
/// # Examples
///
/// ```
/// use rs_mazegen as maze;
/// use rs_mazegen::Algo;
///
/// maze::build_with(Algo::RecursiveBacktracking);
///
/// assert_eq!(20u16, (&*maze::get_maze()).borrow().as_ref().unwrap().height);
/// ```
pub fn build_with(algo: Algo) {
    reinit();

    GRID.with(|grid| {
        // let g = RefCell::borrow(grid).as_ref().unwrap();

        match algo {
            Algo::BinaryTree => panic!("Not yet implemented"),
            Algo::Sidewinder => algos::sidewinder::build_maze(
                RefCell::borrow(grid).as_ref().unwrap()),
            Algo::RecursiveBacktracking => algos::recursive_backtracking::build_maze(
                RefCell::borrow(grid).as_ref().unwrap()),
            Algo::Prim => panic!("Not yet implemented"),
            Algo::Kruskal => panic!("Not yet implemented"),
            Algo::Eller => panic!("Not yet implemented"),
            Algo::HuntAndKill => panic!("Not yet implemented"),
            Algo::AldousBroder => panic!("Not yet implemented"),
            Algo::Wilson => panic!("Not yet implemented"),
            Algo::RecursiveDivision => panic!("Not yet implemented"),
            Algo::GrowingTree => panic!("Not yet implemented"),
        };
    });

}


/// Configures the graphical representation of the generated maze.
///
/// # Examples
///
/// ```
/// use rs_mazegen as maze;
///
/// maze::build();
///
/// maze::img_config(20, 20);
/// maze::save_as_img("test_img_config");
///
/// assert_eq!(20u16, (&*maze::get_maze()).borrow().as_ref().unwrap().height);
/// ```
pub fn img_config(cell_size: u16, wall_thickness: u16) {
    IMG.with(|img| {
        *RefCell::borrow_mut(img) = Some(presenter::to_img::Image::new(
            cell_size,
            wall_thickness
        ));
    });
}


/// Saves the generated image of the maze.
///
/// **Note**
/// - the filename shall be specified without the file format, which by default is set to PNG.
/// - to change the graphical representation of the maze, use method [`img_config`].
///
/// # Examples
///
/// ```
/// use rs_mazegen as maze;
///
/// maze::build();
///
/// maze::save_as_img("test_save_as_img");
///
/// assert_eq!(20u16, (&*maze::get_maze()).borrow().as_ref().unwrap().height);
/// ```
pub fn save_as_img(filename: &str) {
    let fileformat = ".png";
    let file = format!("{}{}", filename, fileformat);

    IMG.with(|img| {
        if RefCell::borrow(img).is_none() {
            img_config(20, 2);
        }

        GRID.with(|grid| {
            // let g = RefCell::borrow(grid).as_ref().unwrap();
            RefCell::borrow(img).as_ref().unwrap().save(
                RefCell::borrow(grid).as_ref().unwrap(), file.as_str());
        });
    });

}
