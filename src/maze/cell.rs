use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::cmp::{PartialEq, Eq};


pub type CellWeakLink = Weak<RefCell<Cell>>;
pub type CellStrongLink = Rc<RefCell<Cell>>;


// #[derive(PartialEq, Eq)]
pub struct Cell {
    pub row: u16,
    pub col: u16,

    links: Vec<CellWeakLink>,

    pub top: Option<CellWeakLink>,
    pub bottom: Option<CellWeakLink>,
    pub left: Option<CellWeakLink>,
    pub right: Option<CellWeakLink>,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}


impl Cell {
    pub fn new(row: u16, col: u16) -> Self {
        Self {
            row,
            col,

            links: vec![],

            top: None,
            bottom: None,
            left: None,
            right: None,
        }
    }


    pub fn link_to(&mut self, cell: CellWeakLink) { //, _bidirect: Option<bool>) {
        self.links.push(Rc::downgrade(&cell.upgrade().unwrap()));
        // if _bidirect.unwrap_or(true) {
        //     let mut cell = (&*cell.upgrade().unwrap()).borrow_mut();
        //     cell.link_to(Rc::downgrade(&Rc::new(RefCell::new(&*self))), Some(false));
        // }
    }


    pub fn has_linked_cells(&self) -> bool {
        !self.links.is_empty()
    }


    pub fn is_linked_to(&self, cell: CellWeakLink) -> bool {
        // self.links.contains(&cell) // PartialEq not implemented on Weak<RefCell<Cell>>

        for link in &self.links {
            if *(&*link.upgrade().unwrap()).borrow() == *(&*cell.upgrade().unwrap()).borrow() {
                return true;
            }
        }

        false
    }


    pub fn neighbours(&self) -> Vec<CellWeakLink> {
        let mut lst: Vec<CellWeakLink> = vec![];

        if let Some(cell) = &self.top {
            lst.push(Rc::downgrade(&cell.upgrade().unwrap()));
        }
        if let Some(cell) = &self.bottom {
            lst.push(Rc::downgrade(&cell.upgrade().unwrap()));
        }
        if let Some(cell) = &self.left {
            lst.push(Rc::downgrade(&cell.upgrade().unwrap()));
        }
        if let Some(cell) = &self.right {
            lst.push(Rc::downgrade(&cell.upgrade().unwrap()));
        }

        lst

    }

}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn cell_link_to() {
        let cell1 = Rc::new(RefCell::new(Cell::new(2, 1)));
        let cell2 = Rc::new(RefCell::new(Cell::new(2, 2)));

        (&*cell1).borrow_mut().link_to(Rc::downgrade(&cell2)); //, Some(true));
        (&*cell2).borrow_mut().link_to(Rc::downgrade(&cell1)); //, Some(true));

        // println!("{}", (&*cell1).borrow().row);
        // println!("{}", RefCell::borrow(&cell1).row);

        assert_eq!(true, (&*cell1).borrow().is_linked_to(Rc::downgrade(&cell2)));
    }


    #[test]
    fn cell_is_not_linked_to() {
        let cell1 = Rc::new(RefCell::new(Cell::new(2, 1)));
        let cell2 = Rc::new(RefCell::new(Cell::new(2, 2)));
        let cell3 = Rc::new(RefCell::new(Cell::new(3, 1)));

        (&*cell1).borrow_mut().link_to(Rc::downgrade(&cell2)); //, Some(true));
        (&*cell2).borrow_mut().link_to(Rc::downgrade(&cell1)); //, Some(true));

        assert_eq!(false, (&*cell1).borrow().is_linked_to(Rc::downgrade(&cell3)));
    }


    #[test]
    fn cell_right_left() {
        let cell1 = Rc::new(RefCell::new(Cell::new(2, 1)));
        let cell2 = Rc::new(RefCell::new(Cell::new(2, 2)));

        (&*cell1).borrow_mut().right = Some(Rc::downgrade(&cell2));
        (&*cell2).borrow_mut().left = Some(Rc::downgrade(&cell1));

        let cell = (&*cell2).borrow();
        assert_eq!(true, cell.left.is_some());
    }


    #[test]
    fn cell_neighbours() {
        let cell1 = Rc::new(RefCell::new(Cell::new(2, 1)));
        let cell2 = Rc::new(RefCell::new(Cell::new(2, 2)));
        let cell3 = Rc::new(RefCell::new(Cell::new(3, 1)));

        (&*cell1).borrow_mut().right = Some(Rc::downgrade(&cell2));
        (&*cell1).borrow_mut().top = Some(Rc::downgrade(&cell3));
        (&*cell2).borrow_mut().left = Some(Rc::downgrade(&cell1));

        let neighbours = (&*cell1).borrow().neighbours();

        assert_eq!(true, !neighbours.is_empty());
    }
}
