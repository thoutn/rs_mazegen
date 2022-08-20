use std::cell::RefCell;
use std::rc::Rc;
use rand::{thread_rng, Rng, seq::SliceRandom};
use crate::maze::{cell, grid};


pub fn build_maze(grid: &grid::Grid) {
    let mut rng = thread_rng();
    for row in &grid.cells {
        let mut run = vec![];

        for cell in row.iter() {
            let cell = Rc::clone(&cell.as_ref().unwrap());
            run.push(Rc::clone(&cell));

            let is_pace_to_close_run = {(&*cell).borrow().right.is_none() ||
                (&*cell).borrow().top.is_some() && rng.gen::<bool>()};

            if is_pace_to_close_run {
                let cell_ = Rc::clone(run.choose(&mut rng).as_ref().unwrap());

                if (&*cell_).borrow().top.is_some() {
                    // links cell to cell.top
                    {
                        let mut c = (&*cell_).borrow_mut();
                        let n = &c.top.as_ref().unwrap().upgrade().unwrap();
                        c.link_to(Rc::downgrade(&n));
                        // (&*cell_).borrow_mut().link_to(Rc::downgrade(
                        //     &(&*cell_).borrow().top.as_ref().unwrap().upgrade().unwrap()
                        // ));
                    }

                    // calls the reverse 'link_to()' cell.top -> cell
                    RefCell::borrow_mut(
                        &(&*cell_).borrow().top.as_ref().unwrap().upgrade().unwrap()
                    ).link_to(Rc::downgrade(&cell_));
                }

                run.clear();
            } else {
                // links cell to cell.right
                {
                    let mut c = (&*cell).borrow_mut();
                    let n = &c.right.as_ref().unwrap().upgrade().unwrap();
                    c.link_to(Rc::downgrade(&n));
                    // (&*cell).borrow_mut().link_to(Rc::downgrade(
                    //         &(&*cell).borrow().right.as_ref().unwrap().upgrade().unwrap()
                    //     ));
                }

                // calls the reverse 'link_to()' cell.right -> cell
                RefCell::borrow_mut(
                    &(&*cell).borrow().right.as_ref().unwrap().upgrade().unwrap()
                ).link_to(Rc::downgrade(&cell));
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::presenter::presenter::print_to_console;


    #[test]
    fn build() {
        let mut grid = grid::Grid::new(5, 5);
        grid.init_grid();

        build_maze(&grid);

        print_to_console(&grid);

        assert!(true);
    }
}