use std::panic::panic_any;

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


pub struct Maze {
    grid: Option<maze::grid::Grid>,

    pub width: u16,
    pub height: u16,

    img: Option<presenter::to_img::Image>,
}


impl Maze {
    pub fn new() -> Self {
        Self {
            grid: None,

            width: 0,
            height: 0,

            img: None,
        }
    }


    /// Returns the maze in its raw format - an object representing the maze internally
    /// in the crate.
    pub fn get_maze(&self) -> &maze::grid::Grid {
        self.grid.as_ref().unwrap()
    }


    /// Initialises the maze geometry.
    pub fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;

        self.grid = Some(maze::grid::Grid::new(width, height));
        self.grid.as_mut().unwrap().init_grid();
    }


    // Resets the maze, if maze is already initialised, otherwise initialises it.
    fn reinit(&mut self) {
        if self.grid.is_none() {
            self.init(20, 20);
        } else {
            self.init(self.width, self.height);
        }
    }


    /// Generates a random maze using the default algorithm.
    /// The default algorithm is Algo::RecursiveBacktracking.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_mazegen::*;
    ///
    /// let mut maze = Maze::new();
    /// maze.build();
    ///
    /// assert_eq!(20u16, maze.height);
    /// ```
    pub fn build(&mut self) {
        self.reinit();
        self.build_with(Algo::RecursiveBacktracking);
    }


    /// Generates a random maze using one of the selected algorithms.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_mazegen::*;
    ///
    /// let mut maze = Maze::new();
    /// maze.build_with(Algo::RecursiveBacktracking);
    ///
    /// assert_eq!(20u16, maze.height);
    /// ```
    pub fn build_with(&mut self, algo: Algo) {
        self.reinit();
        let grid = self.grid.as_ref().unwrap();

        match algo {
            Algo::BinaryTree => panic!("Not yet implemented"),
            Algo::Sidewinder => algos::sidewinder::build_maze(grid),
            Algo::RecursiveBacktracking => algos::recursive_backtracking::build_maze(grid),
            Algo::Prim => panic!("Not yet implemented"),
            Algo::Kruskal => panic!("Not yet implemented"),
            Algo::Eller => panic!("Not yet implemented"),
            Algo::HuntAndKill => panic!("Not yet implemented"),
            Algo::AldousBroder => panic!("Not yet implemented"),
            Algo::Wilson => panic!("Not yet implemented"),
            Algo::RecursiveDivision => panic!("Not yet implemented"),
            Algo::GrowingTree => panic!("Not yet implemented"),
        };
    }


    /// Configures the graphical representation of the generated maze.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_mazegen::*;
    ///
    /// let mut maze = Maze::new();
    /// maze.build();
    /// maze.img_config(20, 20);
    /// maze.save_as_img("test_img_config");
    ///
    /// assert_eq!(20u16, maze.height);
    /// ```
    pub fn img_config(&mut self, cell_size: u16, wall_thickness: u16) {
        self.img = Some(presenter::to_img::Image::new(cell_size, wall_thickness));
    }


    /// Saves the generated image of the maze.
    ///
    /// **Note**
    /// - the filename shall be specified without the file format, which by default is set to PNG.
    /// - to change the graphical representation of the maze, use method `img_config()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_mazegen::*;
    ///
    /// let mut maze = Maze::new();
    /// maze.build();
    /// maze.save_as_img("test_save_as_img");
    ///
    /// assert_eq!(20u16, maze.height);
    /// ```
    pub fn save_as_img(&mut self, filename: &str) {
        let fileformat = ".png";
        let file = format!("{}{}", filename, fileformat);

        if self.img.is_none() {
            self.img_config(20, 2);
        }
        self.img.as_ref().unwrap().save(self.grid.as_ref().unwrap(), file.as_str());
    }
}

