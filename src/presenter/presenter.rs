use std::rc::{Rc, Weak};
// use rand::distributions::uniform::SampleBorrow;
use crate::maze::{cell, grid};


pub fn print_to_console(grid: &grid::Grid) {
    let corner: &'static str = "+";
    let body: &'static str = "    ";
    let rbndr_pass: &'static str = " "; // passage to right adjacent cell
    let rbndr_wall: &'static str = "|"; // wall to right adjacent cell
    let bbndr_pass: &'static str = "    "; // passage to bottom adjacent cell
    let bbndr_wall: &'static str = "----"; // wall to bottom adjacent cell

    let output = "+----".repeat(grid.width as usize) + corner;
    println!("{}", output);

    for row in 0..grid.height as usize {
        let mut line_one = String::from("|");
        let mut line_two = String::from("+");

        for col in 0..grid.width as usize {
            let cell = Rc::clone(&grid.cells[row][col].as_ref().unwrap());
            let c = (&*cell).borrow();

            // let right = &*c.right.as_ref().unwrap();
            line_one += body;
            if let Some(right) = &c.right {
                if (&*cell).borrow().is_linked_to(Rc::downgrade(&right.upgrade().unwrap())) {
                    line_one += rbndr_pass;
                } else {
                    line_one += rbndr_wall;
                }
            } else {
                line_one += rbndr_wall;
            }

            // let bottom = &*c.bottom.as_ref().unwrap();
            if let Some(bottom) = &c.bottom {
                if (&*cell).borrow().is_linked_to(Rc::downgrade(&bottom.upgrade().unwrap())) {
                    line_two += bbndr_pass;
                } else {
                    line_two += bbndr_wall;
                }
            } else {
                line_two += bbndr_wall;
            }
            line_two += corner;

        }

        println!("{}", line_one);
        println!("{}", line_two);
    }
}


// needs to run with 'cargo test -- --nocapture'
#[cfg(test)]
mod tests {
    use super::*;


    // #[test]
    fn draw() {
        let mut grid = grid::Grid::new(5, 5);
        grid.init_grid();

        print_to_console(&grid);

        assert!(true);
    }
}