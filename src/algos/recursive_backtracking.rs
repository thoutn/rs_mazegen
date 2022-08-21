use std::cell::RefCell;
use std::rc::Rc;
use rand::{thread_rng, Rng, seq::SliceRandom};
use crate::maze::{cell, grid};


pub fn build_maze(grid: &grid::Grid) {
    let mut rng = thread_rng();

    let mut stack = vec![Rc::downgrade(&grid.get_rand_cell().upgrade().unwrap()); 1];

    while !stack.is_empty() {
        // picks the last cell from the stack
        let mut current_cell = Rc::clone(&(stack[stack.len() - 1]).upgrade().unwrap());

        // gets all neighbours of the 'current_cell', which are not yet linked to any adjacent cell
        let mut neighbours = vec![];
        let n = RefCell::borrow(&current_cell).neighbours();
        for cell in n.iter() {
            if !RefCell::borrow(cell.upgrade().as_ref().unwrap()).has_linked_cells() {
                neighbours.push(Rc::downgrade(&cell.upgrade().unwrap()));
            }
        }

        if !neighbours.is_empty() {
            // picks one random neighbour from 'neighbours'
            let neighbour = Rc::clone(
                &neighbours.choose(&mut rng).unwrap().upgrade().unwrap());

            // adds selected neighbour to the stack
            stack.push(Rc::downgrade(&neighbour));

            // links 'current_cell' to 'neighbour'
            {
                let mut c = (&*current_cell).borrow_mut();
                c.link_to(Rc::downgrade(&neighbour));
            }

            // calls the reverse 'link_to()' 'neighbour' -> 'current_cell'
            RefCell::borrow_mut(&neighbour).link_to(Rc::downgrade(&current_cell));
        } else {
            // if no unlinked neighbours remain, removes last cell from stack
            stack.pop();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::presenter::to_img::*;


    #[test]
    fn build() {
        let mut grid = grid::Grid::new(15, 15);
        grid.init_grid();

        build_maze(&grid);

        let img = Image::new(20, 2);
        img.save(&grid, "test_backtracker.png");

        assert!(true);
    }
}