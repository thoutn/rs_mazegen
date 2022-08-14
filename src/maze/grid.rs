use std::cell::RefCell;
use std::rc::{Rc, Weak};
use rand::{thread_rng, Rng};
use crate::maze::cell;


pub struct Grid {
    pub width: u16,
    pub height: u16,

    pub cells: Vec<Vec<Option<cell::CellStrongLink>>>,
}

impl Grid {
    pub fn new(width: u16, height:u16) -> Self {
        Self {
            width,
            height,

            cells: vec![],
        }
    }


    pub fn init_grid(&mut self) {
        self.prepare_grid();
        self.configure_cells();
    }


    fn prepare_grid(&mut self) {
        for row in 0..self.height as usize {
            self.cells.push(vec![]);
            for col in 0..self.width as usize{
                self.cells[row].push(Some(Rc::new(RefCell::new(
                    cell::Cell::new(row as u16, col as u16)))));
            }
        }
    }


    fn configure_cells(&mut self) {
        for row in &self.cells {
            for cell in row.iter() {
                let row_ = RefCell::borrow(cell.as_ref().unwrap()).row as i32;
                let col_ = RefCell::borrow(cell.as_ref().unwrap()).col as i32;

                RefCell::borrow_mut(cell.as_ref().unwrap()).top = self.create_neighbour(row_ - 1, col_);
                RefCell::borrow_mut(cell.as_ref().unwrap()).bottom = self.create_neighbour(row_ + 1, col_);
                RefCell::borrow_mut(cell.as_ref().unwrap()).left = self.create_neighbour(row_, col_ - 1);
                RefCell::borrow_mut(cell.as_ref().unwrap()).right = self.create_neighbour(row_, col_ + 1);

            }
        }
    }


    fn create_neighbour(&self, row: i32, col: i32) -> Option<cell::CellWeakLink> {
        if 0 <= row && row <= (self.width - 1) as i32 && 0 <= col && col <= (self.height - 1) as i32 {
            return Some(Rc::downgrade(&self.cells[row as usize][col as usize].as_ref().unwrap()));
        };
        None
    }


    pub fn get_rand_cell(&self) -> cell::CellWeakLink {
        let mut rng = thread_rng();
        let row_ = rng.gen_range(0..self.height) as usize;
        let col_ = rng.gen_range(0..self.width) as usize;

        Rc::downgrade(&self.cells[row_][col_].as_ref().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn grid_create() {
        let grid = Grid::new(5, 10);

        assert_eq!(5, grid.width);
    }


    #[test]
    fn grid_init() {
        let mut grid = Grid::new(5, 5);
        grid.init_grid();

        let cells = grid.cells;
        let cell = cells[3][2].as_ref().unwrap();
        let neighbours = RefCell::borrow(&cell).neighbours();

        assert_eq!(true, !neighbours.is_empty());
    }


    #[test]
    fn grid_top_border_none() {
        let mut grid = Grid::new(5, 5);
        grid.init_grid();

        let cells = grid.cells;
        let cell = cells[0][0].as_ref().unwrap();
        let border = &RefCell::borrow(&cell).top;

        assert_eq!(true, border.is_none());
    }


    #[test]
    fn grid_right_border_none() {
        let mut grid = Grid::new(5, 5);
        grid.init_grid();

        let cells = grid.cells;
        let cell = cells[0][4].as_ref().unwrap();
        let border = &RefCell::borrow(&cell).right;

        assert_eq!(true, border.is_none());
    }


    #[test]
    fn grid_inner_cells_some() {
        let mut grid = Grid::new(5, 5);
        grid.init_grid();

        let cells = grid.cells;
        let cell = cells[0][0].as_ref().unwrap();
        let border = &RefCell::borrow(&cell).bottom;

        assert_eq!(true, border.is_some());
    }


    #[test]
    fn grid_rand_cell() {
        let mut grid = Grid::new(5, 5);
        grid.init_grid();

        let rand_cell = &grid.get_rand_cell();
        let row = (&*rand_cell.upgrade().unwrap()).borrow().row;

        assert!(row < grid.height);
    }

}